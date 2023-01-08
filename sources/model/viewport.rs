use std::simd::f64x4;


#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct Viewport {
	pub matrix: [f64x4; 4],
	pub position: f64x4,
	pub sources: Vec<(usize, String)>,
}


impl Viewport {
	#[inline]
	pub fn new (position: f64x4, matrix: [f64x4; 4], sources: Vec<(usize, String)>) -> Self {
		debug_assert!(sources.windows(2).all(|source| source[0].0 < source[1].0));
		debug_assert!(if let Some((width, _)) = sources.get(0) { *width == 0 } else { true });

		Self {
			matrix,
			position,
			sources,
		}
	}
}
