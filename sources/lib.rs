#![feature(
	iterator_try_collect,
	is_sorted,
	once_cell,
	option_result_contains,
	portable_simd,
)]


#[cfg(all(feature = "canvas", feature = "svg"))]
compile_error!("feature \"canvas\" and feature \"svg\" cannot be enabled at the same time");


pub mod components;
pub mod model;
pub mod state;
pub mod utils;
