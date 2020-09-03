use crate::vector::*;
use crate::ray::*;
use crate::material::*;

use std::ops::Range;

type Object = (Box<dyn Hittable>, Box<dyn Material>);

pub struct World {
	pub objects: Vec<Object>,
}
impl World {
	pub fn hit(&self, ray: Ray, t_range: Range<Float>) -> Option<RayHitInfo> {
		let mut tmp_hit: Option<RayHitInfo> = None;
		let mut closest = t_range.end;
		
		for object in self.objects.iter() {
			if let Some(hit) = object.0.ray_hits(t_range.start..closest, ray) {
				closest = hit.t;
				tmp_hit = Some(hit);
			}
		}
		
		tmp_hit
	}
}
