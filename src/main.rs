use std::rc::Rc;

use camera::Camera;
use color::Color;
use material::{Dielectric, Lambertian, Metal};
use vec3::Vec3;

use crate::{hittable_list::HittableList, sphere::Sphere, vec3::Point3};

pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
mod hittable_list;
pub mod interval;
pub mod camera;
pub mod material;

fn main() {
    let ground_mat = Lambertian {albedo: Color::new(0.8,0.8,0.0)};
    let center_mat = Lambertian {albedo: Color::new(0.1, 0.2, 0.5)};
    let left_mat = Dielectric { refraction_index: 1.5};
    let bubble_mat = Dielectric { refraction_index: 1.0 / 1.5};
    let right_mat = Metal {albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0};
    
    let mut world = HittableList::default();

    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::new(ground_mat));
    let center = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Rc::new(center_mat));
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(left_mat));
    let bubble = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, Rc::new(bubble_mat));
    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Rc::new(right_mat));

    world.add(Rc::new(ground));
    world.add(Rc::new(center));
    world.add(Rc::new(left));
    world.add(Rc::new(bubble));
    world.add(Rc::new(right));

    let mut cam = Camera::new(16.0/9.0, 400);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.look_from = Point3::new(-2.0, 2.0, 1.0);
    cam.look_at = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.fov = 20.0;
    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
