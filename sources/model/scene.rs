use super::{Camera, Shape};


#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Scene {
	pub cameras: Vec<Camera>,
	pub shapes: Vec<Shape>,
}


impl Scene {
	#[inline]
	pub fn new (cameras: Vec<Camera>, shapes: Vec<Shape>) -> Self {
		Self {
			cameras,
			shapes,
		}
	}
}
