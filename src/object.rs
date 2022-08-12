use std::f64::consts::PI;
use crate::min;

pub enum Object {
	Cylinder(f64, f64),
	Box(f64, f64, f64)
}

impl Object {
	pub fn from_values(values: &[f64]) -> Result<Self, ()> {
		if values.len() == 2 {
			Ok(Self::Cylinder(values[0], values[1]))
		}
		else if values.len() == 3 {
			Ok(Self::Box(values[0], values[1], values[2]))
		}
		else {
			Err(())
		}
	}
	
	pub fn volume(&self) -> f64 {
		match self {
			Self::Cylinder(r, d) => r*r*PI*d,
			Self::Box(l,w,h) => l*w*h
		}
	}
	
	pub fn surface_areas(&self) -> Vec<f64> {
		match self {
			Self::Cylinder(r, d) => vec![r*r*PI, r*r*PI, 2.*r*PI*d], // circle top, circle bottom, other face
			Self::Box(l,w,h)     => vec![l*w, l*w, l*h, l*h, w*h, w*h]
		}
	}
	
	pub fn shortest_circumference(&self) -> f64 {
		match self {
			Self::Cylinder(r, d) => {
				min!(
					2.*r*PI,
					(d+(2.*r)) * 2.
				)
			}
			Self::Box(l,w,h) => {
				let mut sides = [l,w,h];
				sides.sort_by(|a,b| a.partial_cmp(b).unwrap());
				
				(sides[0] + sides[1]) * 2.
			}
		}
	}
}

