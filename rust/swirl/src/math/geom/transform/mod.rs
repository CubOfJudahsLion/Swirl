//! Module  :   Transform
//! Purpose :   Affine Transformations for 2D objects.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Modified:   Sat 09/11/2021 09:07 AM


use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use std::cmp::PartialEq;
use std::convert::From;
use std::ops::{Neg, Add, AddAssign, Sub, Mul, Div, MulAssign};
use std::ptr::copy;
use crate::math::Floating;

mod matrix3x3;
use matrix3x3::*;


/// # Description
/// `Transform` represents an **Affine Transformation** in a cartesian
/// 2-dimensional coordinate system. It allows for translations, rotations,
/// skewing and scaling, and to compose said operations.
/// # Properties
/// `Transfrom` is **composable* and *associative*, allowing for cumulative
/// transforms inside nested frames of reference. This allows for the
/// construction of *scene graphs* with multiple containers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transform<N>(
    //  Internal representation follows a
    //  *row-first* order.
    pub [[N; 2]; 3],
    //  A complete matrix would contain an6
    //  extra rightmost column ([0 0 1]
    //  transposed) but it is assumed.
    //
    //  In the same vein, `Point` will be
    //  seen as a to be a *row* vector of 3
    //  elements with an assumed third element
    //  equal to 1. These things were optimized
    //  away.
    //
    //  The internal representation is
    //  crate-public for `Point` to implement
    //  multiplication with it.
);


impl<N> Transform<N> {
    /// Takes individual arguments, in the same order as
    /// the `CanvasRenderingContext2d.setTransform()` method.
    pub fn new_from_set_transform_args(a: N, b: N, c: N, d: N, e: N, f: N) -> Self {
        Transform([[a, b], [c, d], [e, f]])
    }
}


impl<N: Copy> Transform<N> {
    /// Takes its arguments as an actual matrix in *row-first* order.
    /// In memory, this is the same order as specified by
    /// `CanvasRenderingContext2d.setTransform()` argument order.
    pub fn new_from_row_matrix(src_matrix: &[[N; 2]; 3]) -> Self {
        Transform(
            [[src_matrix[0][0], src_matrix[0][1]],
             [src_matrix[1][0], src_matrix[1][1]],
             [src_matrix[2][0], src_matrix[2][1]]]
        )
    }

    /// Takes its arguments as an actual matrix in *column-first* order.
    /// The matrix is equivalent to the tranpose of the one used in
    /// [`new_from_row_matrix`][new_from_row_matrix].
    pub fn new_from_column_matrix(src_matrix: &[[N; 3]; 2]) -> Self {
        Transform(
            [[src_matrix[0][0], src_matrix[1][0]],
             [src_matrix[0][1], src_matrix[1][1]],
             [src_matrix[0][2], src_matrix[1][2]]]
        )
    }

    /// Copy constructor.
    pub fn new_from_transform(src: &Transform<N>) -> Self {
        Transform(
            [[src.0[0][0], src.0[1][0]],
             [src.0[1][0], src.0[1][1]],
             [src.0[2][0], src.0[2][1]]]
        )
    }

    /// Copies another transform into the calling one.
    pub fn copy_from(&mut self, src: &Transform<N>) -> &mut Self {
        unsafe {
            copy(src as *const Transform<N>, self as *mut Transform<N>, 1);
        }
        self
    }

    /// Creates a new transform where each new
    /// value is mapped from `self`.
    pub fn map<O, F: FnMut(N) -> O>(&self, mut f: F) -> Transform<O> {
        Transform(
            [[f(self.0[0][0]), f(self.0[0][1])],
             [f(self.0[1][0]), f(self.0[1][1])],
             [f(self.0[2][0]), f(self.0[2][1])]]
        )
    }

    /// Especialization of `map`. Creates a new transform
    /// with a different type.
    #[inline(always)]
    pub fn map_type<O: From<N>>(&self) -> Transform<O> {
        self.map(<O as From<N>>::from)
    }

    /// Constructor that sets every cell to the same value.
    pub fn new_with_all_set_to(v: N) -> Self {
        Transform([[v, v], [v, v], [v, v]])
    }

    /// Sets all values in the transform to a same value.
    pub fn set_all_to(&mut self, v: N) -> &mut Self {
        for row in 0..3 {
            for col in 0..2 {
                self.0[row][col] = v;
            }
        }
        self
    }
}


impl<N: From<i16> + Copy> Transform<N> {
    /// Generate an identity matrix.
    pub fn new_identity() -> Self {
        let zero    = <N as From<i16>>::from(0);
        let one     = <N as From<i16>>::from(1);
        Transform(
            [[one,  zero],
             [zero, one ],
             [zero, zero]]
        )
    }

    /// Generate a scaling transform that's uniform for x and y.
    #[inline(always)]
    pub fn new_uniform_scale(v: N) -> Self {
        Self::new_scale(v, v)
    }

    /// Set to a uniform scaling transform.
    #[inline(always)]
    pub fn set_to_uniform_scale(&mut self, v: N) -> &mut Self {
        self.set_to_scale(v, v)
    }

    //  One we'll keep to ourselves: conversion from a `Matrix3x3`
    fn new_from_matrix3x3(src: &Matrix3x3<N>) -> Result<Self, MatrixInversionError>
        where N: PartialEq
    {
        let zero    = N::from(0);
        let one     = N::from(1);
        if src.0[0][2] == zero && src.0[1][2] == zero && src.0[2][2] == one {
            Ok(Transform(
                [[src.0[0][0], src.0[0][1]],
                 [src.0[1][0], src.0[1][1]],
                 [src.0[2][0], src.0[2][1]]]
            ))
        }
        else {
            Err(MatrixInversionError::InverseNotAffineTransform)
        }
    }

    /// Set the calling matrix to identity.
    pub fn set_to_identity(&mut self) -> &mut Self {
        let zero        = <N as From<i16>>::from(0);
        let one         = <N as From<i16>>::from(1);
        self.0[0][0]    = one;
        self.0[0][1]    = zero;
        self.0[1][0]    = zero;
        self.0[1][1]    = one;
        self.0[2][0]    = zero;
        self.0[2][1]    = zero;
        self
    }

    /// Generate a translation transform.
    pub fn new_translation(x: N, y: N) -> Self {
        let zero    = <N as From<i16>>::from(0);
        let one     = <N as From<i16>>::from(1);
        Transform(
            [[one,  zero],
             [zero, one ],
             [x,    y   ]]
        )
    }

    /// Set to a translation transform. Use zero to avoid
    /// translation in a particular dimension.
    pub fn set_to_translation(&mut self, x: N, y: N) -> &mut Self {
        let zero        = <N as From<i16>>::from(0);
        let one         = <N as From<i16>>::from(1);
        self.0[0][0]    = one;
        self.0[0][1]    = zero;
        self.0[1][0]    = zero;
        self.0[1][1]    = one;
        self.0[2][0]    = x;
        self.0[2][1]    = y;
        self
    }

    /// Generate a scale transform. Use 1 not to scale a dimension.
    pub fn new_scale(x: N, y: N) -> Self {
        let zero    = <N as From<i16>>::from(0);
        Transform(
            [[x,    zero],
             [zero, y   ],
             [zero, zero]]
        )
    }

    /// Set into a scale transform.
    pub fn set_to_scale(&mut self, x: N, y: N) -> &mut Self {
        let zero    = <N as From<i16>>::from(0);
        self.0[0][0] = x;
        self.0[0][1] = zero;
        self.0[1][0] = zero;
        self.0[1][1] = y;
        self.0[2][0] = zero;
        self.0[2][1] = zero;
        self
    }

    /// Generate a skewing transform. Use zero not to skew in that dimension.
    pub fn new_skew(x: N, y: N) -> Self {
        let zero    = <N as From<i16>>::from(0);
        let one     = <N as From<i16>>::from(1);
        Transform(
            [[one,  y   ],
             [x,    one ],
             [zero, zero]]
        )
    }

    /// Set to a skewing transform.
    pub fn set_to_skew(&mut self, x: N, y: N) -> &mut Self {
        let zero        = <N as From<i16>>::from(0);
        let one         = <N as From<i16>>::from(1);
        self.0[0][0]    = one;
        self.0[0][1]    = y;
        self.0[1][0]    = x;
        self.0[1][1]    = one;
        self.0[2][0]    = zero;
        self.0[2][1]    = zero;
        self
    }
}


impl<N: Floating + From<i16> + Neg<Output=N> + Copy> Transform<N> {
    pub fn new_rotation(rads: N) -> Self {
        let zero    = <N as From<i16>>::from(0);
        let cos     = rads.cos();
        let sin     = rads.sin();
        Transform(
            [[ cos, sin ],
             [-sin, cos ], 
             [zero, zero]]
        )
    }

    pub fn set_to_rotation(&mut self, rot: N) -> &mut Self {
        let zero        = <N as From<i16>>::from(0);
        let cos         = rot.cos();
        let sin         = rot.sin();
        self.0[0][0]    = cos;
        self.0[0][1]    = sin;
        self.0[1][0]    = -sin;
        self.0[1][1]    = cos;
        self.0[2][0]    = zero;
        self.0[2][1]    = zero;
        self
    }
}


/// Poissible errors for a matrix inversion operation.
#[derive(Clone, Copy, Debug)]
pub enum MatrixInversionError {
    /// There is no inverse matrix (i.e., the determinant is zero.)
    NoInverse,
    /// The resulting inverted matrix cannot be converted back
    /// to an affine transformation (i.e., a `Transform`) as
    /// the rightmost columnt is not *[0, 0, 1]ᵀ*.
    InverseNotAffineTransform,
}


impl<N> Transform<N>
    where   N   :   PartialEq
                +   AddAssign
                +   Sub<Output=N> + Mul<Output=N> + Div<Output=N>
                +   From<i16>
                +   Default
                +   Copy
{
    pub fn inverse(&self) -> Result<Self, MatrixInversionError> {
        Matrix3x3::new_from_transform(self)
            .inverse()
            .and_then(|inv|
                Self::new_from_matrix3x3(&inv)
            )
    }
}


impl<N: Copy> Transform<N>
    where   f64 :   From<N>
{
    #[inline]
    pub fn apply(&self, canvas: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        canvas.set_transform(
            <f64>::from(self.0[0][0]),
            <f64>::from(self.0[0][1]),
            <f64>::from(self.0[1][0]),
            <f64>::from(self.0[1][1]),
            <f64>::from(self.0[2][0]),
            <f64>::from(self.0[2][1])
        )
    }
}


macro_rules! map_prop_to_matrix {
    ($name:ident / $name_set:ident = ( $row:literal, $column:literal )) => {
        #[inline(always)]
        pub fn $name(&self) -> N { self.0[$row][$column] }
        #[inline(always)]
        pub fn $name_set(&mut self, v: N) -> &mut Self {
            self.0[$row][$column] = v;
            self
        }
    };
}

impl<N: Copy> Transform<N> {
    map_prop_to_matrix!{x_scale / set_x_scale = (0, 0)}
    map_prop_to_matrix!{y_skew  / set_y_skew  = (0, 1)}
    map_prop_to_matrix!{x_skew  / set_x_skew  = (1, 0)}
    map_prop_to_matrix!{y_scale / set_y_scale = (1, 1)}
    map_prop_to_matrix!{x_trans / set_x_trans = (2, 0)}
    map_prop_to_matrix!{y_trans / set_y_trans = (2, 1)}
}


macro_rules! impl_trans_mul {
    (@mktype r) => { &Transform<N> };
    (@mktype v) => {  Transform<N> };
    ($lhs_rv:ident, $rhs_rv:ident) => {
        impl<N> Mul<impl_trans_mul!(@mktype $rhs_rv)> for impl_trans_mul!(@mktype $lhs_rv)
            where   N   : Add<Output=N> + Mul<Output=N> + Copy
        {
            type Output = Transform<N>;
            fn mul(self, rhs: impl_trans_mul!(@mktype $rhs_rv)) -> Self::Output {
                //  This is more error-prone than a loop, but faster --
                //  and considering the matrixes are not truly square,
                //  multiplication would involve extra ifs. That
                //  wouldn´t do -- this needs to run *blindingly fast*.
                Transform(
                    [   [   self.0[0][0] * rhs.0[0][0] + self.0[0][1] * rhs.0[1][0]
                        ,   self.0[0][0] * rhs.0[0][1] + self.0[0][1] * rhs.0[1][1]
                        ],

                        [   self.0[1][0] * rhs.0[0][0] + self.0[1][1] * rhs.0[1][0]
                        ,   self.0[1][0] * rhs.0[0][1] + self.0[1][1] * rhs.0[1][1]
                        ],

                        [   self.0[2][0] * rhs.0[0][0] + self.0[2][1] * rhs.0[1][0] + rhs.0[2][0]
                        ,   self.0[2][0] * rhs.0[0][1] + self.0[2][1] * rhs.0[1][1] + rhs.0[2][1]
                        ]
                    ]
                )
            }
        }
    };
}

impl_trans_mul!{v, v}
impl_trans_mul!{v, r}
impl_trans_mul!{r, v}
impl_trans_mul!{r, r}


macro_rules! impl_mul_assign {
    (@gen_type r) => { &Transform<N> };
    (@gen_type v) => {  Transform<N> };
    ($rv:ident) => {
        impl<N> MulAssign<impl_mul_assign!(@gen_type $rv)> for Transform<N>
            where   N   :   Add<Output=N> + Mul<Output=N>
                        +   Copy
        {
            fn mul_assign(&mut self, rhs: impl_mul_assign!(@gen_type $rv)) {
                let mut temp    = self.0[0][0] * rhs.0[0][0] + self.0[0][1] * rhs.0[1][0];
                self.0[0][1]    = self.0[0][0] * rhs.0[0][1] + self.0[0][1] * rhs.0[1][1];
                self.0[0][0]    = temp;
                temp            = self.0[1][0] * rhs.0[0][0] + self.0[1][1] * rhs.0[1][0];
                self.0[1][1]    = self.0[1][0] * rhs.0[0][1] + self.0[1][1] * rhs.0[1][1];
                self.0[1][0]    = temp;
                temp            = self.0[2][0] * rhs.0[0][0] + self.0[2][1] * rhs.0[1][0] + rhs.0[2][0];
                self.0[2][1]    = self.0[2][0] * rhs.0[0][1] + self.0[2][1] * rhs.0[1][1] + rhs.0[2][1];
                self.0[2][0]    = temp;
            }
        }
    };
}

impl_mul_assign!{r}
impl_mul_assign!{v}


#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn identity_and_basic_constructors() {
        let mut t1 = Transform::<f64>::new_identity();
        assert_eq!(t1, Transform::<f64>::new_from_column_matrix(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]));
        assert_eq!(t1, Transform::<f64>::new_from_row_matrix(&[[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]));
        assert_eq!(t1, Transform::new_from_set_transform_args(1.0, 0.0, 0.0, 1.0, 0.0, 0.0));
        for i in 0..3 {
            for j in 0..2 {
                t1.0[i][j] = 1000.0 * js_sys::Math::random();
            }
        }
        t1.set_to_identity();
        assert_eq!(t1, Transform::<f64>::new_from_row_matrix(&[[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]));
    }

    #[wasm_bindgen_test]
    fn specific_transformation_constructors_and_setters() {
        let mut t1 = Transform::<f64>::new_from_set_transform_args(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(t1, Transform::new_from_row_matrix(&[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]));
        assert_eq!(t1, Transform::new_from_column_matrix(&[[1.0, 3.0, 5.0], [2.0, 4.0, 6.0]]));
        t1 = Transform::new_translation(1.0, 2.0);
        let mut t2 = Transform::new_with_all_set_to(7.0);
        t2.set_to_translation(1.0, 2.0);
        assert_eq!(t1, t2);
        assert_eq!(t1, Transform::new_from_set_transform_args(1.0, 0.0, 0.0, 1.0, 1.0, 2.0));
        t1 = Transform::<f64>::new_scale(3.0, 4.0);
        t2.set_to_scale(3.0, 4.0);
        assert_eq!(t1, t2);
        assert_eq!(t1, Transform::new_from_set_transform_args(3.0, 0.0, 0.0, 4.0, 0.0, 0.0));
        t1 = Transform::<f64>::new_skew(5.0, 6.0);
        t2.set_to_skew(5.0, 6.0);
        assert_eq!(t1, t2);
        assert_eq!(t1, Transform::new_from_set_transform_args(1.0, 6.0, 5.0, 1.0, 0.0, 0.0));
        let rand_angle = 2.0 * std::f64::consts::PI * js_sys::Math::random();
        let cos = rand_angle.cos();
        let sin = rand_angle.sin();
        t1 = Transform::<f64>::new_rotation(rand_angle);
        t2.set_to_rotation(rand_angle);
        assert_eq!(t1, t2);
        assert_eq!(t1, Transform::new_from_set_transform_args(cos, sin, -sin, cos, 0.0, 0.0));
        t1 = Transform::new_uniform_scale(1.5);
        t2.set_to_uniform_scale(1.5);
        assert_eq!(t1, t2);
        assert_eq!(t1, Transform::new_from_set_transform_args(1.5, 0.0, 0.0, 1.5, 0.0, 0.0));
        t1 = Transform::new_with_all_set_to(7.0);
        t2.set_all_to(7.0);
        assert_eq!(t1, t2);
        assert_eq!(t1, Transform::new_from_set_transform_args(7.0, 7.0, 7.0, 7.0, 7.0, 7.0));
    }

    #[wasm_bindgen_test]
    fn individual_field_setters_and_getters() {
        let mut t = Transform::<i16>::new_with_all_set_to(0);
        t.set_x_scale(1);
        assert_eq!(t.x_scale(), 1);
        assert_eq!(t, Transform::new_from_set_transform_args(1, 0, 0, 0, 0, 0));
        t.set_y_skew(2);
        assert_eq!(t.y_skew(), 2);
        assert_eq!(t, Transform::new_from_set_transform_args(1, 2, 0, 0, 0, 0));
        t.set_x_skew(3);
        assert_eq!(t.x_skew(), 3);
        assert_eq!(t, Transform::new_from_set_transform_args(1, 2, 3, 0, 0, 0));
        t.set_y_scale(4);
        assert_eq!(t.y_scale(), 4);
        assert_eq!(t, Transform::new_from_set_transform_args(1, 2, 3, 4, 0, 0));
        t.set_x_trans(5);
        assert_eq!(t.x_trans(), 5);
        assert_eq!(t, Transform::new_from_set_transform_args(1, 2, 3, 4, 5, 0));
        t.set_y_trans(6);
        assert_eq!(t.y_trans(), 6);
        assert_eq!(t, Transform::new_from_set_transform_args(1, 2, 3, 4, 5, 6));
    }

    #[wasm_bindgen_test]
    fn mapping() {
        let t1 = Transform::<f64>::new_from_set_transform_args(1.0, 1.5, 2.0, 2.5, 3.0, 3.5);
        assert_eq!(t1.map(|v| v / 5.0), Transform::new_from_set_transform_args(0.2, 0.3, 0.4, 0.5, 0.6, 0.7));
        let t2 = Transform::<i32>::new_from_set_transform_args(1, 2, 3, 4, 5, 6);
        assert_eq!(t2.map_type::<f64>(), Transform::<f64>::new_from_set_transform_args(1.0, 2.0, 3.0, 4.0, 5.0, 6.0));
    }

    #[wasm_bindgen_test]
    fn copy_and_copy_constructor() {
        let mut t1 = Transform::<f32>::new_identity();
        let t2 = Transform::<f32>::new_with_all_set_to(7.0);
        t1.copy_from(&t2);
        assert_eq!(t1, t2);
        assert_eq!(t1, Transform::new_from_set_transform_args(7.0, 7.0, 7.0, 7.0, 7.0, 7.0));
        let t3 = Transform::<f32>::new_from_transform(&t2);
        assert_eq!(t2, t3);
    }

    #[wasm_bindgen_test]
    fn transform_composition() {
        //  Formally, a transform is a 3x3 matrix, where the last column is always [0, 0, 1]ᵀ.
        //  I removed the last row and instead of using loops, I coded every result value
        //  manually. It's not just more economic -- it's also faster.
        let mut t1 = Transform::<i16>::new_from_set_transform_args(1, 2, 3, 4, 5, 6);
        let t2 = Transform::<i16>::new_from_set_transform_args(7, 8, 9, 10, 11, 12);
        let product = Transform::<i16>::new_from_row_matrix(&[[25, 28], [57, 64], [100, 112]]);
        assert_eq!(&t1 * &t2, product);
        t1 *= t2;
        assert_eq!(t1, product);
    }

    #[wasm_bindgen_test]
    fn inverse_remapping() {
        let t = Transform::<f32>::new_from_set_transform_args(-1.0, 2.0, -3.0, 4.0, -5.0, 6.0);
        assert_eq!(t.inverse().unwrap(), Transform::new_from_set_transform_args(2.0, -1.0, 3.0/2.0, -1.0/2.0, 1.0, -2.0));
    }
}
