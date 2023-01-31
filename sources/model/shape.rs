use std::cell::OnceCell;
use std::iter::{once, repeat};
use std::simd::f64x4;

use crate::utils::{center, dot_product, normal};


const DIRECTION_THRESHOLD: f64 = 0.9;


#[derive(Clone, Debug)]
#[must_use]
pub struct Shape {
	center: OnceCell<f64x4>,
	normal: OnceCell<f64x4>,
	vertices: Vec<f64x4>,
}


impl Shape {
	#[inline]
	pub const fn new (vertices: Vec<f64x4>) -> Self {
		Self {
			center: OnceCell::new(),
			normal: OnceCell::new(),
			vertices,
		}
	}

	#[inline]
	#[must_use]
	pub fn center (&self) -> f64x4 {
		*self.center.get_or_init(|| center(&self.vertices))
	}

	#[inline]
	pub fn flip (&mut self) {
		self.vertices.reverse();

		if let Some(normal) = self.normal.get_mut() {
			*normal = -*normal;
		}
	}

	#[inline]
	#[must_use]
	pub fn is_downward_facing (&self) -> bool {
		self.normal()[2] <= -DIRECTION_THRESHOLD
	}

	#[inline]
	#[must_use]
	pub fn is_height_negative (&self) -> bool {
		self.normal()[2].is_sign_negative()
	}

	#[inline]
	#[must_use]
	pub fn is_height_positive (&self) -> bool {
		self.normal()[2].is_sign_positive()
	}

	#[inline]
	#[must_use]
	pub fn is_horizontal (&self) -> bool {
		self.normal()[2].abs() >= DIRECTION_THRESHOLD
	}

	#[inline]
	#[must_use]
	pub fn is_upward_facing (&self) -> bool {
		self.normal()[2] >= DIRECTION_THRESHOLD
	}

	#[inline]
	#[must_use]
	pub fn is_vertical (&self) -> bool {
		self.normal()[2].abs() <= 1. - DIRECTION_THRESHOLD
	}

	#[inline]
	#[must_use]
	pub fn normal (&self) -> f64x4 {
		*self.normal.get_or_init(|| normal(&self.vertices))
	}

	/// The `position` argument activates back-face culling.
	#[must_use]
	pub fn path (&self, width: f64, height: f64, matrix: &[f64x4; 4], position: Option<f64x4>, offset: Option<f64x4>) -> Option<String> {
		if let Some(position) = position {
			if dot_product(self.normal(), self.center() - position).is_sign_positive() {
				return None
			}
		}

		let mut path: String = once('M')
			.chain(repeat('L'))
			.zip(self.vertices.iter())
			.map(|(command, point)| {
				let mut point = *point;

				if let Some(offset) = offset {
					point += offset;
				}

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

	#[inline]
	#[must_use]
	pub fn vertices (&self) -> &[f64x4] {
		&self.vertices
	}
}
