use crate::vector::*;
use crate::ray::*;

use std::ops::Range;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Sphere {
	pub center: Point3,
	pub radius: Float,
}
impl Sphere {
	pub fn new(center: Point3, radius: Float) -> Self {
		Sphere { center, radius }
	}
}
impl RayHits for Sphere {
	fn ray_hits(&self, t_range: Range<Float>, ray: Ray) -> Option<RayHitInfo> {
		let ofs: Vec3 = ray.position - self.center;
		
		let a = ray.direction.squared_magnitude();
		let half_b = ofs.dot(ray.direction);
		let c = ofs.squared_magnitude() - self.radius.powi(2);
		
		let discriminant = half_b.powi(2) - a * c;
		if discriminant > 0.0 {
			let disc_root = Float::sqrt(discriminant);
			
			let tmp = (-half_b - disc_root) / a;
			if t_range.contains(&tmp) {
				let t = tmp;
				let position = ray.at(t);
				let normal = (position - self.center) / self.radius;
				
				return Some(RayHitInfo { position, normal, t });
			}
			
			let tmp = (-half_b + disc_root) / a;
			if t_range.contains(&tmp) {
				let t = tmp;
				let position = ray.at(t);
				let normal = (position - self.center) / self.radius;
				
				return Some(RayHitInfo { position, normal, t });
			}
		}
		
		return None;
	}
}
