use core::ops::{Add, Sub, Mul, Div, Neg};
use num_traits::{Zero, One};
use core::iter::Sum;

// TODO: AddAssign and so on (sigh)
pub trait Field: Add<Output=Self> + Sub<Output=Self> + Neg<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Zero + One + PartialEq + Sum {}

impl Field for f32 {}
impl Field for i32 {}
impl Field for f64 {}
impl Field for i64 {}

// TODO: AddAssign and so on (sigh)
pub trait Vector<F: Field>: Add<Output=Self> + Sub<Output=Self> + Neg + Mul<F> + Zero {}

pub trait Matrix<F: Field, const ROW_COUNT: usize, const COLUMN_COUNT: usize>: Zero + Add + Sub + Neg {
}
