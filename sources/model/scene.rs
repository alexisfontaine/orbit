use super::{Camera, Shape};


#[derive(Clone, Debug)]
pub struct Scene {
	pub cameras: Vec<Camera>,
	pub shapes: Vec<Shape>,
}
