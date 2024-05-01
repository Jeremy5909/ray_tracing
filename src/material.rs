use std::default;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{random_unit_vector, reflect, Vec3}};

#[derive(Clone, Copy, Default)]
pub enum EMaterial {
	Lambertian(Lambertian),
	Metal(Metal),
	#[default]
	DefaultMaterial
}

impl Material for EMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            EMaterial::Lambertian(mat) => mat.scatter(r_in, rec, attenuation, scattered),
            EMaterial::Metal(mat) => mat.scatter(r_in, rec, attenuation, scattered),
            EMaterial::DefaultMaterial => false,
			// Implement scatter method for other variants
        }
    }
}
pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
#[derive(Default, Clone, Copy)]
pub struct Lambertian { pub albedo: Color }
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

#[derive(Default, Clone, Copy)]
pub struct Metal { pub albedo: Color }
impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected = reflect(r_in.dir(), &rec.normal);
		*scattered = Ray::new(rec.p, reflected);
		*attenuation = self.albedo;
		true
	}
}
