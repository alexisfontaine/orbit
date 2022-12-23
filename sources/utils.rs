use std::ops::Add;
use std::simd::{f64x4, simd_swizzle, Simd, SimdFloat};


const YZX: [usize; 4] = [1, 2, 0, 3];
const ZXY: [usize; 4] = [2, 0, 1, 3];


pub fn center (points: &[f64x4]) -> f64x4 {
	debug_assert!(points.len() > 0);
	points.iter().copied().reduce(Add::add).unwrap() / Simd::splat(points.len() as _)
}

// pub fn cross_product (vector_1: f64x4, vector_2: f64x4) -> f64x4 {
// 	simd_swizzle!(vector_1, YZX) * simd_swizzle!(vector_2, ZXY) - 
// 	simd_swizzle!(vector_1, ZXY) * simd_swizzle!(vector_2, YZX)
// }

#[inline]
pub fn dot_product (vector_1: f64x4, vector_2: f64x4) -> f64 {
	(vector_1 * vector_2).reduce_sum()
}

/// Based on Newell's method
pub fn normal (points: &[f64x4]) -> f64x4 {
	debug_assert!(points.len() > 2);

	let mut points = points
		.iter()
		.map(|&point| (simd_swizzle!(point, YZX), simd_swizzle!(point, ZXY)))
		.cycle()
		.take(points.len() + 1)
		.peekable();

	let mut normal = f64x4::default();

	while let Some((current, next)) = points.next().zip(points.peek()) {
		normal += (current.0 - next.0) * (current.1 + next.1)
	}

	normal / Simd::splat(dot_product(normal, normal).sqrt())
}
