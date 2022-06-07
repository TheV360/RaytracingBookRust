use std::sync::Arc;
use std::time::Instant;
use std::thread;

use std::sync::mpsc::channel;

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

const RENDER_THREADS: usize = 8;

fn main() {
	let raytracer = Raytracer { screen: Screen { width: 1920, height: 1080 }, max_depth: 16, samples: 48 };
	
	let mut image = RgbImage::new(raytracer.screen.width as u32, raytracer.screen.height as u32);
	
	let world = basic_scene();
	// let world = random_scene();
	
	// Put world inside an Arc, to share it with threads.
	// Don't need any mutation, it'll all be nice and fast.
	let world = Arc::new(world);
	
	// Construct our camera.
	let origin = Point3::new(13.0, 4.0, 3.0);
	let look_at = Point3::ZERO;
	let lens = CameraLens::new_from_dist(0.1, origin, look_at);
	let camera = Camera::new(origin, look_at, Vec3::new_y(1.0), 10.0, raytracer.screen.get_aspect_ratio(), Some(lens));
	
	let start_of_op = Instant::now();
	
	let (tx, rx) = channel();
	
	for thread_no in 0..RENDER_THREADS {
		let t_world = world.clone();
		let tx = tx.clone();
		
		thread::spawn(move || {
			for y in (0..raytracer.screen.height).rev().skip(thread_no).step_by(RENDER_THREADS) {
				let yr = 1.0 - (y as Float / raytracer.screen.height as Float);
				
				for x in 0..raytracer.screen.width {
					let xr = x as Float / raytracer.screen.width as Float;
					
					let pixel = raytracer.get_pixel(&t_world, &camera, xr, yr);
					tx.send((x, y, pixel)).unwrap();
				}
			}
		});
	}
	
	// Not intending on making any other threads.
	drop(tx);
	
	// Haphazardly-implemented "progress bar"
	let mut pixels_plotted = 0usize;
	let total_pixels = raytracer.screen.width * raytracer.screen.height;
	
	// Draw the picture
	while let Ok((x, y, pixel)) = rx.recv() {
		image.put_pixel(x as u32, y as u32, Rgb(pixel.into()));
		
		pixels_plotted += 1;
		if pixels_plotted & 0xFFF == 0 { // (every so often)
			eprint!("\r{:6.2}% done.", (pixels_plotted as f64 / total_pixels as f64) * 100.0);
		}
	}
	
	let duration = start_of_op.elapsed();
	eprintln!("\r100.00% done. Took {duration:.2?}.");
	
	image.save("output.png").expect("couldn't save...");
}

#[allow(dead_code)]
fn basic_scene() -> World {
	let mut world = World { objects: Vec::new(), ..Default::default() };
	
	// Like in real life, everything sits on a really big sphere.
	world.objects.push(Object::new(
		Box::new(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0)),
		Box::new(Lambertian { albedo: Color::all(0.5) }))
	);
	
	// Scatter 81 balls around, for fun.
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
	
	world.objects.push(Object::new(
		Box::new(Sphere::new(Vec3::new_z(-1.0), 0.5)),
		Box::new(Metal { albedo: Color::new(1.0, 0.25, 0.5), fuzz: 0.125 })
	));
	world.objects.push(Object::new(
		Box::new(Sphere::new(Vec3::ZERO, 0.5)),
		Box::new(Metal { albedo: Color::new(0.25, 1.0, 0.5), fuzz: 0.0 })
	));
	world.objects.push(Object::new(
		Box::new(Sphere::new(Vec3::new_z(1.0), 0.5)),
		Box::new(Metal { albedo: Color::new(0.5, 0.25, 1.0), fuzz: 0.25 })
	));
	
	world
}

#[allow(dead_code)]
fn random_scene() -> World {
	let mut world = World { objects: Vec::new(), ..Default::default() };
	
	world.objects.push(Object::new(
		Box::new(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0)),
		Box::new(Lambertian { albedo: Color::all(0.5) })
	));
	
	for a in -11..=11 {
		for b in -11..=11 {
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
