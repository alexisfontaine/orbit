use super::{AspectRatio, Style, Viewport};


#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct Camera {
	pub aspect_ratio: AspectRatio,
	pub styles: Vec<Style>,
	pub viewports: Vec<Viewport>,
}


impl Camera {
	#[inline]
	pub const fn new (
		aspect_ratio_width: usize,
		aspect_ratio_height: usize,
		viewports: Vec<Viewport>,
		styles: Vec<Style>,
	) -> Self {
		Self {
			aspect_ratio: AspectRatio {
				height: aspect_ratio_height,
				width: aspect_ratio_width,
			},
			styles,
			viewports,
		}
	}
}
