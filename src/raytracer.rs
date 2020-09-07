use crate::vector::{Color, Float};
use crate::ray::Ray;
use crate::world::World;
use crate::camera::Camera;
use crate::util;

#[derive(Copy, Clone, Debug)]
pub struct Raytracer {
	pub screen: Screen,
	pub max_depth: usize,
	pub samples: usize,
}
impl Raytracer {
	pub fn get_pixel(&self, world: &World, camera: &Camera, u: Float, v: Float) -> Color {
		let mut sample_results = Color::ZERO;
		
		// Collect all the samples.
		for _ in 0..self.samples {
			// Offset the uv coords slightly.
			// Note that this assumes that uv coords will be biased towards the bottom left.
			let u_ofs = util::random_float() / self.screen.width as Float;
			let v_ofs = util::random_float() / self.screen.height as Float;
			
			// Shoot ray out of camera, and add it to the samples.
			let ray = camera.get_ray(u + u_ofs, v + v_ofs);
			sample_results += self.ray_color(world, ray, 0);
		}
		
		// Finally, average all the samples and calculate the gamma-correct color.
		sample_results.gamma_accurate_average(self.samples)
	}
	
	pub fn ray_color(&self, world: &World, ray: Ray, depth: usize) -> Color {
		// If we've recursed too deep, stop.
		if depth >= self.max_depth {
			return Color::ZERO;
		}
		
		if let Some((obj, hit)) = world.hit(ray, 0.001..Float::INFINITY) {
			if let Some((attenuation, scattered)) = obj.material.scatter(ray, hit) {
				return attenuation * self.ray_color(world, scattered, depth + 1);
			} else {
				return Color::ZERO;
			}
		}
		
		// Sky color.
		let t = 0.5 * (ray.direction.y + 1.0);
		Color::lerp(world.sky_color.0, world.sky_color.1, t)
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Screen {
	pub width: usize, pub height: usize,
}
impl Screen {
	pub fn get_aspect_ratio(&self) -> Float {
		self.width as Float / self.height as Float
	}
}
