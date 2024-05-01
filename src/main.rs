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
    let ground = material::EMaterial::Lambertian(Lambertian {albedo: Color::new(0.8,0.8,0.0)});
    let center = material::EMaterial::Lambertian(Lambertian {albedo: Color::new(0.1, 0.2, 0.5)});
    let left = material::EMaterial::Metal(Metal {albedo: Color::new(0.8, 0.8, 0.8)});
    let right = material::EMaterial::Metal(Metal {albedo: Color::new(0.8, 0.6, 0.2)});
    
    let mut world = HittableList::new();

    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(ground));
    let center = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Box::new(center));
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Box::new(left));
    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Box::new(right));

    world.add(Box::new(ground));
    world.add(Box::new(center));
    world.add(Box::new(left));
    world.add(Box::new(right));

    let mut cam = Camera::new(16.0/9.0, 400);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
