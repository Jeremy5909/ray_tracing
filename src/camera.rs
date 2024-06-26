use std::{f64::INFINITY, io::{self, stdout, BufWriter}};

use rand::{random, thread_rng, Rng};

use crate::{color::{write_color, Color}, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3}};
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

	pub fov: f64,

	pub look_from: Point3,
	pub look_at: Point3,
	// Relative up
	pub vup: Vec3,
	// Camera frame basis vector
	u: Vec3,
	v: Vec3,
	w: Vec3,

	pub defocus_angle: f64,
	pub focus_dist: f64,
	defocus_dist_u: Vec3,
	defocus_dist_v: Vec3,
}
impl Camera {
	pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
		Self {
			aspect_ratio,
			image_width,
			samples_per_pixel: 10,
			fov: 90.0,
			look_at: Point3::new(0.0, 0.0, -1.0),
			vup: Vec3::new(0.0, 1.0, 0.0),
			focus_dist: 10.0,
			..Default::default()}
	}
	pub fn render(&mut self, world: &dyn Hittable) {
		self.initialize();
        let mut writer = BufWriter::new(io::stdout());

		println!("P3\n{} {}\n255", self.image_width, self.image_height);
    	for j in 0..=self.image_height {
        	eprint!("\rScanlines remaining: {} ", self.image_height-j);
        	for i in 0..self.image_width {
            	let mut pixel_color = Color::default();
				for _ in 0..self.samples_per_pixel {
					let r = self.get_ray(i, j);
					pixel_color += Self::ray_color(&r, self.max_depth, world);
				}
            	write_color(&mut writer, self.pixel_samples_scale * pixel_color);
        }
    }
    eprintln!("\rDone.                 \n");
	}
	fn initialize(&mut self) {
		self.image_height = ((self.image_width as f64/self.aspect_ratio) as i32).max(0);

		self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

		self.center = self.look_from;
		let theta = self.fov.to_radians();
		let h = (theta/2.0).tan();		
		let viewport_height = 2.0 * h * self.focus_dist;
		let viewport_width = viewport_height * (self.image_width as f64/self.image_height as f64);

		self.w = unit_vector(&(self.look_from-self.look_at));
		self.u = unit_vector(&cross(self.vup, self.w));
		self.v = cross(self.w, self.u);

		let viewport_u = viewport_width * self.u;
		let viewport_v = viewport_height * -self.v;

		self.pixel_delta_u = viewport_u / self.image_width as f64;
		self.pixel_delta_v = viewport_v / self.image_height as f64;

		let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u/2.0 - viewport_v/2.0;
		self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

		let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
		self.defocus_dist_u = self.u * defocus_radius;
		self.defocus_dist_v = self.v * defocus_radius;
	}
	fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
		if depth <= 0 {
			return Color::default();
		}
		let mut rec = HitRecord::default();
		if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
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
		let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
		let ray_direction = pixel_sample - ray_origin;
		let ray_time: f64 = random();

		Ray::new(ray_origin, ray_direction, ray_time)
	}
	fn sample_square() -> Vec3 {
		let mut rng = thread_rng();
		Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
	}
	fn defocus_disk_sample(&self) -> Point3 {
		let p = random_in_unit_disk();
		self.center + (p[0] * self.defocus_dist_u) + (p[1] * self.defocus_dist_v)
	}
}
