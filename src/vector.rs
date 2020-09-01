use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

pub type Float = f64;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec3 {
	pub x: Float, pub y: Float, pub z: Float,
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
impl Vec3 {
	pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
	
	fn dot(self, other: Self) -> Float {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
	fn cross(self, other: Self) -> Self {
		Vec3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}
	
	fn squared_magnitude(self) -> Float {
		self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
	}
	fn magnitude(self) -> Float {
		self.squared_magnitude().sqrt()
	}
	fn normalize(self) -> Self {
		self / self.magnitude()
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
