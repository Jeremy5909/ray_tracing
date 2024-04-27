use crate::{hittable::{HitRecord, Hittable}, interval::Interval, vec3::{dot, Point3, Vec3}};

pub struct Sphere {
	center: Point3,
	radius: f32
}
impl Sphere {
	pub fn new(center: Point3, radius: f32) -> Self {
		Sphere {center, radius}
	}
}
impl Hittable for Sphere {
	fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let oc = self.center - *r.origin();
		let a = r.dir().length_squared();
		let h = dot(*r.dir(), oc);
		let c = oc.length_squared() - self.radius * self.radius;

		let discriminant = h*h - a*c;
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
		let outward_normal = (rec.p - self.center) / self.radius;
		rec.set_face_normal(r, &outward_normal);

		true
	}
}