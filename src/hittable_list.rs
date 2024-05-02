use crate::{hittable::{HitRecord, Hittable}, interval::Interval};
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut closest_so_far = ray_t.max;

        let hit_anything = self.objects.iter().any(|object| {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), rec) {
                closest_so_far = rec.t;
                true
            } else {
                false
            }
        });
        hit_anything
    }
}
