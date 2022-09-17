use std::ops;

use paste::paste;

/// The `Float` type lets you easily switch between single-precision and double-precision float types.
/// 
/// If someone implements a higher-precision type, they could easily replace this type declaration with
/// their own type.
pub type Float = f64;

// Adapted from
// https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#repetition-with-replacement
macro_rules! replace_type {
	($_i:tt $sub:ty) => {$sub};
}

// Stolen from
// https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
macro_rules! count_tts {
	() => { 0 };
	($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
	($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}

macro_rules! vector_def {
	($pub:vis struct $vec_n:ident { $($component:ident),+ }) => {
		#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default)]
		#[repr(C)]
		$pub struct $vec_n {
			$(pub $component: Float,)+
		}
		
		// Operators
		
		impl ops::Neg for $vec_n {
			type Output = Self;
			fn neg(self) -> Self::Output {
				Self { $($component: -self.$component,)+ }
			}
		}
		
		impl ops::Add for $vec_n {
			type Output = Self;
			fn add(self, other: Self) -> Self::Output {
				Self { $($component: self.$component + other.$component,)+ }
			}
		}
		
		impl ops::Sub for $vec_n {
			type Output = Self;
			fn sub(self, other: Self) -> Self::Output {
				Self { $($component: self.$component - other.$component,)+ }
			}
		}
		
		impl ops::Mul<Float> for $vec_n {
			type Output = Self;
			fn mul(self, other: Float) -> Self::Output {
				Self { $($component: self.$component * other,)+ }
			}
		}
		impl ops::Mul<$vec_n> for Float {
			type Output = $vec_n;
			fn mul(self, other: $vec_n) -> Self::Output { other * self }
		}
		impl ops::Mul for $vec_n {
			type Output = Self;
			/// This will cause a bit of confusion.
			/// This is a naive "multiply each component with its counterpart" thing.
			fn mul(self, other: Self) -> Self::Output {
				Self { $($component: self.$component * other.$component,)+ }
			}
		}
		
		impl ops::Div<Float> for $vec_n {
			type Output = Self;
			fn div(self, other: Float) -> Self::Output {
				Self { $($component: self.$component / other,)+ }
			}
		}
		
		// Assign Operators
		
		impl ops::AddAssign for $vec_n {
			fn add_assign(&mut self, other: Self) {
				*self = Self { $($component: self.$component + other.$component,)+ }
			}
		}
		impl ops::SubAssign for $vec_n {
			fn sub_assign(&mut self, other: Self) {
				*self = Self { $($component: self.$component - other.$component,)+ }
			}
		}
		impl ops::MulAssign<Float> for $vec_n {
			fn mul_assign(&mut self, other: Float) {
				*self = Self { $($component: self.$component * other,)+ }
			}
		}
		impl ops::DivAssign<Float> for $vec_n {
			fn div_assign(&mut self, other: Float) {
				*self = Self { $($component: self.$component / other,)+ }
			}
		}
		
		// Main Implementation
		
		impl $vec_n {
			/// A Vector with all components set to 0.
			pub const ZERO: Self = Self::all(0.0);
			
			/// A Vector with all components set to 1.
			pub const ONE: Self = Self::all(1.0);
			
			/// Create a new Vector, listing each of its components in order.
			pub const fn new( $($component: Float,)+ ) -> Self {
				$vec_n { $($component,)+ }
			}
			
			/// Create a new Vector, with all components set to `val`.
			pub const fn all(val: Float) -> Self {
				$vec_n { $($component: val,)+ }
			}
			
			paste! { $(
				pub const fn [<set_ $component>](mut self, val: Float) -> Self {
					self.$component = val; self
				}
				
				pub const [<$component:upper>]: Self = Self::ZERO.[<set_ $component>](1.0);
			)+ }
			
			paste! { pub const AXES: [Self; count_tts!($($component)+)] = [
				$(Self::[< $component:upper>],)+
			]; }
			
			// quiz: which one's shorter:
			// let x = Vec3::Y * 2.0;
			// let x = Vec3::ZERO.set_y(y);
			// let x = Vec3::new(0.0, y, 0.0);
			// let x = Vec3::set_y(Vec3::ZERO, y);
			
			/// Get the dot product of `self` dot `other`.
			// TODO: this'll need to be separated out into a float-specific
			//       version of Vectors once I want to add types
			pub fn dot(self, other: Self) -> Float {
				let r: Float = 0.0;
				$(
				let r = Float::mul_add(self.$component, other.$component, r);
				)+
				r
			}
			
			// no cross product, too scary, implement for each concrete thing.
			
			/// Get the squared length of a vector. Cheaper to compute than `magnitude`.
			/// Useful for when you only need to know if one length is greater than the other, etc.
			pub fn squared_magnitude(self) -> Float {
				let r: Float = 0.0;
				$(
				let r = r + self.$component.powi(2);
				)+
				r
			}
			
			/// Get the length of a vector.
			pub fn magnitude(self) -> Float {
				self.squared_magnitude().sqrt()
			}
			
			/// Normalize a vector, making it a unit vector.
			pub fn normalize(self) -> Self {
				self / self.magnitude()
			}
			
			/// Project the vector `self` onto the `other` vector.
			pub fn project(self, other: Self) -> Self {
				(other.dot(self) / other.magnitude()) * other
			}
			
			/// Reflect the vector `self` from the `other` normal vector.
			pub fn reflect(self, other: Self) -> Self {
				self - 2.0 * self.dot(other) * other
			}
			
			/// Linear-interpolate between the vectors `self` and `other` by
			/// a factor of `t`.
			pub fn lerp(self, other: Self, t: Float) -> Self {
				self * (1.0 - t) + other * t
			}
		}
		
		// Conversion to/from Tuples
		// `assert_eq!( (1, 2, 3).into(), Vec3::new(1, 2, 3) );`
		paste! {
			type [<$vec_n AsTuple>] = ( $(replace_type!($component Float),)+ );
		
			impl From<$vec_n> for [<$vec_n AsTuple>] {
				fn from(v: $vec_n) -> Self { ( $(v.$component,)+ ) }
			}
			impl From<[<$vec_n AsTuple>]> for $vec_n {
				fn from(( $($component,)+ ): [<$vec_n AsTuple>]) -> Self {
					$vec_n { $($component,)+ }
				}
			}
		}
		
		// Conversion to/from Arrays
		// `assert_eq!( [1, 2, 3].into(), Vec3::new(1, 2, 3) );`
		paste! {
			type [<$vec_n AsArray>] = [Float; count_tts!($($component)+)];
			
			impl From<$vec_n> for [<$vec_n AsArray>] {
				fn from(v: $vec_n) -> Self { [ $(v.$component,)+ ] }
			}
			impl From<[<$vec_n AsArray>]> for $vec_n {
				fn from([ $($component,)+ ]: [<$vec_n AsArray>]) -> Self {
					$vec_n { $($component,)+ }
				}
			}
		}
	};
}

vector_def!(pub struct Vec2 { x, y });
vector_def!(pub struct Vec3 { x, y, z });

impl Vec3 {
	/// Get the cross product of `self` cross `other`.
	pub fn cross(self, other: Self) -> Self {
		Self {
			x: Float::mul_add(self.y, other.z, -self.z * other.y),
			y: Float::mul_add(self.z, other.x, -self.x * other.z),
			z: Float::mul_add(self.x, other.y, -self.y * other.x),
		}
	}
}

impl From<Color> for [u8; 3] {
	fn from(c: Color) -> Self {
		[
			(255.999 * c.x).floor() as u8,
			(255.999 * c.y).floor() as u8,
			(255.999 * c.z).floor() as u8
		]
	}
}

// Alternate Names

pub type Color = Vec3;
impl Color {
	pub fn gamma_accurate_average(self, samples: usize) -> Self {
		let incorrect_avg = self / (samples as Float);
		Self { x: incorrect_avg.x.sqrt(), y: incorrect_avg.y.sqrt(), z: incorrect_avg.z.sqrt() }
	}
}

pub type Point2 = Vec2;
pub type Point3 = Vec3;
