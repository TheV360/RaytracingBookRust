use crate::vector::{Float};
use crate::ray::{Ray, HitInfo};

use std::ops::Range;

/// The `Hittable` trait encompasses all things that can be hit by a [Ray].
/// If some object implements the `Hittable` trait, then it can easily be
/// drawn in the world with an associated [Material].
pub trait Hittable {
	fn ray_hits(&self, t_range: Range<Float>, ray: Ray) -> Option<HitInfo>;
	// fn bounding_box(&self) -> Option<BoundingBox>;
}

pub mod sphere;
