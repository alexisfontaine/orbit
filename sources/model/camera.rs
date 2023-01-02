use super::{Style, Viewport};


#[derive(Clone, Debug)]
pub struct Camera {
	pub aspect_ratio: String,
	pub styles: Vec<Style>,
	pub viewports: Vec<Viewport>,
}
