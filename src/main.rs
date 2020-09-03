use std::time::Instant;
use std::io::{self, Write};

use image::{RgbImage, Rgb};

use rand::prelude::*;

//////////////////

mod vector;
mod ray;
mod util;
mod material;
mod world;
mod camera;
mod sphere;

use vector::*;
use ray::*;
use material::*;
use world::*;
use camera::*;
use sphere::*;

//////////////////

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const ASPECT_RATIO: Float = (WIDTH as Float) / (HEIGHT as Float);

const SAMPLES_PER_PIXEL: usize = 128;
const MAX_DEPTH: usize = 128;

const PROGRESS_BAR_CHARS: usize = 32;

fn ray_color(world: &World, ray: Ray, depth: usize) -> Color {
	if depth >= MAX_DEPTH {
		return Color::ZERO;
	}
	
	if let Some((obj, hit)) = world.hit(ray, 0.001..Float::INFINITY) {
		if let Some((attenuation, scattered)) = obj.material.scatter(ray, hit) {
			return attenuation * ray_color(world, scattered, depth + 1);
		} else {
			return Color::ZERO;
		}
	}
	
	let t = 0.5 * (ray.direction.y + 1.0);
	Color::lerp(Color::new(0.5, 0.7, 1.0), Color::new(1.0, 1.0, 1.0), t)
}

fn main() {
	let mut rng = thread_rng();
	
	let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
	let start_of_op = Instant::now();
	
	let world: World = World { objects: vec![
		Object::new(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)), Box::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) })),
		Object::new(Box::new(Sphere::new(Vec3::new_z(-1.0), 0.5)), Box::new(Lambertian { albedo: Color::new(0.7, 0.3, 0.3) })),
		Object::new(Box::new(Sphere::new(Vec3::new(-1.0, -0.25, -1.5), 0.5)), Box::new(Metal { albedo: Color::all(0.8), fuzz: 0.3 })),
		Object::new(Box::new(Sphere::new(Vec3::new(0.8, 0.5, -1.0), 0.5)), Box::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0 })),
	] };
	
	let camera = Camera::new(Vec3::ZERO, ASPECT_RATIO, 2.0, 1.0);
	
	for y in (0..HEIGHT).rev() {
		let percent_done = 1.0 - ((y as Float) / (HEIGHT as Float));
		let percent_done_int = (percent_done * PROGRESS_BAR_CHARS as Float).round() as usize;
		eprint!("\r{}{} {}%", "█".repeat(percent_done_int), "▒".repeat(PROGRESS_BAR_CHARS - percent_done_int), (percent_done * 100.0).round());
		io::stderr().flush().ok().expect("fuck whoops");
		
		for x in 0..WIDTH {
			let mut pixel_color = Color::ZERO;
			
			for _s in 0..SAMPLES_PER_PIXEL {
				let u = (x as Float + rng.gen::<Float>()) / WIDTH as Float;
				let v = 1.0 - ((y as Float + rng.gen::<Float>()) / HEIGHT as Float);
				
				let r = camera.get_ray(u, v);
				pixel_color = pixel_color + ray_color(&world, r, 0);
			}
			
			// And then write the color.
			let tmp = pixel_color / SAMPLES_PER_PIXEL as Float;
			let gamma_correct_color = Vec3::new(tmp.x.sqrt(), tmp.y.sqrt(), tmp.z.sqrt());
			image.put_pixel(x as u32, y as u32, Rgb(gamma_correct_color.into()));
		}
	}
	let duration = start_of_op.elapsed();
	
	eprint!("\r{} Saving...", "█".repeat(PROGRESS_BAR_CHARS));
	image.save("output.png").unwrap();
	
	eprintln!("\r{} Done. Took {:?}.", "█".repeat(PROGRESS_BAR_CHARS), duration);
}
