use std::{f32::INFINITY, io::{stdout, Write}};

use camera::Camera;
use hittable::{HitRecord, Hittable};
use interval::Interval;
use vec3::{dot, unit_vector};

use crate::{color::{write_color, Color}, hittable_list::HittableList, ray::Ray, sphere::Sphere, vec3::{Point3, Vec3}};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::from(Sphere::new(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::from(Sphere::new(Point3::from(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(1.0, 400);

    cam.render(&world);
}
