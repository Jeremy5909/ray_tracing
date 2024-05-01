use camera::Camera;
use material::Lambertian;

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
    let mat1 = Lambertian::default();
    let mut world = HittableList::new();

    let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(mat1));
    let sphere2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(mat1));

    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));

    let mut cam = Camera::new(16.0/9.0, 400);
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
