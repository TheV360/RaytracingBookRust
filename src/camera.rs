use crate::vector::{Vec2, Vec3, Point3, Float};
use crate::ray::Ray;
use crate::util;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct CameraLens {
	pub aperture: Float,
	pub focus_dist: Float,
}
impl CameraLens {
	pub const fn new(aperture: Float, focus_dist: Float) -> Self {
		Self { aperture, focus_dist }
	}
	
	pub fn new_from_dist(aperture: Float, origin: Point3, look_at: Point3) -> Self {
		let focus_dist = Vec3::magnitude(origin - look_at);
		Self { aperture, focus_dist }
	}
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
/// The `Camera` is an object that can shoot rays out at a scene.
pub struct Camera {
	pub origin: Point3,
	pub lower_left_corner: Point3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub u: Vec3, pub v: Vec3, pub w: Vec3,
	pub camera_lens: Option<CameraLens>,
}
impl Camera {
	/// Sets up the camera. FOV should be in degrees.
	pub fn new(origin: Point3, look_at: Point3, up: Vec3, fov: Float, aspect_ratio: Float, camera_lens: Option<CameraLens>) -> Self {
		let h = (fov.to_radians() / 2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;
		
		let w = Vec3::normalize(origin - look_at);
		let u = Vec3::normalize(Vec3::cross(up, w));
		let v = Vec3::cross(w, u);
		
		if let Some(lens) = camera_lens {
			let horizontal = lens.focus_dist * viewport_width * u;
			let vertical = lens.focus_dist * viewport_height * v;
			let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - lens.focus_dist * w;
			
			Self { origin, lower_left_corner, horizontal, vertical, u, v, w, camera_lens }
		} else {
			let horizontal = viewport_width * u;
			let vertical = viewport_height * v;
			let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;
			
			Self { origin, lower_left_corner, horizontal, vertical, u, v, w, camera_lens }
		}
	}
	
	/// Gets a normalized ray from the Camera's view.
	pub fn get_ray(&self, st: Vec2) -> Ray {
		let (s, t) = st.into();
		
		if let Some(lens) = self.camera_lens {
			let rd = lens.aperture / 2.0 * util::random_in_unit_disk();
			let offset = self.u * rd.x + self.v * rd.y;
			
			Ray::new(
				self.origin + offset, 
				(
					self.lower_left_corner +
					s * self.horizontal +
					t * self.vertical
					- self.origin - offset
				).normalize()
			)
		} else {
			Ray::new(
				self.origin, 
				(
					self.lower_left_corner +
					s * self.horizontal +
					t * self.vertical
					- self.origin
				).normalize()
			)
		}
	}
}
