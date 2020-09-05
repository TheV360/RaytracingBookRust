use crate::vector::Float;
use crate::ray::{Ray, HitInfo, Hittable};
use crate::material::Material;

use std::ops::Range;

pub struct Object {
	pub solid: Box<dyn Hittable + Send + Sync>,
	pub material: Box<dyn Material + Send + Sync>,
}
impl Object {
	pub fn new(solid: Box<dyn Hittable + Send + Sync>, material: Box<dyn Material + Send + Sync>) -> Self {
		Self { solid, material }
	}
}

pub struct World {
	pub objects: Vec<Object>,
}
impl World {
	pub fn hit(&self, ray: Ray, t_range: Range<Float>) -> Option<(&Object, HitInfo)> {
		let mut tmp_hit: Option<(&Object, HitInfo)> = None;
		let mut closest = t_range.end;
		
		for object in &self.objects {
			if let Some(hit) = object.solid.ray_hits(t_range.start..closest, ray) {
				closest = hit.t;
				tmp_hit = Some((object, hit));
			}
		}
		
		tmp_hit
	}
}
