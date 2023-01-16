use std::simd::f64x4;


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
	pub identifier: String,
	pub name: String,
}

#[derive(Clone, Debug)]
#[must_use]
pub struct ShapeStyle {
	pub index: usize,
	pub name: String,
	pub offset: Option<f64x4>,
}

#[derive(Clone, Debug)]
#[must_use]
pub struct Style(Kind);


impl Style {
	#[inline]
	pub fn compound (identifier: String, name: String, children: Vec<Self>) -> Self {
		Self(Kind::Compound(CompoundStyle { children, identifier, name }))
	}

	#[inline]
	pub fn shape (name: String, index: usize, offset: Option<f64x4>) -> Self {
		Self(Kind::Shape(ShapeStyle { index, name, offset }))
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
