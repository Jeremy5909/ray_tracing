use std::{f32::INFINITY, io::stdout};

use rand::{thread_rng, Rng};

use crate::{color::{write_color, Color}, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, vec3::{random_on_hemisphere, unit_vector, Point3, Vec3}};

#[derive(Default)]
pub struct Camera {
	pub aspect_ratio: f32,
	pub image_width: i32,
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3,

	pub sample_per_pixel: i32,
	pixel_samples_scale: f32,
}
impl Camera {
	pub fn new(aspect_ratio: f32, image_width: i32) -> Self {
		Self {aspect_ratio, image_width,sample_per_pixel: 10, ..Default::default()}
	}
	pub fn render(&mut self, world: &dyn Hittable) {
		self.initialize();

		println!("P3\n{} {}\n255", self.image_width, self.image_height);
    	for j in 0..=self.image_height {
        	eprint!("\rScanlines remaining: {} ", self.image_height-j);
        	for i in 0..self.image_height {
            	let mut pixel_color = Color::new();
				for _ in 0..self.sample_per_pixel {
					let r = self.get_ray(i, j);
					pixel_color += self.ray_color(&r, world);
				}
            	write_color(&mut stdout(), self.pixel_samples_scale * pixel_color);
        }
    }
    eprintln!("\rDone.                 \n");
	}
	fn initialize(&mut self) {
		//BUG when proportion != 1.0
		self.image_height = (self.image_width as f32/self.aspect_ratio) as i32;
		self.image_height = if self.image_height < 1 {1} else {self.image_height};

		self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f32;

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
	fn ray_color(&mut self, r: &Ray, world: &dyn Hittable) -> Color {
		let mut rec = HitRecord::new();
		if world.hit(r, Interval::from(0.0, INFINITY), &mut rec) {
			return 0.5 * self.ray_color(&Ray::new(rec.p, random_on_hemisphere(rec.normal)), world);
		}
		let unit_direction = unit_vector(r.dir());
    	let a = 0.5*(unit_direction.y() + 1.0);
    	(1.0-a)*Color::from(1.0, 1.0, 1.0) + a*Color::from(0.5, 0.7, 1.0)
	}
	fn get_ray(&mut self, i: i32, j: i32) -> Ray {
		let offset = Self::sample_square();
		let pixel_sample = self.pixel00_loc
			+ ((i as f32 + offset.x()) * self.pixel_delta_u)
			+ ((j as f32 + offset.y()) * self.pixel_delta_v);
		let ray_origin = self.center;
		let ray_direction = pixel_sample - ray_origin;
		Ray::new(ray_origin, ray_direction)
	}
	fn sample_square() -> Vec3 {
		let mut rng = thread_rng();
		Vec3::from(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
	}
}
