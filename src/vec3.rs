use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, RangeBounds, Sub}};
use rand::{distributions::uniform::SampleRange, random, thread_rng, Rng};

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
	pub e: [f64; 3]
}
impl Vec3 {
	pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
		Self{e: [e0, e1, e2]}
	}

	pub fn x(&self) -> f64 {self.e[0]}
	pub fn y(&self) -> f64 {self.e[1]}
	pub fn z(&self) -> f64 {self.e[2]}

	pub fn length(&self) -> f64 {self.length_squared().sqrt()}
	pub fn length_squared(&self) -> f64 {self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]}

	pub fn random() -> Self {Vec3::new(random(), random(), random())}
	pub fn random_range<T: RangeBounds<f64> + Clone + SampleRange<f64>>(range: T) -> Self {
		let mut random = thread_rng();
		Vec3::new(random.gen_range(range.clone()), random.gen_range(range.clone()), random.gen_range(range.clone()))
	}
	pub fn near_zero(&self) -> bool {
		let s = 1e-8;
		self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
	}
}
pub type Point3 = Vec3;
impl Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Self::Output {
		Vec3::new(-self.e[0],-self.e[1],-self.e[2])
	}
}
impl Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, index: usize) -> &Self::Output {
		&self.e[index]
	}
}
impl IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.e[index]
	}
}
impl Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Self) -> Self::Output {
		Vec3::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
	}
}
impl Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Vec3) -> Self::Output {
		Vec3::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
	}
}
impl AddAssign<Vec3> for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		self.e[0] += rhs.e[0];
		self.e[1] += rhs.e[1];
		self.e[2] += rhs.e[2];
	}
}
impl Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: Self) -> Self::Output {
		Vec3::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
	}
}
impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
	}
}
impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		Vec3::new(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
	}
}
impl MulAssign<Vec3> for Vec3 {
	fn mul_assign(&mut self, rhs: Self) {
		self.e[0] *= rhs.e[0];
		self.e[1] *= rhs.e[1];
		self.e[2] *= rhs.e[2];
	}
}
impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, rhs: f64) {
		self.e[0] *= rhs;
		self.e[1] *= rhs;
		self.e[2] *= rhs;
	}
}
impl Div<Vec3> for Vec3 {
	type Output = Vec3;

	fn div(self, rhs: Self) -> Self::Output {
		Vec3::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
	}
}
impl Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, rhs: f64) -> Self::Output {
		1.0/rhs * self
	}
}
impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, rhs: f64) {
		*self *= 1.0/rhs
	}
}
impl Display for Vec3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
	}
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
	*v / v.length()
}
pub fn dot(first: Vec3, other: Vec3) -> f64 {
	first.e[0] * other.e[0] + 
	first.e[1] * other.e[1] +
	first.e[2] * other.e[2]
}
pub fn cross(first: Vec3, other: Vec3) -> Vec3 {
	Vec3::new(
		first.e[1]*other.e[2] - first.e[2]*other.e[1],
		first.e[2]*other.e[0] - first.e[0]*other.e[2],
		first.e[0]*other.e[1] - first.e[1]*other.e[0]
	)
}
pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::random_range(-1.0..1.0);
		if p.length_squared() < 1.0 {
			return p;
		}
	}
}

pub fn random_unit_vector() -> Vec3 {
	unit_vector(&random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
	let on_unit_sphere = random_unit_vector();
	if dot(on_unit_sphere, normal) > 0.0 {
		on_unit_sphere
	} else {
		-on_unit_sphere
	}
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
	*v - 2.0 * dot(*n, *v) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = dot(- *uv, *n).min(1.0);
	let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
	let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
	r_out_parallel + r_out_perp
}

pub fn random_in_unit_disk() -> Vec3 {
	let mut rng = thread_rng();
	loop {
		let p = Vec3::new(rng.gen_range(-1.0 .. 1.0), rng.gen_range(-1.0 .. 1.0), 0.0);
		if p.length_squared() < 1.0 {
			return p;
		}
	}
}