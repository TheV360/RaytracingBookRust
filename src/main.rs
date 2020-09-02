use std::time::{Duration, Instant};
use std::io::{self, Write};

use image::{RgbImage, Rgb};

use rand::prelude::*;

//////////////////

mod vector;
mod ray;
mod camera;
mod sphere;

use vector::*;
use ray::*;
use camera::*;
use sphere::*;

//////////////////

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const ASPECT_RATIO: Float = (WIDTH as Float) / (HEIGHT as Float);

const SAMPLES_PER_PIXEL: usize = 32;

const PROGRESS_BAR_CHARS: usize = 32;

fn ray_color(world: &Vec<Box<dyn RayHits>>, ray: Ray) -> Color {
	for object in world {
		let t = object.ray_hits(0.0..Float::INFINITY, ray); //tODO: what does this want
		if let Some(v) = t {
			return (v.normal + Color::new(1.0, 1.0, 1.0)) / 2.0;
		}
	}
	let t = 0.5 * (ray.direction.y + 1.0);
	Color::lerp(Color::new(0.5, 0.7, 1.0), Color::new(1.0, 1.0, 1.0), t)
}

fn main() {
	let mut rng = thread_rng();
	
	let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
	let start_of_op = Instant::now();
	
	let world: Vec<Box<dyn RayHits>> = vec![
		Box::new(Sphere::new(Vec3::new_z(-1.0), 0.5)),
		Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
	];
	
	let camera = Camera::new(Vec3::ZERO, ASPECT_RATIO, 2.0, 1.0);
	
	for y in (0..HEIGHT).rev() {
		let percent_done = 1.0 - ((y as Float) / (HEIGHT as Float));
		let percent_done_int = (percent_done * PROGRESS_BAR_CHARS as Float).round() as usize;
		eprint!("\r{}{} {}%", "█".repeat(percent_done_int), "▒".repeat(PROGRESS_BAR_CHARS - percent_done_int), (percent_done * 100.0).round());
		io::stderr().flush().ok().expect("fuck whoops");
		
		for x in 0..WIDTH {
			let mut pixel_color = Color::ZERO;
			
			for _s in 0..SAMPLES_PER_PIXEL {
				let u = ((x as Float) + rng.gen::<Float>()) / (WIDTH as Float);
				let v = 1.0 - (((y as Float) + rng.gen::<Float>()) / (HEIGHT as Float));
				
				let r = camera.get_ray(u, v);
				pixel_color = pixel_color + ray_color(&world, r);
			}
			
			// And then write the color.
			image.put_pixel(x as u32, y as u32, Rgb((pixel_color / SAMPLES_PER_PIXEL as Float).into()));
		}
	}
	let duration = start_of_op.elapsed();
	
	eprint!("\r{} Saving...", "█".repeat(PROGRESS_BAR_CHARS));
	image.save("output.png").unwrap();
	
	eprintln!("\r{} Done. Took {:?}.", "█".repeat(PROGRESS_BAR_CHARS), duration);
}
