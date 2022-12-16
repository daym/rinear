/* A possible representation of affine space elements */

use crate::traits::Field;
use crate::matrices::ColumnVector;

pub type Point<F, const DIM: usize> = ColumnVector<F, {DIM + 1}>;

//impl<F: Field, const DIM: usize> Point<F, DIM> {
    pub fn new_point<F: Field + Copy, const DIM: usize>(coordinates: [F; DIM]) -> ColumnVector<F, {DIM + 1}> {
        let mut xcoordinates: [F; DIM + 1] = [F::one(); DIM + 1];
        for i in 0..DIM {
            xcoordinates[i] = coordinates[i]
        }
        ColumnVector::new_from_coordinates(xcoordinates)
    }
    pub fn new_vector<F: Field + Copy, const DIM: usize>(coordinates: [F; DIM]) -> ColumnVector<F, {DIM + 1}> {
        let mut xcoordinates: [F; DIM + 1] = [F::zero(); DIM + 1];
        for i in 0..DIM {
            xcoordinates[i] = coordinates[i]
        }
        ColumnVector::new_from_coordinates(xcoordinates)
    }
    pub fn is_point<F: Field + Copy, const DIM: usize>(m: &ColumnVector<F, DIM>) -> bool {
        let marker = m.obi(DIM);
        marker == F::one()
    }
    pub fn is_vector<F: Field + Copy, const DIM: usize>(m: &ColumnVector<F, DIM>) -> bool {
        let marker = m.obi(DIM);
        marker == F::zero()
    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        let p_0 = new_point([1.0, 2.0, 3.0]);
        let p_1 = new_point([4.0, 5.0, 6.0]);
        let p_2 = new_point([10.0, 11.0, 12.0]);
        let v_1 = new_vector([7.0, 8.0, 9.0]);
        assert!(is_vector(&(&p_1 - &p_0)));
        assert!(!is_vector(&(&p_1 + &p_0)));
        assert!(!is_point(&(&p_1 + &p_0)));
        assert!(is_point(&(&v_1 + &p_0)));
        let p_m = (&(&p_0 + &p_1) + &p_2)*(1.0/3.0);
        assert!(is_point(&p_m));
        assert!(!is_vector(&p_m));
        let v_2 = &(p_0*(1.0/2.0) + p_1*(1.0/2.0) - p_2);
        assert!(is_vector(v_2));
        assert!(!is_point(v_2));
    }
}