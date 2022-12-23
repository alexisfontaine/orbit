use super::{Camera, Shape};


#[derive(Debug)]
pub struct Scene {
	pub cameras: Vec<Camera>,
	pub shapes: Vec<Shape>,
}
