use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{random_unit_vector, reflect, Vec3}};

pub trait Material: Default {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Default)]
pub struct Lambertian { albedo: Color }
impl Material for Lambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let mut scatter_dir = rec.normal + random_unit_vector();

		// Catch degenerate scatter direction
		if scatter_dir.near_zero() {
			scatter_dir = rec.normal
		}

		*scattered = Ray::new(rec.p, scatter_dir);
		*attenuation = self.albedo.clone();
		true
	}
}

#[derive(Default)]
pub struct Metal { albedo: Color }
impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected = reflect(r_in.dir(), &rec.normal);
		*scattered = Ray::new(rec.p, reflected);
		*attenuation = self.albedo;
		true
	}
}
