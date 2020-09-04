use crate::vector::{Vec3, Float};

use rand::prelude::*;

// I don't want to reimport rand everywhere.
pub fn random_float() -> Float { random() }

pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::new(
			(random::<Float>() * 2.0) - 1.0,
			(random::<Float>() * 2.0) - 1.0,
			(random::<Float>() * 2.0) - 1.0
		);
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

pub fn refract(unit_vector: Vec3, normal: Vec3, etai_over_etat: Float) -> Vec3 {
	let cos_theta = Vec3::dot(-unit_vector, normal);
	let r_out_perpendicular = etai_over_etat * (unit_vector + cos_theta * normal);
	let r_out_parallel = -Float::sqrt(Float::abs(1.0 - r_out_perpendicular.squared_magnitude())) * normal;
	r_out_perpendicular + r_out_parallel
}

pub fn schlick(cosine: Float, refractive_index: Float) -> Float {
	let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
	Float::mul_add(1.0 - r0, (1.0 - cosine).powi(5), r0)
}
