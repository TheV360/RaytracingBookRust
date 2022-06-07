use crate::vector::{Vec3, Color, Float};
use crate::ray::{Ray, HitInfo};
use crate::util; // Because then it's obvious that "HEY THIS IS FROM UTIL!"

pub trait Material {
	fn scatter(&self, ray: Ray, hit: HitInfo) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
	pub albedo: Color,
}
impl Material for Lambertian {
	fn scatter(&self, _ray: Ray, hit: HitInfo) -> Option<(Color, Ray)> {
		let scatter_direction = hit.normal + util::random_unit_vector();
		let scattered = Ray::new(hit.position, scatter_direction);
		let attenuation = self.albedo;
		
		Some((attenuation, scattered))
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
	pub albedo: Color,
	pub fuzz: Float,
}
impl Material for Metal {
	fn scatter(&self, ray: Ray, hit: HitInfo) -> Option<(Color, Ray)> {
		let reflected = ray.direction.normalize().reflect(hit.normal);
		let scattered = Ray::new(hit.position, reflected + self.fuzz * util::random_in_unit_sphere());
		let attenuation = self.albedo;
		
		Some((attenuation, scattered))
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
	pub refractive_index: Float,
}
impl Material for Dielectric {
	fn scatter(&self, ray: Ray, hit: HitInfo) -> Option<(Color, Ray)> {
		let etai_over_etat = if hit.front_face { 
			self.refractive_index.recip()
		} else {
			self.refractive_index
		};
		
		let unit_direction = ray.direction.normalize();
		
		let cos_theta = Float::min(Vec3::dot(-unit_direction, hit.normal), 1.0);
		let sin_theta = Float::sqrt(1.0 - cos_theta.powi(2));
		if etai_over_etat * sin_theta > 1.0 {
			let reflected = Vec3::reflect(unit_direction, hit.normal);
			let scattered = Ray::new(hit.position, reflected);
			
			Some((Color::ONE, scattered))
		} else {
			let reflect_prob = util::schlick(cos_theta, etai_over_etat);
			if util::random_float() < reflect_prob {
				let reflected = Vec3::reflect(unit_direction, hit.normal);
				let scattered = Ray::new(hit.position, reflected);
				
				Some((Color::ONE, scattered))
			} else {
				let refracted = util::refract(unit_direction, hit.normal, etai_over_etat);
				let scattered = Ray::new(hit.position, refracted);
				
				Some((Color::ONE, scattered))
			}
		}
	}
}
