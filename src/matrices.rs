
// TODO: Support a lot more references, both in Mul<&...> and also in Self of those impls.

use core::ops::{Add, Mul, Neg, Sub};
use num_traits::{Zero, One, Pow};
use crate::traits::{Field, Vector, Matrix};
use core::fmt;

// # MatrixRc: Row-major

#[derive(PartialEq, Clone)]
#[repr(C)]
pub struct MatrixRc<F: Field, const ROW_COUNT: usize, const COLUMN_COUNT: usize>([[F; COLUMN_COUNT]; ROW_COUNT]);

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Zero for MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	fn zero() -> Self {
		let result = [[F::zero(); COLUMN_COUNT]; ROW_COUNT];
		MatrixRc(result)
	}
	fn is_zero(&self) -> bool {
		let result = [[F::zero(); COLUMN_COUNT]; ROW_COUNT];
		self.0 == result
	}
}

impl<F: Field + Copy, const RC_COUNT: usize> One for MatrixRc<F, RC_COUNT, RC_COUNT> {
	fn one() -> Self {
		let mut result = [[F::zero(); RC_COUNT]; RC_COUNT];
		for i in 0..RC_COUNT {
			result[i][i] = F::one()
		}
		Self(result)
	}
}

impl<F: Field + Copy, const RC_COUNT: usize> MatrixRc<F, RC_COUNT, RC_COUNT> {
	pub fn trace(&self) -> F {
		let mut result = F::zero();
		for i in 0..RC_COUNT {
			result = result + self.0[i][i]
		}
		result
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Matrix<F, ROW_COUNT, COLUMN_COUNT> for MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {}

impl<F: Field + Copy, const ROW_COUNT: usize, const X_COUNT: usize, const COLUMN_COUNT: usize> Mul<&MatrixRc<F, X_COUNT, COLUMN_COUNT>> for &MatrixRc<F, ROW_COUNT, X_COUNT> {
	type Output = MatrixRc<F, ROW_COUNT, COLUMN_COUNT>;
	fn mul(self, rhs: &MatrixRc<F, X_COUNT, COLUMN_COUNT>) -> Self::Output {
		let mut result = [[F::zero(); COLUMN_COUNT]; ROW_COUNT];
		for i in 0..ROW_COUNT {
			for k in 0..COLUMN_COUNT {
				for j in 0..X_COUNT {
					result[i][k] = result[i][k] + self.0[i][j] * rhs.0[j][k]
				}
			}
		}
		MatrixRc(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const X_COUNT: usize, const COLUMN_COUNT: usize> Mul<MatrixRc<F, X_COUNT, COLUMN_COUNT>> for MatrixRc<F, ROW_COUNT, X_COUNT> {
	type Output = MatrixRc<F, ROW_COUNT, COLUMN_COUNT>;
	fn mul(self, rhs: MatrixRc<F, X_COUNT, COLUMN_COUNT>) -> Self::Output {
		(&self).mul(&rhs)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Add for &MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = MatrixRc<F, ROW_COUNT, COLUMN_COUNT>;
	fn add(self, rhs: Self) -> Self::Output {
		let mut result = [[F::zero(); COLUMN_COUNT]; ROW_COUNT];
		for i in 0..ROW_COUNT {
			for j in 0..COLUMN_COUNT {
				result[i][j] = self.0[i][j] + rhs.0[i][j]
			}
		}
		MatrixRc(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Add for MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		(&self).add(&rhs)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Sub for &MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = MatrixRc<F, ROW_COUNT, COLUMN_COUNT>;
	fn sub(self, rhs: Self) -> Self::Output {
		let mut result = [[F::zero(); COLUMN_COUNT]; ROW_COUNT];
		for i in 0..ROW_COUNT {
			for j in 0..COLUMN_COUNT {
				result[i][j] = self.0[i][j] - rhs.0[i][j]
			}
		}
		MatrixRc(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Sub for MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		(&self).sub(&rhs)
	}
}


impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Neg for &MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = MatrixRc<F, ROW_COUNT, COLUMN_COUNT>;
	fn neg(self) -> Self::Output {
		let mut result = [[F::zero(); COLUMN_COUNT]; ROW_COUNT];
		for i in 0..ROW_COUNT {
			for j in 0..COLUMN_COUNT {
				result[i][j] = -self.0[i][j]
			}
		}
		MatrixRc(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Neg for MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn neg(self) -> Self::Output {
		(&self).neg()
	}
}


impl<F: Field + Copy, const RC_COUNT: usize> Pow<usize> for &MatrixRc<F, RC_COUNT, RC_COUNT> {
	type Output = MatrixRc<F, RC_COUNT, RC_COUNT>;
	fn pow(self, rhs: usize) -> Self::Output {
		let mut result = Self::Output::one();
		for _i in 0..rhs {
			result = &result * self;
		}
		result
	}
}

impl<F: Field + Copy, const RC_COUNT: usize> Pow<usize> for MatrixRc<F, RC_COUNT, RC_COUNT> {
	type Output = Self;
	fn pow(self, rhs: usize) -> Self::Output {
		(&self).pow(rhs)
	}
}

impl<F: Field + Copy + fmt::Debug, const ROW_COUNT: usize, const COLUMN_COUNT: usize> fmt::Debug for MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if COLUMN_COUNT > 1 {
			write!(f, "MatrixRc::new_from_vectors([")?;
		}
		for i in 0..COLUMN_COUNT {
			let mut coordinates: [F; ROW_COUNT] = [F::zero(); ROW_COUNT];
			for j in 0..ROW_COUNT {
				coordinates[j] = self.0[j][i];
			}
			write!(f, "ColumnVector::new_from_coordinates({:?}),", coordinates)?;
		}
		if COLUMN_COUNT > 1 {
			write!(f, "])")?;
		}
		Ok(())
	}
}

// # MatrixCr: Column-major

// TODO: maybe a reference instead; or a Cow
#[derive(PartialEq, Clone)]
#[repr(C)]
pub struct MatrixCr<F: Field, const ROW_COUNT: usize, const COLUMN_COUNT: usize>([[F; ROW_COUNT]; COLUMN_COUNT]);

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Add for &MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = MatrixCr<F, ROW_COUNT, COLUMN_COUNT>;
	fn add(self, rhs: Self) -> Self::Output {
		let mut result = [[F::zero(); ROW_COUNT]; COLUMN_COUNT];
		for i in 0..COLUMN_COUNT {
			for j in 0..ROW_COUNT {
				result[i][j] = self.0[i][j] + rhs.0[i][j]
			}
		}
		MatrixCr(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Add for MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		(&self).add(&rhs)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Sub for &MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = MatrixCr<F, ROW_COUNT, COLUMN_COUNT>;
	fn sub(self, rhs: Self) -> Self::Output {
		let mut result = [[F::zero(); ROW_COUNT]; COLUMN_COUNT];
		for i in 0..COLUMN_COUNT {
			for j in 0..ROW_COUNT {
				result[i][j] = self.0[i][j] - rhs.0[i][j]
			}
		}
		MatrixCr(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Sub for MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		(&self).sub(&rhs)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Neg for &MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = MatrixCr<F, ROW_COUNT, COLUMN_COUNT>;
	fn neg(self) -> Self::Output {
		let mut result = [[F::zero(); ROW_COUNT]; COLUMN_COUNT];
		for i in 0..COLUMN_COUNT {
			for j in 0..ROW_COUNT {
				result[i][j] = -self.0[i][j]
			}
		}
		MatrixCr(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Neg for MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn neg(self) -> Self::Output {
		(&self).neg()
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Zero for MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	fn zero() -> Self {
		let result = [[F::zero(); ROW_COUNT]; COLUMN_COUNT];
		Self(result)
	}
	fn is_zero(&self) -> bool {
		let result = [[F::zero(); ROW_COUNT]; COLUMN_COUNT];
		self.0 == result
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const X_COUNT: usize, const COLUMN_COUNT: usize> Mul<&MatrixCr<F, X_COUNT, COLUMN_COUNT>> for &MatrixCr<F, ROW_COUNT, X_COUNT> {
	type Output = MatrixCr<F, ROW_COUNT, COLUMN_COUNT>;
	fn mul(self, rhs: &MatrixCr<F, X_COUNT, COLUMN_COUNT>) -> Self::Output {
		let mut result = [[F::zero(); ROW_COUNT]; COLUMN_COUNT];
		for i in 0..ROW_COUNT {
			for k in 0..COLUMN_COUNT {
				for j in 0..X_COUNT {
					result[k][i] = result[k][i] + self.0[j][i] * rhs.0[k][j]
				}
			}
		}
		MatrixCr(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const X_COUNT: usize, const COLUMN_COUNT: usize> Mul<MatrixCr<F, X_COUNT, COLUMN_COUNT>> for MatrixCr<F, ROW_COUNT, X_COUNT> {
	type Output = MatrixCr<F, ROW_COUNT, COLUMN_COUNT>;
	fn mul(self, rhs: MatrixCr<F, X_COUNT, COLUMN_COUNT>) -> Self::Output {
		(&self).mul(&rhs)
	}
}

impl<F: Field + Copy, const RC_COUNT: usize> One for MatrixCr<F, RC_COUNT, RC_COUNT> {
	fn one() -> Self {
		let mut result = [[F::zero(); RC_COUNT]; RC_COUNT];
		for i in 0..RC_COUNT {
			result[i][i] = F::one()
		}
		Self(result)
	}
}

impl<F: Field + Copy, const RC_COUNT: usize> MatrixCr<F, RC_COUNT, RC_COUNT> {
	pub fn trace(&self) -> F {
		let mut result = F::zero();
		for i in 0..RC_COUNT {
			result = result + self.0[i][i]
		}
		result
	}
}

impl<F: Field + Copy + fmt::Debug, const ROW_COUNT: usize, const COLUMN_COUNT: usize> fmt::Debug for MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if COLUMN_COUNT > 1 {
			write!(f, "MatrixCr::new_from_vectors([")?;
		}
		for i in 0..COLUMN_COUNT {
			let mut coordinates: [F; ROW_COUNT] = [F::zero(); ROW_COUNT];
			for j in 0..ROW_COUNT {
				coordinates[j] = self.0[i][j];
			}
			write!(f, "ColumnVector::new_from_coordinates({:?}),", coordinates)?;
		}
		if COLUMN_COUNT > 1 {
			write!(f, "])")?;
		}
		Ok(())
	}
}

pub type ColumnVector<F, const DIM: usize> = MatrixCr<F, DIM, 1>;

impl<F: Field + Copy, const ROW_COUNT: usize> ColumnVector<F, ROW_COUNT> {
	/// One-based index.
	/// # Examples
	/// ```
	/// # use rinear::matrices::ColumnVector;
	/// assert_eq!(ColumnVector::<i32, 3>::new_from_coordinates([10, 20, 30]).obi(1), 10);
	/// assert_eq!(ColumnVector::<i32, 3>::new_from_coordinates([10, 20, 30]).obi(3), 30)
	/// ```
	pub fn obi(&self, index: usize) -> F {
		assert!(index > 0);
		self.0[0][index - 1]
	}
	pub fn new_from_coordinates(coordinates: [F; ROW_COUNT]) -> Self {
		Self([coordinates])
	}
}

impl<F: Field + Copy> ColumnVector<F, 3> {
	/// Cross product.
	/// # Examples
	/// ```
	/// # use rinear::matrices::ColumnVector;
	/// # use num_traits::Zero;
	/// let a = ColumnVector::new_from_coordinates([10, 20, 30]);
	/// assert_eq!(a.cross(&a), Zero::zero());
	/// ```
	pub fn cross(&self, b: &Self) -> Self {
		let a = self;
		Self::new_from_coordinates([
			a.obi(2) * b.obi(3) - a.obi(3) * b.obi(2),
			a.obi(1) * b.obi(3) - a.obi(3) * b.obi(1),
			a.obi(1) * b.obi(2) - a.obi(2) * b.obi(1)
		])
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Mul<F> for MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn mul(self, rhs: F) -> Self {
		Self(self.0.map(|a| a.map(|aa| aa * rhs)))
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> Mul<F> for MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	type Output = Self;
	fn mul(self, rhs: F) -> Self {
		Self(self.0.map(|a| a.map(|aa| aa * rhs)))
	}
}

impl<F: Field + Copy, const DIM: usize> Vector<F> for ColumnVector<F, DIM> {}

impl<F: Field + Copy, const DIM: usize> ColumnVector<F, DIM> {
	/// Dot product (scalar product).
	/// # Examples
	/// ```
	/// # use rinear::matrices::ColumnVector;
	/// # use num_traits::Zero;
	/// let a = ColumnVector::new_from_coordinates([10, 20, 30]);
	/// assert_eq!(a.dot(&a), 1400);
	/// let b = ColumnVector::new_from_coordinates([0, 0, 0]);
	/// assert_eq!(a.dot(&b), 0);
	/// ```
	pub fn dot(&self, rhs: &Self) -> F {
		self.0[0].into_iter().zip(rhs.0[0]).map(|(a,b)| a * b).into_iter().sum()
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> MatrixRc<F, ROW_COUNT, COLUMN_COUNT> {
	/// Column 1-based index
	/// # Examples
	/// ```
	/// # use rinear::matrices::{ColumnVector, MatrixRc};
	/// assert_eq!(MatrixRc::new_from_vectors([
	///     ColumnVector::new_from_coordinates([10, 20, 30]),
	///     ColumnVector::new_from_coordinates([40, 50, 60])
	/// ]).cobi(1), ColumnVector::new_from_coordinates([10, 20, 30]));
	/// assert_eq!(ColumnVector::new_from_coordinates([10, 20, 30]).obi(3), 30)
	/// ```
	pub fn cobi(&self, index: usize) -> ColumnVector<F, ROW_COUNT> {
		assert!(index > 0);
		let index = index - 1;
		let mut result: [F; ROW_COUNT] = [F::zero(); ROW_COUNT];
		for i in 0..ROW_COUNT {
			result[i] = self.0[i][index]
		}
		ColumnVector::new_from_coordinates(result)
	}
	/// Construct a matrix from the given column vectors
	pub fn new_from_vectors(columns: [ColumnVector<F, ROW_COUNT>; COLUMN_COUNT]) -> Self {
		let mut result: [[F; COLUMN_COUNT]; ROW_COUNT] = [[F::zero(); COLUMN_COUNT]; ROW_COUNT];
		for i in 0..ROW_COUNT {
			for j in 0..COLUMN_COUNT {
				result[i][j] = columns[j].0[0][i]
			}
		}
		Self(result)
	}
}

impl<F: Field + Copy, const ROW_COUNT: usize, const COLUMN_COUNT: usize> MatrixCr<F, ROW_COUNT, COLUMN_COUNT> {
	/// Column 1-based index
	/// # Examples
	/// ```
	/// # use rinear::matrices::{ColumnVector, MatrixRc};
	/// assert_eq!(MatrixRc::new_from_vectors([
	///     ColumnVector::new_from_coordinates([10, 20, 30]),
	///     ColumnVector::new_from_coordinates([40, 50, 60])
	/// ]).cobi(1), ColumnVector::new_from_coordinates([10, 20, 30]));
	/// assert_eq!(ColumnVector::<i32, 3>::new_from_coordinates([10, 20, 30]).obi(3), 30)
	/// ```
	pub fn cobi(&self, index: usize) -> ColumnVector<F, ROW_COUNT> {
		assert!(index > 0);
		ColumnVector::new_from_coordinates(self.0[index - 1])
	}
	/// Construct a matrix from the given column vectors
	pub fn new_from_vectors(columns: [ColumnVector<F, ROW_COUNT>; COLUMN_COUNT]) -> Self {
		Self(columns.map(|x| x.0[0]))
	}
}

impl<F: Field + Copy, const RC_COUNT: usize> Pow<usize> for &MatrixCr<F, RC_COUNT, RC_COUNT> {
	type Output = MatrixCr<F, RC_COUNT, RC_COUNT>;
	fn pow(self, rhs: usize) -> Self::Output {
		let mut rhs = rhs;
		let mut result = MatrixCr::<F, RC_COUNT, RC_COUNT>::one();
		while rhs > 0 {
			result = &result * self;
			rhs = rhs - 1;
		}
		result
	}
}

impl<F: Field + Copy, const RC_COUNT: usize> Pow<usize> for MatrixCr<F, RC_COUNT, RC_COUNT> {
	type Output = Self;
	fn pow(self, rhs: usize) -> Self::Output {
		(&self).pow(rhs)
	}
}

/* Tests are better without those */

#[cfg(not(test))]
impl Copy for MatrixRc<f32, 1, 1> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 1, 2> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 1, 3> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 1, 4> {
}

#[cfg(not(test))]
impl Copy for MatrixRc<f32, 2, 1> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 2, 2> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 2, 3> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 2, 4> {
}

#[cfg(not(test))]
impl Copy for MatrixRc<f32, 3, 1> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 3, 2> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 3, 3> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 3, 4> {
}

#[cfg(not(test))]
impl Copy for MatrixRc<f32, 4, 1> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 4, 2> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 4, 3> {
}
#[cfg(not(test))]
impl Copy for MatrixRc<f32, 4, 4> {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn test_matrices() {
		let v = ColumnVector::new_from_coordinates(
			[1.0,
			 3.0,
			 4.0]);
		let w = ColumnVector::new_from_coordinates(
			[1.0,
			 0.0,
			 4.0]);
		assert_eq!(v + w, ColumnVector::new_from_coordinates(
			[2.0,
			 3.0,
			 8.0]));
		let v = ColumnVector::new_from_coordinates(
			[1.0,
			 3.0,
			 4.0]);
		assert_eq!(v.cross(&v), ColumnVector::zero());
		let v = ColumnVector::new_from_coordinates(
			[1.0,
			 3.0,
			 4.0]);
		//eprintln!("v = {:?}", v);
		assert_eq!(v.dot(&v), 26.0);

		let v2 = MatrixCr::new_from_vectors([ColumnVector::new_from_coordinates([1.0, 3.0, 4.0]),]);
		let v3 = ColumnVector::new_from_coordinates([1.0, 3.0, 4.0]);
		assert_eq!(v2, v3);
		assert_eq!(v, v3);
		let vb = MatrixCr::new_from_vectors([
			ColumnVector::new_from_coordinates([1.0, 3.0, 4.0]),
			ColumnVector::new_from_coordinates([5.0, 4.0, 3.0]),
			ColumnVector::new_from_coordinates([0.0, 7.0, 1.0]),
		]);
		assert!(vb != One::one());
		assert_eq!((&vb).pow(1), *&vb);
		assert!((&vb).pow(0) == One::one());
		assert_eq!((&vb).neg().neg(), vb);
		assert_eq!((&vb).pow(2), MatrixCr::new_from_vectors([
			ColumnVector::new_from_coordinates([16.0, 43.0, 17.0]),
			ColumnVector::new_from_coordinates([25.0, 52.0, 35.0]),
			ColumnVector::new_from_coordinates([35.0, 35.0, 22.0]),
		]));
		assert_eq!((&vb).pow(3), MatrixCr::new_from_vectors([
			ColumnVector::new_from_coordinates([231.0, 339.0, 210.0]),
			ColumnVector::new_from_coordinates([285.0, 528.0, 291.0]),
			ColumnVector::new_from_coordinates([210.0, 399.0, 267.0]),
		]));
		let _x = MatrixRc::<i32, 0, 0>::new_from_vectors([]);
	}
}
