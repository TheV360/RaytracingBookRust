use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::thread::sleep;

use image::{RgbImage, Rgb};

//////////////////

mod vector;
mod ray;
mod sphere;

use vector::*;
use ray::*;
use sphere::*;

//////////////////

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const ASPECT_RATIO: Float = (WIDTH as Float) / (HEIGHT as Float);

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
	let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
	let start_of_op = Instant::now();
	
	let viewport_height = 2.0;
	let viewport_width = ASPECT_RATIO * viewport_height;
	let focal_length = 1.0;
	
	let origin = Point3::ZERO;
	let horizontal = Vec3::new_x(viewport_width);
	let vertical = Vec3::new_y(viewport_height);
	let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new_z(focal_length);
	
	let mut world: Vec<Box<dyn RayHits>> = vec![
		Box::new(Sphere::new(Vec3::new_z(-1.0), 0.5)),
		Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
	];
	
	/*for i in -3isize..=3 {
		for j in -3isize..=3 {
			world.push(Box::new(
				Sphere::new(
					Vec3::new(i as Float * 3.0, j as Float * 3.0, -(5 + i.abs() + j.abs()) as Float),
					0.5
				)
			));
		}
	}*/
	
	for y in (0..HEIGHT).rev() {
		let percent_done = 1.0 - ((y as Float) / (HEIGHT as Float));
		let percent_done_int = (percent_done * PROGRESS_BAR_CHARS as Float).round() as usize;
		eprint!("\r{}{} {}%", "█".repeat(percent_done_int), "▒".repeat(PROGRESS_BAR_CHARS - percent_done_int), (percent_done * 100.0).round());
		io::stderr().flush().ok().expect("fuck whoops");
		
		for x in 0..WIDTH {
			let u = (x as Float) / (WIDTH as Float);
			let v = 1.0 - ((y as Float) / (HEIGHT as Float));
			
			let r = Ray::new(origin, (lower_left_corner + u * horizontal + v * vertical - origin).normalize());
			// let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
			let pixel_color = ray_color(&world, r);
			
			// And then write the color.
			image.put_pixel(x as u32, y as u32, Rgb(pixel_color.into()));
		}
	}
	let duration = start_of_op.elapsed();
	
	eprint!("\r{} Saving...", "█".repeat(PROGRESS_BAR_CHARS));
	image.save("output.png").unwrap();
	
	eprintln!("\r{} Done. Took {:?}.", "█".repeat(PROGRESS_BAR_CHARS), duration);
}
