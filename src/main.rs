use std::rc::Rc;

use camera::Camera;
use color::Color;
use material::{Dielectric, Lambertian, Metal};
use rand::{random, thread_rng, Rng};
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
    let mut world = HittableList::default();

    let ground_mat = Lambertian {albedo: Color::new(0.5,0.5,0.5)};
    world.add(Rc::new(Sphere::stationary(Point3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(ground_mat))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Point3::new(a as f64 + 0.9*random::<f64>(), 0.2, b as f64+ 0.9*random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian{ albedo };
                    let center2 = center + Vec3::new(0.0, thread_rng().gen_range(0.0..0.5), 0.0);
                    world.add(Rc::new(Sphere::moving(center, center2, 0.2, Rc::new(sphere_material))));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random() * Color::random();
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    let sphere_material = Metal { albedo, fuzz };
                    world.add(Rc::new(Sphere::stationary(center, 0.2, Rc::new(sphere_material))));
                } else {
                    let sphere_material = Dielectric { refraction_index: 1.5};
                    world.add(Rc::new(Sphere::stationary(center, 0.2, Rc::new(sphere_material))));
                }
            }
        }
    }

    let material_one = Dielectric { refraction_index: 1.5};
    world.add(Rc::new(Sphere::stationary(Point3::new(0.0, 1.0, 0.0), 1.0, Rc::new(material_one))));
    let material_two = Lambertian { albedo: Color::new(0.4, 0.2, 0.1) };
    world.add(Rc::new(Sphere::stationary(Point3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(material_two))));
    let material_three = Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0};
    world.add(Rc::new(Sphere::stationary(Point3::new(4.0, 1.0, 0.0), 1.0, Rc::new(material_three))));


    
    let mut cam = Camera::new(16.0/9.0, 300);
    cam.samples_per_pixel = 40;
    cam.max_depth = 10;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.fov = 20.0;
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
