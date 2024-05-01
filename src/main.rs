use camera::Camera;
use color::Color;
use material::{Lambertian, Metal};

use crate::{hittable_list::HittableList, sphere::Sphere, vec3::Point3};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod material;

fn main() {
    let ground = Lambertian {albedo: Color::new(0.8,0.8,0.0)};
    let center = Lambertian {albedo: Color::new(0.1, 0.2, 0.5)};
    let left = Metal {albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.3};
    let right = Metal {albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0};
    
    let mut world = HittableList::new();

    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &ground);
    let center = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, &center);
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, &left);
    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, &right);

    world.add(&ground);
    world.add(&center);
    world.add(&left);
    world.add(&right);

    let mut cam = Camera::new(16.0/9.0, 400);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
