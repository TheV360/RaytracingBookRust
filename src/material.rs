use crate::vector::{Color, Float};
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
