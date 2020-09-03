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

#[derive(Copy, Clone)]
pub struct RayHitInfo {
	pub position: Point3,
	pub normal: Vec3,
	pub t: Float,
	pub front_face: bool,
}
impl RayHitInfo {
	pub fn get_face_normal_info(ray: Ray, outward_normal: Vec3) -> (bool, Vec3) {
		let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
		let normal = if front_face { outward_normal } else { -outward_normal };
		
		(front_face, normal)
	}
}

pub trait Hittable {
	fn ray_hits(&self, t_range: Range<Float>, ray: Ray) -> Option<RayHitInfo>;
}
