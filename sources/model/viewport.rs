use std::simd::f64x4;

use super::Frame;


#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct Viewport {
	pub frames: Vec<Frame>,
	pub matrix: [f64x4; 4],
	pub position: f64x4,
}


impl Viewport {
	#[inline]
	pub fn new (position: f64x4, matrix: [f64x4; 4], frames: Vec<Frame>) -> Self {
		debug_assert!(frames.is_sorted_by_key(|frame| frame.kind.as_ref().zip(frame.size.or(Some(usize::MAX)))));
		debug_assert!(frames.iter().any(Frame::is_fallback));

		Self {
			frames,
			matrix,
			position,
		}
	}
}
