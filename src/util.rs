use crate::vector::*;
use crate::ray::*;

use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum DiffuseMode {
	FlawedLambertian,
	IdealLambertian,
	Hemisphere
}
impl DiffuseMode {
	pub fn get_diffuse_result(self, r: RayHitInfo) -> Vec3 {
		r.position + match self {
			DiffuseMode::FlawedLambertian => r.normal + random_in_unit_sphere(),
			DiffuseMode::IdealLambertian => r.normal + random_unit_vector(),
			DiffuseMode::Hemisphere => random_in_hemisphere(r.normal),
		}
	}
}

pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::new((random::<Float>() * 2.0) - 1.0, (random::<Float>() * 2.0) - 1.0, (random::<Float>() * 2.0) - 1.0);
		if p.squared_magnitude() >= 1.0 { return p; }
	}
}

pub fn random_unit_vector() -> Vec3 {
	let a: Float = random::<Float>() * 2.0 * (std::f64::consts::PI as Float);
	let z: Float = (random::<Float>() * 2.0) - 1.0;
	let r = Float::sqrt(1.0 - z.powi(2));
	Vec3::new(
		r * Float::cos(a),
		r * Float::sin(a),
		z
	)
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
	let o = random_in_unit_sphere();
	if o.dot(normal) > 0.0 {
		return o;
	} else {
		return -o;
	}
}
