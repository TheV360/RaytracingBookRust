use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crossbeam_utils::thread;

use image::{RgbImage, Rgb};

//////////////////

mod vector;
mod ray;
mod util;
mod material;
mod world;
mod camera;
mod sphere;
mod raytracer;

use vector::{Vec3, Point3, Color, Float};
use material::{Lambertian, Metal, Dielectric};
use world::{World, Object};
use camera::{Camera, CameraLens};
use sphere::Sphere;
use raytracer::{Raytracer, Screen};

//////////////////

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

const SAMPLES_PER_PIXEL: usize = 64;
const MAX_DEPTH: usize = 64;

const RENDER_THREADS: usize = 8;

const PROGRESS_BAR_CHARS: usize = 32;

fn main() {
	let image: Arc<Mutex<RgbImage>> = Arc::new(Mutex::new(RgbImage::new(WIDTH as u32, HEIGHT as u32)));
	
	let raytracer = Raytracer { screen: Screen { width: WIDTH, height: HEIGHT }, max_depth: MAX_DEPTH, samples: SAMPLES_PER_PIXEL };
	
	// let world = random_scene();
	let mut world = World { objects: vec![
		Object::new(Box::new(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0)), Box::new(Lambertian { albedo: Color::all(0.5) })),
		Object::new(Box::new(Sphere::new(Vec3::ZERO, 0.5)), Box::new(Metal { albedo: Color::new(1.0, 0.25, 0.5), fuzz: 0.0 })),
		Object::new(Box::new(Sphere::new(Vec3::new_z(-1.0), 0.5)), Box::new(Metal { albedo: Color::new(0.25, 1.0, 0.5), fuzz: 0.125 })),
		Object::new(Box::new(Sphere::new(Vec3::new_z(1.0), 0.5)), Box::new(Metal { albedo: Color::new(0.5, 0.25, 1.0), fuzz: 0.25 })),
	]};
	for i in -8..=8 {
		for j in -8..=8 {
			world.objects.push(Object::new(
				Box::new(Sphere::new(Vec3::new(i as Float, 0.5 + (i as Float).sin() * (j as Float).cos(), j as Float) / 2.0, 0.2)),
				if util::random_float() < 0.8 {
					Box::new(Lambertian { albedo: (Vec3::ONE + util::random_color()) / 2.0 })
				} else {
					Box::new(Dielectric { refractive_index: 1.5 })
				}
			));
		}
	}
	
	let origin = Point3::new(13.0, 4.0, 3.0);
	let look_at = Point3::ZERO;
	let lens = CameraLens::new_from_dist(0.1, origin, look_at);
	let camera = Camera::new(origin, look_at, Vec3::new_y(1.0), 10.0, raytracer.screen.get_aspect_ratio(), Some(lens));
	
	let start_of_op = Instant::now();
	
	thread::scope(|s| {
		let world_ref = &world;
		
		for thread_no in 0..RENDER_THREADS {
			let image_ref = image.clone();
			
			s.spawn(move |_| {
				for y in (0..HEIGHT).rev() {
					for x in (0..WIDTH).skip(thread_no).step_by(RENDER_THREADS) {
						let pixel = raytracer.get_pixel(
							world_ref, &camera,
							x as Float / raytracer.screen.width as Float,
							1.0 - (y as Float / raytracer.screen.height as Float)
						);
						
						image_ref.lock().expect("well I failed.").put_pixel(x as u32, y as u32, Rgb(pixel.into()));
					}
				}
			});
		}
	}).expect("AAAAA GOD DANG TI");
	let duration = start_of_op.elapsed();
	
	// Note: the fancy \x1B[?C characters move the cursor forward ? characters.
	// This sadly doesn't work on Windows Terminal, but that's because it sucks.
	
	eprint!("\r\x1B[{}C Saving...", PROGRESS_BAR_CHARS);
	image.lock().expect("uhuhhh what").save("output.png").unwrap();
	
	eprintln!("\r\x1B[{}C Done. Took {:.2?}.", PROGRESS_BAR_CHARS, duration);
}

fn print_console_progress_bar(percent_done: Float) {
	let percent_done_int = (percent_done * PROGRESS_BAR_CHARS as Float).round() as usize;
	eprint!("\r{}{} {:.2}%", "\u{2588}".repeat(percent_done_int), "\u{2592}".repeat(PROGRESS_BAR_CHARS - percent_done_int), percent_done * 100.0);
	io::stderr().flush().expect("Could not flush stderr! That's weird.");
}

fn random_scene() -> World {
	let mut world = World { objects: Vec::new() };
	
	world.objects.push(Object::new(
		Box::new(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0)),
		Box::new(Lambertian { albedo: Color::all(0.5) })
	));
	
	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = util::random_float();
			let center = Point3::new(
				Float::mul_add(util::random_float(), 0.9, a as Float),
				0.2,
				Float::mul_add(util::random_float(), 0.9, b as Float)
			);
			let shape = Sphere::new(center, 0.2);
			
			if choose_mat < 0.8 {
				let albedo = util::random_color() * util::random_color();
				
				let material = Lambertian { albedo };
				world.objects.push(Object::new(Box::new(shape), Box::new(material)));
			} else if choose_mat < 0.95 {
				let albedo = (util::random_color() + Color::ONE) / 2.0;
				let fuzz = (util::random_float() + 1.0) / 2.0;
				
				let material = Metal { albedo, fuzz };
				world.objects.push(Object::new(Box::new(shape), Box::new(material)));
			} else {
				let material = Dielectric { refractive_index: 1.5 };
				world.objects.push(Object::new(Box::new(shape), Box::new(material)));
			}
		}
	}
	
	world.objects.push(Object::new(
		Box::new(Sphere::new(Point3::new_y(1.0), 1.0)),
		Box::new(Dielectric { refractive_index: 1.5 })
	));
	world.objects.push(Object::new(
		Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0)),
		Box::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) })
	));
	world.objects.push(Object::new(
		Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0)),
		Box::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 })
	));
	
	world
}
