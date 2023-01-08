use super::{Style, Viewport};


#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct Camera {
	pub aspect_ratio: String,
	pub styles: Vec<Style>,
	pub viewports: Vec<Viewport>,
}


impl Camera {
	#[inline]
	pub const fn new (aspect_ratio: String, viewports: Vec<Viewport>, styles: Vec<Style>) -> Self {
		Self {
			aspect_ratio,
			styles,
			viewports,
		}
	}
}
