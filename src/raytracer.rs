use crate::vector::{Vec2, Color, Float};
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
	pub fn get_pixel(&self, world: &World, camera: &Camera, uv: Vec2) -> Color {
		let mut sample_results = Color::ZERO;
		
		// Collect all the samples.
		for _ in 0..self.samples {
			// Offset the uv coords slightly.
			let uv_ofs = Vec2::new(
				util::random_float() / self.screen.width as Float,
				util::random_float() / self.screen.height as Float
			);
			// This assumes that the uv coordinates represent
			// the bottom left of each pixel in the camera, as
			// `random_float` outputs a number in [0, 1).
			
			// Shoot ray out of camera, and add it to the samples.
			sample_results += self.get_sample(world, camera, uv + uv_ofs);
		}
		
		// Finally, average all the samples and calculate the gamma-correct color.
		sample_results.gamma_accurate_average(self.samples)
	}
	
	pub fn get_sample(&self, world: &World, camera: &Camera, uv: Vec2) -> Color {
		self.ray_color(world, camera.get_ray(uv), 0)
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

// TODO: make a Vec3i and replace Screen with it.
#[derive(Copy, Clone, Debug)]
pub struct Screen {
	pub width: usize, pub height: usize,
}
impl Screen {
	pub fn aspect_ratio(&self) -> Float {
		self.width as Float / self.height as Float
	}
}
impl From<(usize, usize)> for Screen {
	fn from((width, height): (usize, usize)) -> Self {
		Screen { width, height }
	}
}
