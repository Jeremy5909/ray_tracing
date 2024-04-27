use std::{f32::INFINITY, io::{stdout, Write}};

use crate::{color::{write_color, Color}, hittable::{self, HitRecord, Hittable}, interval::Interval, ray::Ray, vec3::{unit_vector, Point3, Vec3}};

pub struct Camera {
	pub aspect_ratio: f32,
	pub image_width: i32,
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3
}
impl Camera {
	pub fn new(aspect_ratio: f32, image_width: i32) -> Self {
		Self {
			aspect_ratio,
			image_width,
			image_height: 0,
			center: Point3::new(),
			pixel00_loc: Point3::new(),
			pixel_delta_u: Vec3::new(),
			pixel_delta_v: Vec3::new()}
	}
	pub fn render(&mut self, world: &dyn Hittable) {
		self.initialize();

		println!("P3\n{} {}\n255", self.image_width, self.image_height);
    	for j in 0..=self.image_height {
        	eprintln!("Scanlines remaining: {}", self.image_height-j);
        	stdout().flush().unwrap();
        	for i in 0..self.image_height {
            	let pixel_center = self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
            	let ray_direction = pixel_center - self.center;
            	let r = Ray::new(self.center, ray_direction);

            	let pixel_color = Self::ray_color(&r, world);
            	write_color(&mut stdout(), pixel_color);
        }
    }
    eprintln!("Done");
	}
	fn initialize(&mut self) {
		self.image_height = (self.image_width as f32/self.aspect_ratio) as i32;
		self.image_height = if self.image_height < 1 {1} else {self.image_height};

		self.center = Point3::from(0.0, 0.0, 0.0);
		let focal_length = 1.0f32;
		let viewport_height = 2.0f32;
		let viewport_width = viewport_height * ((self.image_width/self.image_height) as f32);

		let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
		let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

		self.pixel_delta_u = viewport_u / self.image_width as f32;
		self.pixel_delta_v = viewport_v / self.image_height as f32;

		let viewport_upper_left = self.center - Vec3::from(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
		self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
	}
	fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
		let mut rec = HitRecord::new();
		if world.hit(r, Interval::from(0.0, INFINITY), &mut rec) {
			return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
		}
		let unit_direction = unit_vector(r.dir());
    	let a = 0.5*(unit_direction.y() + 1.0);
    	(1.0-a)*Color::from(1.0, 1.0, 1.0) + a*Color::from(0.5, 0.7, 1.0)
	}
}