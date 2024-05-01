use crate::{hittable::{HitRecord, Hittable}, interval::Interval};

pub struct HittableList<'a> {
	objects: Vec<&'a dyn Hittable>,
}
impl<'a> HittableList<'a> {
	pub fn new() -> Self {
		HittableList{objects: Vec::new()}
	}
	pub fn add(&mut self, object: &'a dyn Hittable) {
		self.objects.push(object);
	}
}
impl<'a> Hittable for HittableList<'a> {
	fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::default();
		let mut hit_anything = false;
		let mut closest_so_far = ray_t.max;

		for object in &self.objects {
			if object.hit(r, Interval::from(ray_t.min, closest_so_far), &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.clone().t;
				*rec = temp_rec.clone();
			}
		}
		hit_anything
	}
}