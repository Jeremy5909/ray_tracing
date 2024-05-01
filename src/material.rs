use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{random_unit_vector, reflect}};

pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
pub struct DefaultMaterial;
impl Material for DefaultMaterial {
	fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Color, _: &mut Ray) -> bool {
		false
	}
}

pub struct Lambertian { pub albedo: Color }
impl Material for Lambertian {
	fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let mut scatter_dir = rec.normal + random_unit_vector();

		// Catch degenerate scatter direction
		if scatter_dir.near_zero() {
			scatter_dir = rec.normal
		}

		*scattered = Ray::new(rec.p, scatter_dir);
		*attenuation = self.albedo;
		true
	}
}

pub struct Metal { pub albedo: Color }
impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected = reflect(r_in.dir(), &rec.normal);
		*scattered = Ray::new(rec.p, reflected);
		*attenuation = self.albedo;
		true
	}
}
