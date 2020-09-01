use std::time::{Instant};
use std::io::{self, Write};

use image::{RgbImage, Rgb};

//////////////////

mod vector;
mod ray;

use vector::*;
use ray::*;

//////////////////

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

type FrameBufferInner = [[Color; WIDTH]; HEIGHT];
type FrameBuffer = Box<FrameBufferInner>;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct Sphere {
	pub center: Vec3,
	pub radius: Float,
}
impl Sphere {
	// fn ray_intersect(self, ray: Ray) -> bool {
	// 	false
	// }
}

fn main() {
	// Beautiful elegant syntax...
	// let mut framebuffer: FrameBuffer = box [[Vec3::ZERO; WIDTH]; HEIGHT];
	
	// What I have to use for this to work in stable.
	// I stole this. I am not smart enough to figure this out on my own.
	// https://github.com/rust-lang/rust/issues/49733#issuecomment-621666613
	let mut framebuffer: FrameBuffer = {
		use std::alloc::{alloc_zeroed, Layout, handle_alloc_error};
		let layout = Layout::new::<FrameBufferInner>();
		let ptr = unsafe { alloc_zeroed(layout) };
		if ptr.is_null() {
			handle_alloc_error(layout);
		}
		unsafe { Box::from_raw(ptr as *mut FrameBufferInner) }
	};
	// It doesn't even directly initialize any values. >:(
	
	let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);
	let start_of_op = Instant::now();
	
	for y in (0..HEIGHT).rev() {
		eprint!("\rScanlines remaining: {}...", (HEIGHT - y));
		io::stderr().flush().ok().expect("fuck whoops");
		
		for x in 0..WIDTH {
			framebuffer[y][x] = Color { x: (x as Float) / (WIDTH as Float), y: 1.0 - ((y as Float) / (HEIGHT as Float)), z: 0.25 };
			image.put_pixel(x as u32, y as u32, Rgb(framebuffer[y][x].into()));
		}
	}
	eprintln!("\rScanlines remaining: [DONE]!!!");
	
	image.save("output.png").unwrap();
	
	let duration = start_of_op.elapsed();
	eprintln!("The operation took {:?}.", duration);
}
