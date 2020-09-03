use crate::vector::*;
use crate::ray::*;
use crate::util; // Because then it's obvious that "HEY THIS IS FROM UTIL!"

pub trait Material {
	fn scatter(self, ray: Ray, hit: RayHitInfo) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
	pub albedo: Color,
}
impl Material for Lambertian {
	fn scatter(self, _ray: Ray, hit: RayHitInfo) -> Option<(Color, Ray)> {
		let scatter_direction = hit.normal + util::random_unit_vector();
		let scattered = Ray::new(hit.position, scatter_direction);
		let attenuation = self.albedo;
		
		Some((attenuation, scattered))
	}
}

#[derive(Copy, Clone)]
pub struct Metal {
	pub albedo: Color,
}
impl Material for Metal {
	fn scatter(self, ray: Ray, hit: RayHitInfo) -> Option<(Color, Ray)> {
		let reflected = ray.direction.normalize().reflect(hit.normal);
		let scattered = Ray::new(hit.position, reflected);
		let attenuation = self.albedo;
		
		Some((attenuation, scattered))
	}
}
