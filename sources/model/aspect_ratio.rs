#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct AspectRatio {
	pub height: usize,
	pub width: usize,
}


impl AspectRatio {
	#[inline]
	#[must_use]
	pub fn format (&self) -> String {
		format!("{}/{}", self.width, self.height)
	}

	#[inline]
	#[must_use]
	pub fn value (&self) -> f64 {
		self.width as f64 / self.height as f64
	}
}
