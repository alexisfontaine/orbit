#![feature(
	iterator_try_collect,
	once_cell,
	portable_simd,
)]


#[cfg(all(feature = "canvas", feature = "svg"))]
compile_error!("feature \"canvas\" and feature \"svg\" cannot be enabled at the same time");


pub mod components;
pub mod model;
pub mod state;
pub mod utils;
