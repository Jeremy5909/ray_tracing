use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::Material, vec3::{dot, Point3, Vec3}};
use std::rc::Rc;

pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    // Consider something with enums
    pub fn stationary(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere { center1: center, radius: radius.max(0.0), mat, is_moving: false, center_vec: Default::default() }
    }

    pub fn moving(center1: Point3, center2: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere { center1, radius: radius.max(0.0), mat, is_moving: true, center_vec: center2-center1 }
    }

    fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + time*self.center_vec
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving { self.sphere_center(r.time()) } else { self.center1 };

        let oc = center - *r.origin();
        let a = r.dir().length_squared();
        let h = dot(*r.dir(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find nearest root that lies in acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}
