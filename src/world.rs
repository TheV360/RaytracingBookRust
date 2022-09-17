use crate::vector::{Color, Float};
use crate::ray::{Ray, HitInfo};
use crate::solid::Hittable;
use crate::material::Material;

use std::ops::Range;

/// An Object is a [Hittable] shape with some [Material].
pub struct Object {
	pub solid: Box<dyn Hittable + Send + Sync>,
	pub material: Box<dyn Material + Send + Sync>,
}
impl Object {
	pub fn new(solid: Box<dyn Hittable + Send + Sync>, material: Box<dyn Material + Send + Sync>) -> Self {
		Self { solid, material }
	}
}

/// A World has a bunch of [Object]s.
pub struct World {
	pub objects: Vec<Object>,
	pub sky_color: (Color, Color),
}
impl World {
	/// Shoot a [Ray] out and have it react to [Object]s in the [World].
	/// 
	/// Returns which Object it hit, along with some [HitInfo].
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
impl Default for World {
	fn default() -> Self {
		World {
			sky_color: (Color::new(0.5, 0.7, 1.0), Color::ONE),
			objects: Vec::new(),
		}
	}
}
