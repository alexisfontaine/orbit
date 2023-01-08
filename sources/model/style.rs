#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Style {
	pub index: usize,
	pub name: String,
}


impl Style {
	#[inline]
	pub fn new (name: String, index: usize) -> Self {
		Self {
			index,
			name,
		}
	}
}
