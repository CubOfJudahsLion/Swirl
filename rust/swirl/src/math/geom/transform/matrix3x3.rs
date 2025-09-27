//! Module  :   Matrix3x3
//! Purpose :   Operations on actual 3x3 matrices.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu 09/09/2021 01:47 PM
//! Modified:   Tue 09/28/2021 10:41 AM


use std::cmp::PartialEq;
use std::ops::{AddAssign, Sub, Mul, Div};

use super::*;


/// A fully realized 3x3 matrix, made for more
/// complex algorithms.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix3x3<N>(pub [[N; 3]; 3]);


impl<N> Default for Matrix3x3<N>
    where N: Default + Copy
{
    fn default() -> Self {
        let d = N::default();
        Matrix3x3(
            [[d, d, d],
             [d, d, d],
             [d, d, d]]
        )
    }
}

impl<N: Copy> Matrix3x3<N> {
    /// Makes a `Matrix3x3` equivalent to a `Transform`.
    pub fn new_from_transform(t: &Transform<N>) -> Self
        where N: From<i16>
    {
        let zero    = N::from(0);
        let one     = N::from(1);
        Matrix3x3(
            [[t.0[0][0], t.0[0][1], zero],
             [t.0[1][0], t.0[1][1], zero],
             [t.0[2][0], t.0[2][1], one ]]
        )
    }

    //  Produces a matrix of minors.
    fn minors(&self) -> Self
        where   N   :   PartialEq
                    +   Sub<Output=N> + Mul<Output=N>
                    +   Default
    {
        let mut result = Self::default();
        for row in 0..3_usize {
            for col in 0..3_usize {
                let res_rows        =   (0..3_usize).filter(|r| *r != row).collect::<Vec<_>>();
                let res_cols        =   (0..3_usize).filter(|c| *c != col).collect::<Vec<_>>();
                result.0[row][col]  =   self.0[res_rows[0]][res_cols[0]] * self.0[res_rows[1]][res_cols[1]]
                                    -   self.0[res_rows[0]][res_cols[1]] * self.0[res_rows[1]][res_cols[0]];
            }
        }
        result
    }

    //  Obtain the determinant. Assume that a
    //  matrix of minors has been calculated prior.
    fn determinant(&self, minors: &Self) -> N
        where   N   :   AddAssign
                    +   Mul<Output=N>
                    +   From<i16>
    {
        let one         = N::from(1);
        let one_neg     = N::from(-1);
        let mut result  = N::from(0);
        for col in 0..3_usize {
            let sign     =  if col == 1 { one_neg } else { one };
            result      +=  sign * self.0[0][col] * minors.0[0][col];
        }
        result
    }

    /// Computes the inverse using the method of adjuncts,
    /// as long as the matrix has a non-zero determinant.
    pub fn inverse(&self) -> Result<Self, MatrixInversionError>
        where   N   :   PartialEq
                    +   AddAssign
                    +   Sub<Output=N> + Mul<Output=N> + Div<Output=N>
                    +   From<i16>
                    +   Default
    {
        let zero        = N::from(0);
        let one         = N::from(1);
        let one_neg     = N::from(-1);
        let minors      = self.minors();
        let det         = self.determinant(&minors);
        if det == zero {
            Err(MatrixInversionError::NoInverse)
        }
        else {
            Ok({
                let mut result = Matrix3x3::default();
                for row in 0..3_usize {
                    for col in 0..3_usize {
                        //  Sign follows a checkerboard pattern -- applied to the minors,
                        //  it yields the matrix of cofactors
                        let sign            = if (row + col) % 2 == 1 { one_neg } else { one };
                        //  And we applied that but transpose (which yields the adjoint)
                        //  and divide by the determinant, producing the inverse.
                        result.0[col][row]  = minors.0[row][col] * sign / det;
                    }
                }
                result
            })
        }
    }
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    #[wasm_bindgen_test]
    fn creation_from_transform() {
        let t = Transform::<f32>::new_from_set_transform_args(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let m = Matrix3x3::new_from_transform(&t);
        assert_eq!(m, Matrix3x3([[1.0, 2.0, 0.0], [3.0, 4.0, 0.0], [5.0, 6.0, 1.0]]));
    }

    #[wasm_bindgen_test]
    fn inverse() {
        let mut t   = Transform::<f32>::new_from_set_transform_args(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let mut m   = Matrix3x3::new_from_transform(&t);
        let minors  = m.minors();
        assert_eq!(minors, Matrix3x3([[4.0, 3.0, -2.0], [2.0, 1.0, -4.0], [0.0, 0.0, -2.0]]));
        let det = m.determinant(&minors);
        assert_eq!(det, -2.0);
        assert_eq!(m.inverse().unwrap(), Matrix3x3([[-2.0, 1.0, 0.0], [3.0/2.0, -1.0/2.0, 0.0], [1.0, -2.0, 1.0]]));
        t           = Transform::<f32>::new_from_set_transform_args(-2.0, 1.0, 1.0, -1.0, 5.0, -5.0);
        m           = Matrix3x3::new_from_transform(&t);
        assert_eq!(m.inverse().unwrap(), Matrix3x3([[-1.0, -1.0, 0.0], [-1.0, -2.0, 0.0], [0.0, -5.0, 1.0]]));
    }
}
