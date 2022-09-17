use crate::vector::{Vec3, Point3, Float};

/// You know it from geometry class, it's a Ray!!!
/// 
/// Has a position and a direction.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Ray {
	pub position: Point3,
	pub direction: Vec3,
}
impl Ray {
	pub const fn new(position: Point3, direction: Vec3) -> Self {
		Self { position, direction }
	}
	
	pub fn at(&self, t: Float) -> Point3 {
		self.position + t * self.direction
	}
}

#[derive(Copy, Clone, Debug)]
pub struct HitInfo {
	pub position: Point3,
	pub normal: Vec3,
	pub t: Float,
	pub front_face: bool,
}
impl HitInfo {
	pub fn get_face_normal_info(ray: Ray, outward_normal: Vec3) -> (bool, Vec3) {
		let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
		let normal = if front_face { outward_normal } else { -outward_normal };
		
		(front_face, normal)
	}
}
