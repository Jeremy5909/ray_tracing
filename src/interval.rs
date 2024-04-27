use std::f32::{INFINITY, NEG_INFINITY};

pub struct Interval {
	pub min: f32,
	pub max: f32
}
impl Interval {
	pub fn new() -> Self {
		Interval {min: NEG_INFINITY, max: INFINITY}
	}
	pub fn from(min: f32, max: f32) -> Self {
		Interval {min, max}
	}
	pub fn size(&self) -> f32 {
		self.max - self.min
	}
	pub fn contains(&self, x: f32) -> bool {
		self.min <= x && x <= self.max
	}
	pub fn surrounds(&self, x: f32) -> bool {
		self.min < x && x < self.max
	}
}