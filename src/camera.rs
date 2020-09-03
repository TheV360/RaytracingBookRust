use crate::vector::{Vec3, Point3, Float};
use crate::ray::Ray;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
/// The `Camera` is an object that can shoot rays out at a scene.
pub struct Camera {
	pub origin: Point3,
	pub lower_left_corner: Point3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
}
impl Camera {
	/// Sets up the camera. Pretty self-explanatory.
	pub fn new(origin: Vec3, aspect_ratio: Float, viewport_height: Float, focal_length: Float) -> Self {
		let viewport_width = aspect_ratio * viewport_height;
		
		let horizontal = Vec3::new_x(viewport_width);
		let vertical = Vec3::new_y(viewport_height);
		let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new_z(focal_length);
		
		Self { origin, lower_left_corner, horizontal, vertical, }
	}
	
	/// Gets a normalized ray from the Camera's view.
	pub fn get_ray(&self, u: Float, v: Float) -> Ray {
		Ray::new(
			self.origin, 
			(
				self.lower_left_corner +
				u * self.horizontal +
				v * self.vertical
				- self.origin
			).normalize()
		)
	}
}
