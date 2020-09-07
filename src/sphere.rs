use crate::vector::{Vec3, Point3, Float};
use crate::ray::{Ray, HitInfo, Hittable};

use std::ops::Range;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default)]
pub struct Sphere {
	pub center: Point3,
	pub radius: Float,
}
impl Sphere {
	pub const fn new(center: Point3, radius: Float) -> Self {
		Self { center, radius }
	}
}
impl Hittable for Sphere {
	fn ray_hits(&self, t_range: Range<Float>, ray: Ray) -> Option<HitInfo> {
		let ofs: Vec3 = ray.position - self.center;
		
		let a = ray.direction.squared_magnitude();
		let half_b = ofs.dot(ray.direction);
		let c = ofs.squared_magnitude() - self.radius.powi(2);
		
		let discriminant = half_b.powi(2) - a * c;
		if discriminant > 0.0 {
			let disc_root = Float::sqrt(discriminant);
			
			// TODO: this is still a bit ugly.
			for sign in (-1..=1).step_by(2) {
				let t = disc_root.mul_add(sign as Float, -half_b) / a;
				
				if t_range.contains(&t) {
					let position = ray.at(t);
					let (front_face, normal) =
						HitInfo::get_face_normal_info(ray, (position - self.center) / self.radius);
					
					return Some(HitInfo { position, normal, t, front_face });
				}
			}
		}
		
		None
	}
}
