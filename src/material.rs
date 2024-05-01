use rand::random;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{dot, random_unit_vector, reflect, refract, unit_vector}};

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

pub struct Metal { pub albedo: Color, pub fuzz: f64 }
impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected = reflect(r_in.dir(), &rec.normal);
		let reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
		*scattered = Ray::new(rec.p, reflected);
		*attenuation = self.albedo;
		true
	}
}

pub struct Dielectric { pub refraction_index: f64 }
impl Dielectric {
	fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
		let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
		let r0 = r0 * r0;
		r0 + (1.0-r0) * (1.0 - cosine).powi(5)

	}
}
impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		*attenuation = Color::new(1.0, 1.0, 1.0);
		let ri = if rec.front_face {1.0/self.refraction_index} else {self.refraction_index};

		let unit_dir = unit_vector(r_in.dir());
		let cos_theta = dot(-unit_dir, rec.normal).min(1.0);
		let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

		let direction = if ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > random() {
			reflect(&unit_dir, &rec.normal)
		} else {
			refract(&unit_dir, &rec.normal, ri)
		};

		*scattered = Ray::new(rec.p, direction);
		true
	}
}