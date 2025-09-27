//! Module  :   Transformable
//! Purpose :   A trait for object that can undergo *Affine Transformations*.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   2021-02-22 14:51
//! Modified:   Sat 09/11/2021 09:14 AM


use std::cmp::PartialEq;
use std::ops::{Neg, Add, Mul};

use crate::math::numeric::Floating;
use crate::math::geom::point::Point;
use crate::math::geom::transform::Transform;

use super::transformable_state::TransformableState;


/// An (abstract) base object that isn't anything other than
/// `Transformable`, though it contains nothing.
/// It delegates functionality through containtment and
/// macro code injection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseTransformable<N> {
    pub(crate) state_       : TransformableState,       //  Update flag
    //pub(crate) dirt_hook    : Rc<Box<dyn FnMut(bool)>>, //  A function triggered by any update
                                                        //  of the update flag.
    pub(crate) pivot        : Point<i16>,               //  Cached i16 coord
    pub(crate) p            : Option<Transform<N>>,     //  Actual pivot transform
    pub(crate) k            : Option<Transform<N>>,     //  sKewing transform
    pub(crate) s            : Option<Transform<N>>,     //  Scaling transform
    pub(crate) rot          : N,                        //  Separately kept rotation angle
    pub(crate) trans        : Point<i16>,               //  Separately kept translation
    pub(crate) rt           : Option<Transform<N>>,     //  Rotation+Translation
                                                        //    (they are trivially combined.)
    pub(crate) total        : Transform<N>,             //  Cumulative transform
}


impl<N> BaseTransformable<N>
    where N: Default + From<i16> + Copy
{
    fn default() -> Self {
        BaseTransformable {
            state_      :   TransformableState::GloballyUpdatedWithInverse,
            pivot       :   Point::default(),
            p           :   None,
            k           :   None,
            s           :   None,
            rot         :   N::default(),
            trans       :   Point::default(),
            rt          :   None,
            total       :   Transform::<N>::new_identity(),
        }
    }
}


impl<N> BaseTransformable<N>
    where N: From<i16> + Default + Copy
{
    #[inline(always)]
    pub fn new() -> Self
        where   N: Default
    {
        Self::default()
    }
}


impl<N> BaseTransformable<N>
    where N :   From<i16>
            +   PartialEq
            +   Add<Output=N> + Mul<Output=N> + Neg<Output=N>
            +   Floating
            +   Copy
{

    pub fn new_with_values(
        x_pivot     :   i16,
        y_pivot     :   i16,
        x_skew      :   N,
        y_skew      :   N,
        x_scale     :   N,
        y_scale     :   N,
        rotation    :   N,
        x           :   i16,
        y           :   i16
    ) -> Self
    {
        let zero    =   <N as From<i16>>::from(0);
        let one     =   <N as From<i16>>::from(1);
        let pivot   =   Point::new(x_pivot, y_pivot);
        let p       =   if x_pivot == 0 && y_pivot == 0 {
                            None
                        }
                        else {
                            Some(Transform::new_translation((-x_pivot).into(), (-y_pivot).into()))
                        };
        let k       =   if x_skew == zero && y_skew == zero {
                            None
                        }
                        else {
                            Some(Transform::new_skew(x_skew, y_skew))
                        };
        let s       =   if x_scale == one && y_scale == one {
                            None
                        }
                        else {
                            Some(Transform::new_scale(x_scale, y_scale))
                        };
        let rot     =   rotation;
        let trans   =   Point::new(x, y);
        let rt      =   if rotation == zero && x == 0 && y == 0 {
                            None
                        }
                        else {
                            Some(
                                    Transform::new_rotation(rotation)
                                *   Transform::new_translation(x.into(), y.into())
                            )
                        };
        let total   =   [&k, &s, &rt]
            .iter()
            .filter_map(|m| m.as_ref())
            .fold(
                p.clone().unwrap_or_else(|| Transform::new_identity()),
                |accum, curr| accum * curr
            );
        BaseTransformable {
            state_      : TransformableState::GloballyUpdatedWithInverse,
            pivot, p,
            k, s,
            rot, trans, rt,
            total,
        }
    }

    #[inline(always)]
    pub(crate) fn recalc_pivot(this: &mut BaseTransformable<N>) {
        this.p  = if this.pivot.x == 0 && this.pivot.y == 0
        {
            None
        }
        else {
            Some(Transform::<N>::new_translation(
                (-this.pivot.x).into(),
                (-this.pivot.y).into()
            ))
        };
    }

    #[inline(always)]
    pub(crate) fn recalc_skew(this: &mut BaseTransformable<N>, kx: N, ky: N) {
        let zero    =   <N as From<i16>>::from(0);
        this.k      =   if      kx == zero
                            &&  ky == zero
        {
            None
        }
        else {
            Some(Transform::<N>::new_skew(kx, ky))
        };
    }

    #[inline(always)]
    pub(crate) fn recalc_scale(this: &mut BaseTransformable<N>, x_scale: N, y_scale: N) {
        let one =   <N as From<i16>>::from(1);
        this.s  =   if      x_scale == one
                        &&  y_scale == one
        {
            None
        }
        else {
            Some(Transform::<N>::new_scale(x_scale, y_scale))
        };
    }

    #[inline(always)]
    pub(crate) fn recalc_rot_trans(this: &mut BaseTransformable<N>) {
        let zero    =   <N as From<i16>>::from(0);
        this.rt     =   if      this.rot == zero
                            &&  this.trans.x == 0
                            &&  this.trans.y == 0
        {
            None
        }
        else {
            let mut res = Transform::<N>::new_rotation(this.rot);
            res
                .set_x_trans(this.trans.x.into())
                .set_y_trans(this.trans.y.into());
            Some(res)
        };
    }
}


/// A trait to embed any type with `Transformable` functionality
/// through having a `BaseTransformable`.
pub trait HasBaseTransformable<N> {
    fn base_transformable(&self) -> &BaseTransformable<N>;
    fn base_transformable_mut(&self) -> &mut BaseTransformable<N>;
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    pub fn transform_diff(t1: &Transform<f32>, t2: &Transform<f32>) -> f32 {
        let mut accum = 0.0_f32;
        for row in 0..3 {
            for col in 0..2 {
                accum += f32::abs(t1.0[row][col] - t2.0[row][col]);
            }
        }
        accum
    }

    #[wasm_bindgen_test]
    fn constructors() {
        let mut t = BaseTransformable::<f32>::new();
        assert_eq!(
            t,
            BaseTransformable {
                state_      :   TransformableState::GloballyUpdatedWithInverse,
                pivot       :   Point::new(0, 0),
                p           :   None,
                k           :   None,
                s           :   None,
                rot         :   0.0,
                trans       :   Point::new(0, 0),
                rt          :   None,
                total       :   Transform::new_from_set_transform_args(1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            }
        );
        //
        //${1:x_pivot: i16}, ${2:y_pivot: i16}, ${3:x_skew: N}, ${4:y_skew: N}, ${5:x_scale: N}, ${6:y_scale: N}, ${7:rotation: N}, ${8:x: i16}, ${9:y: i16}
        t = BaseTransformable::new_with_values(0, 0, 0.0, 0.0, 1.0, 1.0, 0.0, 0, 0);
        assert_eq!(
            t,
            BaseTransformable {
                state_      :   TransformableState::GloballyUpdatedWithInverse,
                pivot       :   Point::new(0, 0),
                p           :   None,
                k           :   None,
                s           :   None,
                rot         :   0.0,
                trans       :   Point::new(0, 0),
                rt          :   None,
                total       :   Transform::new_identity(),
            }
        );
        //
        t = BaseTransformable::new_with_values(1, 2, 3.0, 4.0, 5.0, 6.0, 7.0, 8, 9);
        let cos = f32::cos(7.0);
        let sin = f32::sin(7.0);
        //
        assert!(t.state_ == TransformableState::GloballyUpdatedWithInverse);
        assert_eq!(t.pivot, Point::new(1, 2));
        assert_eq!(t.p, Some(Transform::new_translation(-1.0, -2.0)));
        assert_eq!(t.k, Some(Transform::new_skew(3.0, 4.0)));
        assert_eq!(t.s, Some(Transform::new_scale(5.0, 6.0)));
        assert_eq!(t.rot, 7.0);
        assert_eq!(t.trans, Point::new(8, 9));
        assert_eq!(t.rt, Some(Transform::new_from_set_transform_args(cos, sin, -sin, cos, 8.0, 9.0)));
        assert!(
            transform_diff(
                &t.total,
                &Transform::<f32>::new_from_set_transform_args(
                    -11.99816709753441,
                     21.37858709783325,
                      7.366614222836835,
                     14.37821250684166,
                      5.264938651860744,
                    -41.13501211151658
                )
            ) < 0.0001
        );
    }
}
