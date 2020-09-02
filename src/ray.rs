use crate::vector::*;

use std::ops::Range;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Ray {
	pub position: Point3,
	pub direction: Vec3,
}
impl Ray {
	pub fn new(position: Point3, direction: Vec3) -> Self {
		Ray { position, direction }
	}
	
	pub fn at(&self, t: Float) -> Point3 {
		self.position + t * self.direction
	}
}

pub struct RayHitInfo {
	pub position: Point3,
	pub normal: Vec3,
	pub t: Float,
}

pub trait RayHits {
	fn ray_hits(&self, t_range: Range<Float>, ray: Ray) -> Option<RayHitInfo>;
}
