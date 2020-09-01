use crate::vector::*;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct Ray {
	pub position: Point3,
	pub direction: Vec3,
}
impl Ray {
	fn at(&self, t: Float) -> Point3 {
		self.position + t * self.direction
	}
}
