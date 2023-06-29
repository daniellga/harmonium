use crate::from::CowArray2;
use crate::structs::{HComplexMatrix, HFloatMatrix};
use arrow2::types::NativeType;
use num_complex::Complex;
use num_traits::Float;

impl<T> HFloatMatrix<T>
where
    T: Float + NativeType,
{
    /// Converts from dB to power.
    /// $db_to_power(x) = reference * 10.0**(x * 0.1)$
    pub fn db_to_power(&mut self, reference: T) {
        let cow_arr2 = CowArray2::from(&mut *self);
        let a = T::from(10).unwrap();
        let b = T::from(0.1).unwrap();

        match cow_arr2 {
            CowArray2::Mut(mut mut_arr2) => {
                mut_arr2.mapv_inplace(|x| reference * a.powf(b * x));
            }
            CowArray2::Owned(owned_arr2) => {
                let new_arr2 = owned_arr2.mapv(|x| reference * a.powf(b * x));
                *self = HFloatMatrix::from(new_arr2);
            }
        }
    }
}

impl<T> HComplexMatrix<T>
where
    T: Float + NativeType,
{
    /// Converts from dB to power.
    /// $db_to_power(x) = reference * 10.0**(x * 0.1)$
    pub fn db_to_power(&mut self, reference: T) {
        let cow_arr2 = CowArray2::from(&mut *self);
        let a = Complex::from(T::from(10).unwrap());
        let b = Complex::from(T::from(0.1).unwrap());
        let t = Complex::from(reference);

        match cow_arr2 {
            CowArray2::Mut(mut mut_arr2) => {
                mut_arr2.mapv_inplace(|x| t * a.powc(b * x));
            }
            CowArray2::Owned(owned_arr2) => {
                let new_arr2 = owned_arr2.mapv(|x| t * a.powc(b * x));
                *self = HComplexMatrix::from(new_arr2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comparison::{compare_hcmatrix, compare_hfmatrix};

    use super::*;

    #[test]
    fn db_to_power_test() {
        let mut hmatrix =
            HFloatMatrix::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2).unwrap();
        let initial_ptr = format!("{:p}", hmatrix.as_slice());
        hmatrix.db_to_power(1.0);
        let result = HFloatMatrix::new_from_vec(
            vec![
                1.258925, 1.584893, 1.995262, 2.511886, 3.162278, 3.981072, 5.011872, 6.309574,
            ],
            2,
        )
        .unwrap();
        let final_ptr = format!("{:p}", hmatrix.as_slice());
        println!("{:?}", hmatrix);
        println!("{:?}", result);
        assert_eq!(initial_ptr, final_ptr);
        assert!(compare_hfmatrix(hmatrix, result));

        let mut hmatrix =
            HFloatMatrix::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2).unwrap();
        let initial_ptr = format!("{:p}", hmatrix.as_slice());
        let _b = hmatrix.clone();
        hmatrix.db_to_power(1.0);
        let result = HFloatMatrix::new_from_vec(
            vec![
                1.258925, 1.584893, 1.995262, 2.511886, 3.162278, 3.981072, 5.011872, 6.309574,
            ],
            2,
        )
        .unwrap();
        let final_ptr = format!("{:p}", hmatrix.as_slice());
        assert_ne!(initial_ptr, final_ptr);
        assert!(compare_hfmatrix(hmatrix, result));

        let mut hmatrix =
            HComplexMatrix::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2).unwrap();
        let initial_ptr = format!("{:p}", hmatrix.as_slice());
        hmatrix.db_to_power(1.0);
        let result = HComplexMatrix::new_from_vec(
            vec![
                1.12777415,
                0.55948071,
                1.20712802,
                1.58868299,
                0.59488038,
                3.10581991,
                -1.34296574,
                4.82859269,
            ],
            2,
        )
        .unwrap();
        println!("{:?}", hmatrix);
        println!("{:?}", result);
        let final_ptr = format!("{:p}", hmatrix.as_slice());
        assert_eq!(initial_ptr, final_ptr);
        assert!(compare_hcmatrix(hmatrix, result));

        let mut hmatrix =
            HComplexMatrix::new_from_vec(vec![1., 2., 3., 4., 5., 6., 7., 8.], 2).unwrap();
        let initial_ptr = format!("{:p}", hmatrix.as_slice());
        let _b = hmatrix.clone();
        hmatrix.db_to_power(1.0);
        let result = HComplexMatrix::new_from_vec(
            vec![
                1.12777415,
                0.55948071,
                1.20712802,
                1.58868299,
                0.59488038,
                3.10581991,
                -1.34296574,
                4.82859269,
            ],
            2,
        )
        .unwrap();
        let final_ptr = format!("{:p}", hmatrix.as_slice());
        assert_ne!(initial_ptr, final_ptr);
        assert!(compare_hcmatrix(hmatrix, result));
    }
}
