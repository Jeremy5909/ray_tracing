use std::{f64::INFINITY, io::stdout};

use rand::{thread_rng, Rng};

use crate::{color::{write_color, Color}, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, vec3::{unit_vector, Point3, Vec3}};
use crate::material::Material;
#[derive(Default)]
pub struct Camera {
	pub aspect_ratio: f64,
	pub image_width: i32,
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3,

	pub samples_per_pixel: i32,
	pixel_samples_scale: f64,

	pub max_depth: i32,
}
impl Camera {
	pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
		Self {aspect_ratio, image_width,samples_per_pixel: 10, ..Default::default()}
	}
	pub fn render(&mut self, world: &dyn Hittable) {
		self.initialize();

		println!("P3\n{} {}\n255", self.image_width, self.image_height);
    	for j in 0..=self.image_height {
        	eprint!("\rScanlines remaining: {} ", self.image_height-j);
        	for i in 0..self.image_width {
            	let mut pixel_color = Color::default();
				for _ in 0..self.samples_per_pixel {
					let r = self.get_ray(i, j);
					pixel_color += Self::ray_color(&r, self.max_depth, world);
				}
            	write_color(&mut stdout(), self.pixel_samples_scale * pixel_color);
        }
    }
    eprintln!("\rDone.                 \n");
	}
	fn initialize(&mut self) {
		self.image_height = (self.image_width as f64/self.aspect_ratio) as i32;
		self.image_height = if self.image_height < 1 {1} else {self.image_height};

		self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

		self.center = Point3::new(0.0, 0.0, 0.0);
		let focal_length = 1.0;
		let viewport_height = 2.0;
		let viewport_width = viewport_height * (self.image_width as f64/self.image_height as f64);

		let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
		let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

		self.pixel_delta_u = viewport_u / self.image_width as f64;
		self.pixel_delta_v = viewport_v / self.image_height as f64;

		let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
		self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
	}
	fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
		if depth <= 0 {
			return Color::default();
		}
		let mut rec = HitRecord::default();
		if world.hit(r, Interval::from(0.001, INFINITY), &mut rec) {
			let mut scattered = Ray::default();
			let mut attenuation = Color::default();
			if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
				return attenuation * Self::ray_color(&scattered, depth-1, world);
			}
			return Color::default();

		}
		let unit_direction = unit_vector(r.dir());
    	let a = 0.5*(unit_direction.y() + 1.0);
    	(1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
	}
	fn get_ray(&mut self, i: i32, j: i32) -> Ray {
		let offset = Self::sample_square();
		let pixel_sample = self.pixel00_loc
			+ ((i as f64 + offset.x()) * self.pixel_delta_u)
			+ ((j as f64 + offset.y()) * self.pixel_delta_v);
		let ray_origin = self.center;
		let ray_direction = pixel_sample - ray_origin;
		Ray::new(ray_origin, ray_direction)
	}
	fn sample_square() -> Vec3 {
		let mut rng = thread_rng();
		Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
	}
}
