use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;

pub type Float = f64;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec3 {
	pub x: Float,
	pub y: Float,
	pub z: Float,
}
impl Add for Vec3 {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, }
	}
}
impl Sub for Vec3 {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, }
	}
}
impl Mul<Float> for Vec3 {
	type Output = Self;
	fn mul(self, other: Float) -> Self::Output {
		Vec3 { x: self.x * other, y: self.y * other, z: self.z * other, }
	}
}
impl Mul<Vec3> for Float {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Self::Output {
		Vec3 { x: self * other.x, y: self * other.y, z: self * other.z, }
	}
}
impl Div<Float> for Vec3 {
	type Output = Self;
	fn div(self, other: Float) -> Self::Output {
		Vec3 { x: self.x / other, y: self.y / other, z: self.z / other, }
	}
}
impl Neg for Vec3 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Vec3 { x: -self.x, y: -self.y, z: -self.z }
	}
}
impl Vec3 {
	/// A Vector with all components set to 0.
	pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
	
	/// A Vector with all components set to 1.
	pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
	
	/// Make a new Vector, setting the x, y, and z components to their respective parameters.
	pub fn new(x: Float, y: Float, z: Float) -> Self {
		Vec3 { x, y, z }
	}
	
	/// Make a new vector with `x` as the x component and 0 for the others.
	pub fn new_x(x: Float) -> Self {
		Vec3 { x, y: 0.0, z: 0.0 }
	}
	
	/// Make a new vector with `y` as the y component and 0 for the others.
	pub fn new_y(y: Float) -> Self {
		Vec3 { x: 0.0, y, z: 0.0 }
	}
	
	/// Make a new vector with `z` as the z component and 0 for the others.
	pub fn new_z(z: Float) -> Self {
		Vec3 { x: 0.0, y: 0.0, z }
	}
	
	/// Get the dot product of `self` dot `other`.
	pub fn dot(self, other: Self) -> Float {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
	
	/// Get the cross product of `self` cross `other`.
	pub fn cross(self, other: Self) -> Self {
		Vec3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
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
	/// Interpolates between the vectors `self` and `other` by a factor `t`.  Linearly. Uhh. good documentation.
	pub fn lerp(self, other: Self, t: Float) -> Self {
		self * t + other * (1.0 - t)
	}
}

impl Default for Vec3 {
	fn default() -> Self {
		Vec3::ZERO
	}
}
impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl From<Vec3> for [u8; 3] {
	fn from(v: Vec3) -> Self {
		[
			(255.999 * v.x).floor() as u8,
			(255.999 * v.y).floor() as u8,
			(255.999 * v.z).floor() as u8
		]
	}
}

pub type Color = Vec3;
pub type Point3 = Vec3;
