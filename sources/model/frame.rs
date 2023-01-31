#[derive(Clone, Debug)]
#[must_use]
#[non_exhaustive]
pub struct Frame {
	pub kind: Option<String>,
	pub size: Option<usize>,
	pub source: String,
}


impl Frame {
	#[inline]
	pub const fn new (source: String) -> Self {
		Self::with_media_query(None, None, source)
	}

	#[inline]
	pub const fn with_media_query (kind: Option<String>, size: Option<usize>, source: String) -> Self {
		Self {
			kind,
			size,
			source,
		}
	}

	#[inline]
	#[must_use]
	pub const fn is_fallback (&self) -> bool {
		self.kind.is_none() && self.size.is_none()
	}
}
