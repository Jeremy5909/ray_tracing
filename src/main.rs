use camera::Camera;
use color::Color;
use material::{Dielectric, Lambertian, Metal};

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
    let ground_mat = Lambertian {albedo: Color::new(0.8,0.8,0.0)};
    let center_mat = Lambertian {albedo: Color::new(0.1, 0.2, 0.5)};
    let left_mat = Dielectric { refraction_index: 1.5};
    let bubble_mat = Dielectric { refraction_index: 1.0 / 1.5};
    let right_mat = Metal {albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0};
    
    let mut world = HittableList::new();

    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &ground_mat);
    let center = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, &center_mat);
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, &left_mat);
    let bubble = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, &bubble_mat);
    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, &right_mat);

    world.add(&ground);
    world.add(&center);
    world.add(&left);
    world.add(&bubble);
    world.add(&right);

    let mut cam = Camera::new(16.0/9.0, 400);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
