use crate::vec3::{Point3, Vec3};

struct Ray {
	origin: Point3,
	dir: Vec3,
}
impl Ray {
	pub fn new(origin: Point3, dir: Vec3) -> Self {
		Ray {origin, dir}
	}
	pub fn at(&self, t: f32) -> Point3 {
		self.origin + t*self.dir
	}
	pub fn origin(&self) -> &Point3 {
		&self.origin
	}
	pub fn dir(&self) -> &Vec3 {
		&self.dir
	}
}