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

use vector::{Vec3, Color, Float};
use ray::Ray;
use material::{Lambertian, Metal, Dielectric};
use world::{World, Object};
use camera::Camera;
use sphere::Sphere;

//////////////////

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const ASPECT_RATIO: Float = (WIDTH as Float) / (HEIGHT as Float);

const SAMPLES_PER_PIXEL: usize = 128;
const MAX_DEPTH: usize = 128;

const PROGRESS_BAR_CHARS: usize = 32;

fn ray_color(world: &World, ray: Ray, depth: usize) -> Color {
	// If we've recursed too deep, stop.
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
	
	// Sky color.
	let t = 0.5 * (ray.direction.y + 1.0);
	Color::lerp(Color::new(0.5, 0.7, 1.0), Color::ONE, t)
}

fn print_console_progress_bar(percent_done: Float) {
	let percent_done_int = (percent_done * PROGRESS_BAR_CHARS as Float).round() as usize;
	eprint!("\r{}{} {:.2}%", "\u{2588}".repeat(percent_done_int), "\u{2592}".repeat(PROGRESS_BAR_CHARS - percent_done_int), percent_done * 100.0);
	io::stderr().flush().expect("Could not flush stderr! That's weird.");
}

fn main() {
	let mut rng = thread_rng();
	
	let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
	
	let world = World { objects: vec![
		Object::new(
			Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)), 
			Box::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) })
		),
		Object::new(
			Box::new(Sphere::new(Vec3::new_z(-1.0), 0.5)),
			Box::new(Lambertian { albedo: Color::new(0.7, 0.3, 0.3) })
		),
		Object::new(
			Box::new(Sphere::new(Vec3::new(-1.0, -0.25, -1.5), 0.5)),
			Box::new(Metal { albedo: Color::all(0.8), fuzz: 0.0 })
		),
		Object::new(
			Box::new(Sphere::new(Vec3::new(0.8, 0.5, -1.0), 0.25)),
			Box::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0 })
		),
		Object::new(
			Box::new(Sphere::new(Vec3::new(-1.25, -0.25, -0.75), 0.25)),
			Box::new(Dielectric { refractive_index: 1.5 })
		),
	] };
	
	let camera = Camera::new(Vec3::ZERO, ASPECT_RATIO, 2.0, 1.0);
	
	let start_of_op = Instant::now();
	for y in (0..HEIGHT).rev() {
		print_console_progress_bar(1.0 - ((y as Float) / (HEIGHT as Float)));
		
		for x in 0..WIDTH {
			let mut pixel_color = Color::ZERO;
			
			for _ in 0..SAMPLES_PER_PIXEL {
				// Get uv screen coordinates for the camera.
				let u = (x as Float + rng.gen::<Float>()) / WIDTH as Float;
				let v = 1.0 - ((y as Float + rng.gen::<Float>()) / HEIGHT as Float);
				
				// Shoot ray out of the camera.
				let r = camera.get_ray(u, v);
				pixel_color += ray_color(&world, r, 0);
			}
			
			// And then write the color.
			let tmp = pixel_color / SAMPLES_PER_PIXEL as Float;
			let gamma_correct_color = Vec3::new(tmp.x.sqrt(), tmp.y.sqrt(), tmp.z.sqrt());
			image.put_pixel(x as u32, y as u32, Rgb(gamma_correct_color.into()));
		}
	}
	let duration = start_of_op.elapsed();
	
	// Note: the fancy \x1B[?C characters move the cursor forward ? characters.
	// This sadly doesn't work on Windows Terminal, but that's because it sucks.
	
	eprint!("\r\x1B[{}C Saving...", PROGRESS_BAR_CHARS);
	image.save("output.png").unwrap();
	
	eprintln!("\r\x1B[{}C Done. Took {:.2?}.", PROGRESS_BAR_CHARS, duration);
}
