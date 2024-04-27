use std::io::{Stdout, Write};

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(out: &mut Stdout, pixel_color: Color) {
	let r = pixel_color.x();
	let g = pixel_color.y();
	let b = pixel_color.z();

	let intensity = Interval::from(0.0, 0.999);
	let rbyte = (256.0 * intensity.clamp(r)) as i32;
	let gbyte = (256.0 * intensity.clamp(g)) as i32;
	let bbyte = (256.0 * intensity.clamp(b)) as i32;
	out.write_all(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes()).unwrap();
}