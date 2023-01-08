#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct Style {
	pub index: usize,
	pub name: String,
}


impl Style {
	#[inline]
	pub const fn new (name: String, index: usize) -> Self {
		Self {
			index,
			name,
		}
	}
}
