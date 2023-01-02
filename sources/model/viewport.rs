use std::simd::f64x4;

use super::Source;


#[derive(Clone, Debug)]
pub struct Viewport {
	pub matrix: [f64x4; 4],
	pub position: f64x4,
	pub source: Option<Source>,
}
