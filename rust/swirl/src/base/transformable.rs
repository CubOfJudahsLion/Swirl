//! Module  :   Transformable
//! Purpose :   A trait for object that can undergo *Affine Transformations*.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   2021-02-22 14:51
//! Modified:   Mon 07/04/2022 05:38 PM


use std::cmp::PartialEq;
use std::default::Default;
use std::ops::{Neg, Add, Mul};

use crate::math::numeric::Floating;
use crate::math::geom::transform::Transform;

use super::transformable_state::*;
use super::base_transformable::*;


/// A structure that preserves data for
/// transform state.
#[derive(Clone, Debug)]
pub struct TransformState<N> {
    pivot_      : (i16, i16),
    skew_       : (N, N),
    rotation_   : N,
    scale_      : (N, N),
    trans_      : (i16, i16),
}


/// # Transformable Element
///
/// _An element that supports all classical transformations
/// in two dimensions._
///
/// ## Description
/// Graphical elements in a scene graph are typically capable
/// of being transformed in a number of ways. Underlying
/// these tranformations is a framework of composable matrices
/// known as
/// [Affine Transformations](https://en.wikipedia.org/wiki/Affine_transformation),
/// a standard *Linear Algebra* framework for both 2D and 3D
/// scene graphs. Any transformation -- be it a translation,
/// rotation, skewing or scale -- can be represented as a
/// matrix, and through matrix multiplication, **composed** with
/// other transforms. Composability is what allows element
/// nesting, among other useful properties.
///
/// ## Two-stage mechanism
/// The final (local) transform needs to be computed from
/// individual components such as `x_scale` or `rotation`. It
/// does not happen automatically every time said individual
/// components is modified; that would be wasting computing
/// power. Instead, modifications cause a "state" flag to be
/// set, and the *rendering* process computes the transform
/// when it needs it. The state of the flag can be queried
/// through the `state()` method. However, if users need
/// to force the update for any reason, this can be done by
/// calling `update_transform()`.
///
/// ## Order of operations
/// Matrix multiplication is **not** commutative, so the order
/// of operations matters. Literature often suggests to use a
/// scale - rotate - translate order (**SRT**.) However, we
/// include the additional operations of *pivoting* (i.e.,
/// setting a different origin for the element) and *skewing*
/// (translating a point in one axis depending on its position
/// in another.) Our actual order of operations is **PKSRT**
/// (where '**P**' stands for pivoting and '**K**' for
/// skewing.) The mnemonic "*PiKe SoRTie*" can be used to
/// remember it.
/// The resultant matrix is called the **Local Transform**
/// for the object. For a complete, final **World Transform**,
/// the Local Transform has to be composed with cumulative
/// transforms of its containers.
//
// /// ## Manual transforms
// /// If the order of operations is not the desired one, users
// /// can directly set the local transform using the
// /// `set_transform()` method. `set_transform()` takes a
// /// `math::geom::transform::Transform`, which can be
// /// created and combined from other transforms in any
// /// order the user sees fit. Then it sets the state flag,
// /// avoiding computation from individual components.
// /// If the user calls the `set_state` method with a `true`
// /// value, or changes one of the component values (i.e.,
// /// calling any `set_` method), the state flag will be set
// /// again and the next rendering pass will compute the
// /// transform from components.
pub trait Transformable<N>: HasBaseTransformable<N>
    where   N   :   Default
                +   From<i16> + Floating
                +   PartialEq
                +   Add<Output=N> + Mul<Output=N> + Neg<Output=N>
                +   Copy
{
    #[inline]
    fn x_pivot(&self) -> i16 {
        self.base_transformable().pivot.x
    }

    fn set_x_pivot(&mut self, px: i16) {
        let mut this    = self.base_transformable_mut();
        this.pivot.x    = px;
        BaseTransformable::<N>::recalc_pivot(this);
        self.set_state(TransformableState::Dirty);
    }

    #[inline]
    fn y_pivot(&self) -> i16 {
        self.base_transformable().pivot.y
    }

    fn set_y_pivot(&mut self, py: i16) {
        let this        = self.base_transformable_mut();
        this.pivot.y    = py;
        BaseTransformable::<N>::recalc_pivot(this);
        self.set_state(TransformableState::Dirty);
    }

    fn set_pivot(&mut self, px: i16, py: i16) {
        let this        = self.base_transformable_mut();
        this.pivot.x    = px;
        this.pivot.y    = py;
        BaseTransformable::<N>::recalc_pivot(this);
        self.set_state(TransformableState::Dirty);
    }

    fn x_skew(&self) -> N {
        self.base_transformable().k
            .as_ref()
            .map(|k| k.x_skew())
            .unwrap_or_else(|| <N as From<i16>>::from(0))
    }

    fn set_x_skew(&mut self, kx: N) {
        let this    =   self.base_transformable_mut();
        let ky          =   this
            .k
            .as_ref()
            .map(|k| k.y_skew())
            .unwrap_or_else(|| <N as From<i16>>::from(0));
        BaseTransformable::<N>::recalc_skew(this, kx, ky);
        self.set_state(TransformableState::Dirty);
    }

    fn y_skew(&self) -> N {
        self.base_transformable().k
            .as_ref()
            .map(|k| k.y_skew())
            .unwrap_or_else(|| <N as From<i16>>::from(0))
    }

    fn set_y_skew(&mut self, ky: N) {
        let this    = self.base_transformable_mut();
        let kx      = this.k
            .as_ref()
            .map(|k| k.x_skew())
            .unwrap_or_else(|| <N as From<i16>>::from(0));
        BaseTransformable::<N>::recalc_skew(this, kx, ky);
        self.set_state(TransformableState::Dirty);
    }

    fn set_skew(&mut self, kx: N, ky: N) {
        let this    = self.base_transformable_mut();
        BaseTransformable::<N>::recalc_skew(this, kx, ky);
        self.set_state(TransformableState::Dirty);
    }

    fn x_scale(&self) -> N {
        self.base_transformable().s
            .as_ref()
            .map(|s| s.x_scale())
            .unwrap_or_else(|| <N as From<i16>>::from(1))
    }

    fn set_x_scale(&mut self, x_scale: N) {
        let this    =   self.base_transformable_mut();
        let y_scale =   this.s
            .as_ref()
            .map(|s| s.y_scale())
            .unwrap_or_else(|| <N as From<i16>>::from(1));
        BaseTransformable::<N>::recalc_scale(this, x_scale, y_scale);
        self.set_state(TransformableState::Dirty);
    }

    fn y_scale(&self) -> N {
        self.base_transformable().s
            .as_ref()
            .map(|s| s.y_scale())
            .unwrap_or_else(|| <N as From<i16>>::from(1))
    }

    fn set_y_scale(&mut self, y_scale: N) {
        let this        =   self.base_transformable_mut();
        let x_scale     =   this.s
            .as_ref()
            .map(|s| s.x_scale())
            .unwrap_or_else(|| <N as From<i16>>::from(1));
        BaseTransformable::<N>::recalc_scale(this, x_scale, y_scale);
        self.set_state(TransformableState::Dirty);
    }

    fn set_scale(&mut self, x_scale: N, y_scale: N) {
        let this    = self.base_transformable_mut();
        BaseTransformable::<N>::recalc_scale(this, x_scale, y_scale);
        self.set_state(TransformableState::Dirty);
    }

    fn set_uniform_scale(&mut self, scale: N) {
        let this    = self.base_transformable_mut();
        BaseTransformable::<N>::recalc_scale(this, scale, scale);
        self.set_state(TransformableState::Dirty);
    }

    fn is_scale_uniform(&self) -> bool {
        self.base_transformable().s
            .as_ref()
            .map(|s| s.x_scale() == s.y_scale())
            .unwrap_or(true)
    }

    #[inline]
    fn rotation(&self) -> N {
        self.base_transformable().rot
    }

    fn set_rotation(&mut self, rot: N) {
        let this        =   self.base_transformable_mut();
        this.rot        =   rot;
        BaseTransformable::<N>::recalc_rot_trans(this);
        self.set_state(TransformableState::Dirty);
    }

    #[inline]
    fn x(&self) -> i16 {
        self.base_transformable().trans.x
    }

    #[inline]
    fn y(&self) -> i16 {
        self.base_transformable().trans.y
    }

    fn set_x(&mut self, x: i16) {
        let this        =   self.base_transformable_mut();
        this.trans.x    =   x;
        BaseTransformable::<N>::recalc_rot_trans(this);
        self.set_state(TransformableState::Dirty);
    }

    fn set_y(&mut self, y: i16) {
        let this        = self.base_transformable_mut();
        this.trans.y    = y;
        BaseTransformable::<N>::recalc_rot_trans(this);
        self.set_state(TransformableState::Dirty);
    }

    fn set_x_and_y(&mut self, x: i16, y: i16) {
        let this        = self.base_transformable_mut();
        this.trans.x    = x;
        this.trans.y    = y;
        BaseTransformable::<N>::recalc_rot_trans(this);
        self.set_state(TransformableState::Dirty);
    }

    #[inline(always)]
    fn set_both_x_and_y_to(&mut self, v: i16) {
        self.set_x_and_y(v, v);
    }


    //  The previous methods follow the pretense that altering the
    //  state of the `Transformable` requires a mutable reference,
    //  but the following ones allow for internal updating during
    //  rendering or calculations depending on local or world
    //  transformations.

    #[inline]
    fn state(&self) -> TransformableState {
        self.base_transformable().state_
    }

    #[inline]
    fn set_state(&self, state: TransformableState) {
        let this    = self.base_transformable_mut();
        this.state_ = state;
    }

    /// Updates (and returns) the local transform.
    fn update_transform(&self) -> &Transform<N> {
        //  Internal structure to permit mixing
        //  of references and owned objects.
        //  Note: I'm not using std::borrow::Cow
        //  because you have to implement two
        //  other traits before you can use it.
        enum RefOrObj<'a, T> {
            Ref(&'a T),
            Own(T),
        }
        //  Methods:
        impl<T: Clone> RefOrObj<'_, T> {
            //  Consume and turn into the
            //  contained object (or clone if ref)
            fn to_owned(self) -> T {
                use RefOrObj::*;
                match self {
                    Ref(r)  => r.clone(),
                    Own(o)  => o,
                }
            }
            //  Pass referernce to obj.
            fn as_ref(&self) -> &T {
                use RefOrObj::*;
                match self {
                    &Ref(r)     => r,
                    &Own(ref o) => o,
                }
            }
        }
        //  Update algorithm proper.
        let this    = self.base_transformable_mut();
        if this.state_ == TransformableState::Dirty {
            this.total  = [&this.k, &this.s, &this.rt]
                .iter()
                .fold(
                    RefOrObj::Ref(&this.p),
                    |acc, &next| {
                        match (acc.as_ref(), next) {
                            (_,            &None)           => acc,
                            (&None,        &Some(_))        => RefOrObj::Ref(next),
                            (&Some(ref a), &Some(ref b))    => RefOrObj::Own(Some(a * b)),
                        }
                    }
                )
                .to_owned()
                .unwrap_or_else(|| Transform::<N>::new_identity());
            this.state_ = TransformableState::LocallyUpdated;
        }
        &this.total
    }

    /// Returns the total (local) `Transform`.
    /// If you need to ensure that the local
    /// transform is updated, call `update_transform`
    /// instead.
    fn transform(&self) -> &Transform<N> {
        &self.base_transformable().total
    }

    // /// Allow the transform to be set by hand. Do note that
    // /// this operation doesn't alter the `state` of
    // /// the `Transformable`, which would need to be set
    // /// by hand.
    // fn set_transform(&self, t: &Transform<N>) {
    //     let this    = self.base_transformable_mut();
    //     this.total.copy_from(t);
    // }
    //

    fn save_transform(&self) -> TransformState<N> {
        TransformState {
            pivot_      : (self.x_pivot(), self.y_pivot()),
            skew_       : (self.x_skew(), self.y_skew()),
            scale_      : (self.x_scale(), self.y_scale()),
            rotation_   : self.rotation(),
            trans_      : (self.x(), self.y()),
        }
    }

    fn restore_transform(&mut self, ts: &TransformState<N>) {
        self.set_pivot(ts.pivot_.0, ts.pivot_.1);
        self.set_skew(ts.skew_.0,  ts.skew_.1);
        self.set_scale(ts.scale_.0, ts.scale_.1);
        self.set_rotation(ts.rotation_);
        self.set_x_and_y(ts.trans_.0, ts.trans_.1);
        self.update_transform();
    }
}


#[cfg(test)]
pub mod tests {
    use std::cell::RefCell;
    use wasm_bindgen_test::*;
    use super::*;
    use super::super::base_transformable::tests::transform_diff;

    #[wasm_bindgen_test]
    fn setters_and_getters() {
        use TransformableState::*;
        struct Wrap<N>(RefCell<BaseTransformable<N>>);
        impl<N> Wrap<N>
            where N :   From<i16>
                    +   PartialEq
                    +   Add<Output=N> + Mul<Output=N> + Neg<Output=N>
                    +   Floating
                    +   Copy
        {
            fn new() -> Self {
                Wrap(RefCell::new(BaseTransformable::<N>::new()))
            }
        }
        impl<N> HasBaseTransformable<N> for Wrap<N> {
            fn base_transformable(&self) -> &BaseTransformable<N> {
                unsafe { &*self.0.as_ptr() }
            }
            fn base_transformable_mut(&self) -> &mut BaseTransformable<N> {
                unsafe { &mut *self.0.as_ptr() }
            }
        }
        impl<N> Transformable<N> for Wrap<N>
            where N: Floating + From<i16>
        {}
        //
        let mut wrap = Wrap::<f32>::new();
        //  identity
        assert_eq!(wrap.transform(), &Transform::new_from_set_transform_args(1.0, 0.0, 0.0, 1.0, 0.0, 0.0));
        //  pivot
        assert!(wrap.0.borrow().p.is_none());
        assert!(wrap.state() == GloballyUpdatedWithInverse);
        assert_eq!(wrap.x_pivot(), 0);
        assert_eq!(wrap.y_pivot(), 0);
        wrap.set_x_pivot(2);
        assert!(wrap.state() == Dirty);
        assert!(wrap.0.borrow().p.is_some());
        assert_eq!(wrap.x_pivot(), 2);
        assert_eq!(wrap.y_pivot(), 0);
        wrap.set_x_pivot(0);
        assert!(wrap.0.borrow().p.is_none());
        wrap.set_y_pivot(3);
        assert!(wrap.0.borrow().p.is_some());
        assert_eq!(wrap.x_pivot(), 0);
        assert_eq!(wrap.y_pivot(), 3);
        wrap.set_y_pivot(0);
        assert!(wrap.0.borrow().p.is_none());
        wrap.set_pivot(0, 0);
        assert!(wrap.0.borrow().p.is_none());
        wrap.set_pivot(2, 3);
        assert!(wrap.0.borrow().p.is_some());
        assert_eq!(wrap.x_pivot(), 2);
        assert_eq!(wrap.y_pivot(), 3);
        assert_eq!(wrap.0.borrow().pivot.x, 2);
        assert_eq!(wrap.0.borrow().pivot.y, 3);
        assert_eq!(wrap.0.borrow().p, Some(Transform::new_from_set_transform_args(1.0, 0.0, 0.0, 1.0, -2.0, -3.0)));
        wrap.update_transform();
        assert_eq!(wrap.transform(), &Transform::new_from_set_transform_args(1.0, 0.0, 0.0, 1.0, -2.0, -3.0));
        //
        //  skewing
        assert!(wrap.0.borrow().k.is_none());
        assert!(wrap.state() == LocallyUpdated);
        assert_eq!(wrap.x_skew(), 0.0);
        assert_eq!(wrap.y_skew(), 0.0);
        wrap.set_x_skew(4.0);
        assert!(wrap.state() == Dirty);
        assert!(wrap.0.borrow().k.is_some());
        assert_eq!(wrap.x_skew(), 4.0);
        assert_eq!(wrap.y_skew(), 0.0);
        wrap.set_x_skew(0.0);
        assert!(wrap.0.borrow().k.is_none());
        wrap.set_y_skew(5.0);
        assert!(wrap.0.borrow().k.is_some());
        assert_eq!(wrap.x_skew(), 0.0);
        assert_eq!(wrap.y_skew(), 5.0);
        wrap.set_y_skew(0.0);
        assert!(wrap.0.borrow().k.is_none());
        wrap.set_x_skew(3.0);
        wrap.set_y_skew(4.0);
        assert!(wrap.0.borrow().k.is_some());
        wrap.set_skew(0.0, 0.0);
        assert_eq!(wrap.x_skew(), 0.0);
        assert_eq!(wrap.y_skew(), 0.0);
        assert!(wrap.0.borrow().k.is_none());
        wrap.set_skew(4.0, 5.0);
        assert!(wrap.0.borrow().k.is_some());
        assert_eq!(wrap.x_skew(), 4.0);
        assert_eq!(wrap.y_skew(), 5.0);
        assert_eq!(wrap.0.borrow().k, Some(Transform::new_from_set_transform_args(1.0, 5.0, 4.0, 1.0, 0.0, 0.0)));
        wrap.update_transform();
        assert_eq!(wrap.transform(), &Transform::new_from_set_transform_args(1.0, 5.0, 4.0, 1.0, -14.0, -13.0));
        //
        //  scaling
        assert!(wrap.0.borrow().s.is_none());
        assert!(wrap.state() == LocallyUpdated);
        assert_eq!(wrap.x_scale(), 1.0);
        assert_eq!(wrap.y_scale(), 1.0);
        assert!(wrap.is_scale_uniform());
        wrap.set_x_scale(6.0);
        assert!(!wrap.is_scale_uniform());
        assert!(wrap.state() == Dirty);
        assert!(wrap.0.borrow().s.is_some());
        assert_eq!(wrap.x_scale(), 6.0);
        assert_eq!(wrap.y_scale(), 1.0);
        wrap.set_x_scale(1.0);
        assert!(wrap.is_scale_uniform());
        assert!(wrap.0.borrow().s.is_none());
        wrap.set_y_scale(7.0);
        assert!(wrap.0.borrow().s.is_some());
        assert_eq!(wrap.x_scale(), 1.0);
        assert_eq!(wrap.y_scale(), 7.0);
        assert!(!wrap.is_scale_uniform());
        wrap.set_y_scale(1.0);
        assert!(wrap.is_scale_uniform());
        assert!(wrap.0.borrow().s.is_none());
        wrap.set_uniform_scale(1.0);
        assert!(wrap.0.borrow().s.is_none());
        assert!(wrap.is_scale_uniform());
        wrap.set_uniform_scale(6.5);
        assert!(wrap.is_scale_uniform());
        assert_eq!(wrap.x_scale(), 6.5);
        assert_eq!(wrap.y_scale(), 6.5);
        assert!(wrap.0.borrow().s.is_some());
        wrap.set_scale(1.0, 1.0);
        assert!(wrap.0.borrow().s.is_none());
        assert!(wrap.is_scale_uniform());
        wrap.set_scale(6.0, 7.0);
        assert!(wrap.0.borrow().s.is_some());
        assert_eq!(wrap.x_scale(), 6.0);
        assert_eq!(wrap.y_scale(), 7.0);
        assert!(!wrap.is_scale_uniform());
        assert_eq!(wrap.0.borrow().s, Some(Transform::new_from_set_transform_args(6.0, 0.0, 0.0, 7.0, 0.0, 0.0)));
        wrap.update_transform();
        assert_eq!(wrap.transform(), &Transform::new_from_set_transform_args(6.0, 35.0, 24.0, 7.0, -84.0, -91.0));
        //
        //  rotation and translation
        assert!(wrap.0.borrow().rt.is_none());
        assert!(wrap.state() == LocallyUpdated);
        assert_eq!(wrap.x(), 0);
        assert_eq!(wrap.y(), 0);
        assert_eq!(wrap.rotation(), 0.0);
        wrap.set_x(8);
        assert!(wrap.state() == Dirty);
        assert!(wrap.0.borrow().rt.is_some());
        assert_eq!(wrap.x(), 8);
        assert_eq!(wrap.y(), 0);
        assert_eq!(wrap.rotation(), 0.0);
        wrap.set_x(0);
        assert!(wrap.0.borrow().rt.is_none());
        wrap.set_y(9);
        assert!(wrap.0.borrow().rt.is_some());
        assert_eq!(wrap.x(), 0);
        assert_eq!(wrap.y(), 9);
        assert_eq!(wrap.rotation(), 0.0);
        wrap.set_y(0);
        assert!(wrap.0.borrow().rt.is_none());
        assert_eq!(wrap.rotation(), 0.0);
        wrap.set_both_x_and_y_to(1);
        assert!(!wrap.0.borrow().rt.is_none());
        assert_eq!(wrap.x(), 1);
        assert_eq!(wrap.y(), 1);
        wrap.set_both_x_and_y_to(0);
        assert!(wrap.0.borrow().rt.is_none());
        let pi_10 = std::f32::consts::PI / 10.0;
        wrap.set_rotation(pi_10);
        assert!(wrap.0.borrow().rt.is_some());
        assert_eq!(wrap.x(), 0);
        assert_eq!(wrap.y(), 0);
        assert_eq!(wrap.rotation(), pi_10);
        wrap.set_rotation(0.0);
        assert!(wrap.0.borrow().rt.is_none());
        wrap.set_x_and_y(8, 9);
        assert!(wrap.0.borrow().rt.is_some());
        wrap.set_rotation(pi_10);
        assert!(!wrap.is_scale_uniform());
        assert!(wrap.0.borrow().rt.is_some());
        assert_eq!(wrap.x(), 8);
        assert_eq!(wrap.y(), 9);
        assert_eq!(wrap.rotation(), pi_10);
        let c = f32::cos(pi_10);
        let s = f32::sin(pi_10);
        assert_eq!(wrap.0.borrow().rt, Some(Transform::new_from_set_transform_args(c, s, -s, c, 8.0, 9.0)));
        wrap.update_transform();
        assert!(
            transform_diff(
                wrap.transform(),
                &Transform::new_from_set_transform_args(
                      -5.109255705352237,
                      35.14108003658006,
                      20.66223743045905,
                      14.07380347906481,
                     -43.76820088067268,
                    -103.5035705103545
                )
            ) < 0.00002
        );
        // identity and sub-matrix abscence tests again
        wrap.set_x_pivot(0);
        wrap.set_y_pivot(0);
        wrap.set_x_skew(0.0);
        wrap.set_y_skew(0.0);
        wrap.set_rotation(0.0);
        wrap.set_x_scale(1.0);
        wrap.set_y_scale(1.0);
        wrap.set_both_x_and_y_to(0);
        assert!(wrap.0.borrow().p.is_none());
        assert!(wrap.0.borrow().k.is_none());
        assert!(wrap.0.borrow().s.is_none());
        assert!(wrap.0.borrow().rt.is_none());
        assert_eq!(wrap.update_transform(), &Transform::new_identity());
    }
}
