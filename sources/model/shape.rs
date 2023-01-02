use std::cell::OnceCell;
use std::iter::{once, repeat};
use std::simd::f64x4;

use crate::utils::{center, dot_product, normal};


#[derive(Clone, Debug)]
pub struct Shape {
	center: OnceCell<f64x4>,
	normal: OnceCell<f64x4>,
	vertices: Vec<f64x4>,
}


impl Shape {
	#[inline]
	pub fn new (vertices: Vec<f64x4>) -> Self {
		Self {
			center: OnceCell::new(),
			normal: OnceCell::new(),
			vertices,
		}
	}

	#[inline]
	pub fn center (&self) -> f64x4 {
		*self.center.get_or_init(|| center(&self.vertices))
	}

	#[inline]
	pub fn flip (&mut self) {
		self.vertices.reverse();
		self.normal.get_mut().map(|normal| *normal = -*normal);
	}

	#[inline]
	pub fn is_vertical (&self) -> bool {
		self.normal()[2].abs() < 0.9
	}

	#[inline]
	pub fn normal (&self) -> f64x4 {
		*self.normal.get_or_init(|| normal(&self.vertices))
	}

	pub fn path (&self, width: f64, height: f64, matrix: &[f64x4; 4]) -> Option<String> {
		let mut path: String = once('M')
			.chain(repeat('L'))
			.zip(self.vertices.iter())
			.map(|(command, point)| {
				let mut point = *point;

				point[3] = 1.;

				let ratio = dot_product(matrix[3], point);

				// Checks whether the point is inside the current camera viewport or not
				(ratio > 0. && dot_product(matrix[2], point) > 0.).then(|| format!("{}{:.4} {:.4}", command,
					(1. + dot_product(matrix[0], point) / ratio) * width / 2.,
					// In Web canvas, the y-axis is inverted.
					(1. - dot_product(matrix[1], point) / ratio) * height / 2.
				))
			})
			.try_collect()?;

		path.push('z');
		Some(path)
	}
}
