use std::ops;
use std::fmt;

/// The `Float` type lets you easily switch between single-precision and double-precision float types.
/// 
/// If someone implements a higher-precision type, they could easily replace this type declaration with
/// their own type.
pub type Float = f64;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default)]
/// A Vector with 3 components: x, y, and z.
pub struct Vec3 {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}

// Operators

impl ops::Neg for Vec3 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self { x: -self.x, y: -self.y, z: -self.z }
	}
}

impl ops::Add for Vec3 {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, }
	}
}

impl ops::Sub for Vec3 {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, }
	}
}

impl ops::Mul<Float> for Vec3 {
	type Output = Self;
	fn mul(self, other: Float) -> Self::Output {
		Self { x: self.x * other, y: self.y * other, z: self.z * other, }
	}
}
impl ops::Mul<Vec3> for Float {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Self::Output { other * self }
}
impl ops::Mul for Vec3 {
	type Output = Self;
	/// This will cause a bit of confusion. This is a naive "multiply each component with its counterpart" thing.
	fn mul(self, other: Self) -> Self::Output {
		Self { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z, }
	}
}

impl ops::Div<Float> for Vec3 {
	type Output = Self;
	fn div(self, other: Float) -> Self::Output {
		Self { x: self.x / other, y: self.y / other, z: self.z / other, }
	}
}

// Assign Operators

impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		*self = Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, }
	}
}

impl ops::SubAssign for Vec3 {
	fn sub_assign(&mut self, other: Self) {
		*self = Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, }
	}
}

impl ops::MulAssign<Float> for Vec3 {
	fn mul_assign(&mut self, other: Float) {
		*self = Self { x: self.x * other, y: self.y * other, z: self.z * other, }
	}
}

impl ops::DivAssign<Float> for Vec3 {
	fn div_assign(&mut self, other: Float) {
		*self = Self { x: self.x / other, y: self.y / other, z: self.z / other, }
	}
}

// Main impl

impl Vec3 {
	/// A Vector with all components set to 0.
	pub const ZERO: Self = Self::all(0.0);
	
	/// A Vector with all components set to 1.
	pub const ONE: Self = Self::all(1.0);
	
	/// Make a new Vector, setting the x, y, and z components to their respective parameters.
	pub const fn new(x: Float, y: Float, z: Float) -> Self {
		Self { x, y, z }
	}
	
	/// Make a new Vector, setting all components to `w`.
	pub const fn all(w: Float) -> Self {
		Self { x: w, y: w, z: w }
	}
	
	/// Make a new vector with `x` as the x component and 0 for the others.
	pub const fn new_x(x: Float) -> Self {
		Self { x, y: 0.0, z: 0.0 }
	}
	
	/// Make a new vector with `y` as the y component and 0 for the others.
	pub const fn new_y(y: Float) -> Self {
		Self { x: 0.0, y, z: 0.0 }
	}
	
	/// Make a new vector with `z` as the z component and 0 for the others.
	pub const fn new_z(z: Float) -> Self {
		Self { x: 0.0, y: 0.0, z }
	}
	
	/// Get the dot product of `self` dot `other`.
	pub fn dot(self, other: Self) -> Float {
		Float::mul_add(self.x, other.x, Float::mul_add(self.y, other.y, self.z * other.z))
	}
	
	/// Get the cross product of `self` cross `other`.
	pub fn cross(self, other: Self) -> Self {
		Self {
			x: Float::mul_add(self.y, other.z, -self.z * other.y),
			y: Float::mul_add(self.z, other.x, -self.x * other.z),
			z: Float::mul_add(self.x, other.y, -self.y * other.x),
		}
	}
	
	/// Get the squared length of a vector. Cheaper to compute than `magnitude`.
	/// Useful for when you only need to know if one length is greater than the other, etc.
	pub fn squared_magnitude(self) -> Float {
		self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
	}
	
	/// Get the length of a vector.
	pub fn magnitude(self) -> Float {
		self.squared_magnitude().sqrt()
	}
	
	/// Normalize a vector, making it a unit vector.
	pub fn normalize(self) -> Self {
		self / self.magnitude()
	}
	
	/// This will project the vector `self` onto the vector `other`.
	pub fn project(self, other: Self) -> Self {
		(other.dot(self) / other.magnitude()) * other
	}
	
	/// Reflects the vector `self` from the normal vector `other`.
	pub fn reflect(self, other: Self) -> Self {
		self - 2.0 * self.dot(other) * other
	}
	
	// TODO: make this generic???
	/// Interpolates between the vectors `self` and `other` by a factor `t`.
	/// Linearly. Uhh. good documentation.
	pub fn lerp(self, other: Self, t: Float) -> Self {
		self * (1.0 - t) + other * t
	}
}

// Additional Utility Traits

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
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

pub type Point3 = Vec3;
