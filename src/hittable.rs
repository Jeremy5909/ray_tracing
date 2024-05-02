use std::rc::Rc;

use crate::{interval::Interval, material::{DefaultMaterial, Material}, ray::Ray, vec3::{dot, Point3, Vec3}};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self { 
            p: Default::default(), 
            normal: Default::default(), 
            mat: Rc::new(DefaultMaterial), 
            t: Default::default(), 
            front_face: Default::default() 
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(*r.dir(), *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
