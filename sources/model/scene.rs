use super::{Camera, Shape};


#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct Scene {
	pub cameras: Vec<Camera>,
	pub shapes: Vec<Shape>,
}


impl Scene {
	#[inline]
	pub const fn new (cameras: Vec<Camera>, shapes: Vec<Shape>) -> Self {
		Self {
			cameras,
			shapes,
		}
	}
}
