use camera::Camera;

use crate::{hittable_list::HittableList, sphere::Sphere, vec3::Point3};

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

    let mut cam = Camera::new(16.0/9.0, 400);
    cam.sample_per_pixel = 100;

    cam.render(&world);
}
