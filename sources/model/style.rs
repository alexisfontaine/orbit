#[derive(Clone, Debug)]
#[must_use]
enum Kind {
	Compound(CompoundStyle),
	Shape(ShapeStyle),
}


#[derive(Clone, Debug)]
#[must_use]
pub struct CompoundStyle {
	pub children: Vec<Style>,
	pub name: String,
}

#[derive(Clone, Debug)]
#[must_use]
pub struct ShapeStyle {
	pub index: usize,
	pub name: String,
}

#[derive(Clone, Debug)]
#[must_use]
pub struct Style(Kind);


impl Style {
	#[inline]
	pub fn compound (name: String, children: Vec<Self>) -> Self {
		Self(Kind::Compound(CompoundStyle { children, name }))
	}

	#[inline]
	pub fn shape (name: String, index: usize) -> Self {
		Self(Kind::Shape(ShapeStyle { index, name }))
	}

	#[inline]
	#[must_use]
	pub(crate) fn get_compound (&self) -> Option<&CompoundStyle> {
		if let Kind::Compound(style) = &self.0 {
			Some(style)
		} else {
			None
		}
	}

	#[inline]
	#[must_use]
	pub(crate) fn get_shape (&self) -> Option<&ShapeStyle> {
		if let Kind::Shape(style) = &self.0 {
			Some(style)
		} else {
			None
		}
	}
}
