use super::{Style, Viewport};


#[derive(Debug)]
pub struct Camera {
	pub aspect_ratio: String,
	pub styles: Vec<Style>,
	pub viewports: Vec<Viewport>,
}
