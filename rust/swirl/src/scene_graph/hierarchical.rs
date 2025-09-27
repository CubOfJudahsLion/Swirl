//! Module  :   Hierarchical
//! Purpose :   A trait for objects that *may* have parents and,
//!             if so, applies ancestror transforms cumulatively.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Fri Sep 10 11:13:17 AM CST 2021
//! Modified:   Wed 08 Jun 2022 08:35:20 PM CST


use std::cmp::PartialOrd;
use std::default::Default;
use std::ops::{Neg, Add, Mul};

use crate::math::numeric::Floating;
use crate::math::geom::transform::*;
use crate::math::geom::point::Point;
use crate::math::geom::rect::RectBounds;

use crate::base::transformable_state::TransformableState;
use crate::base::transformable::*;

use super::base_hierarchical::*;
use super::frame::SceneGraphFrame;


/// The `Hierarchical` trait is a sub-trait of `HasBaseHierarchical`, and
/// transitively, of `Transformable`. Through `HasBaseHierarchical`, it
/// extends `Transformable`s to be able to have a **parent** (and thus a
/// hierarchy), and calculating the cumulative transforms from the top-most
/// ancestor.
pub trait Hierarchical<N>: Transformable<N> + HasBaseHierarchical<N>
    where   N   :   Floating + From<i16>
                +   Default
                +   RectBounds
                +   Neg<Output=N> + Add<Output=N> + Mul<Output=N>
                +   PartialOrd
                +   Copy
{
    #[inline]
    fn parent(&self) -> &Option<SceneGraphFrame<N>> {
        &self.base_hierarchical().par
    }

    #[inline]
    fn parent_mut(&mut self) -> &mut Option<SceneGraphFrame<N>> {
        &mut self.base_hierarchical_mut().par
    }

    fn set_parent(&self, new_par: Option<SceneGraphFrame<N>>) {
        if self.base_hierarchical().par != new_par {
            if self.state() > TransformableState::LocallyUpdated {
                self.set_state(TransformableState::LocallyUpdated);
            }
            self.base_hierarchical_mut().par = new_par;
        }
    }

    #[inline]
    fn world_transform(&self) -> &Transform<N> {
         &self.base_hierarchical().world
    }

    fn set_world_transform(&self, t: &Transform<N>) {
        self.base_hierarchical_mut().world.copy_from(t);
    }

    /// Updates the world transform, locally, and with an explicit parent (world)
    /// transform. This method is mostly used by the renderer, and not meant for
    /// API users.
    fn update_world_transform_from_parents(
        &self,
        parent_transform: Option<(bool, &Transform<N>)>
    ) -> Option<(bool, &Transform<N>)>
    {
        let local_dirty = self.state() <= TransformableState::LocallyUpdated;
        let local       = self.update_transform();
        match parent_transform {
            Some((par_dirty, par_trans))  => {
                let dirty = local_dirty || par_dirty;
                if dirty {
                    self.set_world_transform(&(local * par_trans));
                    self.set_state(TransformableState::GloballyUpdated);
                }
                Some((dirty, self.world_transform()))
            },
            _           => {
                if local_dirty {
                    self.set_world_transform(local);
                    self.set_state(TransformableState::GloballyUpdated);
                }
                Some((local_dirty, self.world_transform()))
            },
        }
    }

    /// Update the world transform, recursively gathering its parent's (if there is one.)
    fn update_world_transform_from_top(&self) -> (bool, &Transform<N>) {
        let local_dirty = self.state() <= TransformableState::LocallyUpdated;
        let local_trans = self.update_transform();
        match self.parent() {
            None            => {
                if local_dirty {
                    self.set_world_transform(local_trans);
                    self.set_state(TransformableState::GloballyUpdated);
                }
                (local_dirty, self.world_transform())
            },
            Some(ref par)   => {
                let (par_dirty, par_trans)  = par.update_world_transform_from_top();
                let dirty                   = par_dirty || local_dirty;
                if dirty {
                    self.set_world_transform(&(local_trans * par_trans));
                    self.set_state(TransformableState::GloballyUpdated);
                }
                (dirty, self.world_transform())
            },
        }
    }

    fn local_to_global(&self, p: &Point<N>) -> Point<N> {
        p * self.update_world_transform_from_top().1
    }

    fn global_to_local(&self, p: &Point<N>) -> Point<N> {
        let hier    = self.base_hierarchical_mut();
        if self.state() < TransformableState::GloballyUpdatedWithInverse {
            let world       = self.update_world_transform_from_top().1;
            hier.world_inv  = world
                .inverse()
                .unwrap_or_else(|_| Transform::new_with_all_set_to(N::from(0)));
            self.set_state(TransformableState::GloballyUpdatedWithInverse);
        }
        p * &hier.world_inv
    }
}


#[cfg(test)]
pub mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::*;
    use crate::math::numeric::Floating;
    use crate::math::rect::*;
    use crate::math::geom::transform::Transform;
    use crate::base::bounded::*;
    use crate::base::base_transformable::*;
    use crate::base::drawable::*;
    use super::super::leaf::SceneGraphLeaf;
    use super::super::frame::SceneGraphFrame;
    use super::super::node::SceneGraphNode;
    use wasm_bindgen::JsValue;
    use web_sys::CanvasRenderingContext2d;
    use wasm_bindgen_test::*;

    #[derive(Clone, Debug)]
    struct WrapInner<N> {
        t: BaseTransformable<N>,
        h: BaseHierarchical<N>,
    }

    #[derive(Clone, Debug)]
    struct Wrap<N>(Rc<RefCell<WrapInner<N>>>);

    impl<N> Wrap<N> {
        pub fn new() -> Self
            where   N   :   Floating + From<i16>
        {
            Wrap(Rc::new(RefCell::new(WrapInner {
                t: BaseTransformable::<N>::new(),
                h: BaseHierarchical::<N>::new(),
            })))
        }
    }

    impl<N> HasBaseTransformable<N> for Wrap<N> {
        fn base_transformable(&self) -> &BaseTransformable<N> {
            unsafe { &(*self.0.as_ptr()).t }
        }

        fn base_transformable_mut(&self) -> &mut BaseTransformable<N> {
            unsafe { &mut (*self.0.as_ptr()).t }
        }
    }

    impl<N> Transformable<N> for Wrap<N>
        where N: Floating + From<i16>
    {}

    impl<N> HasBaseHierarchical<N> for Wrap<N> {
        fn base_hierarchical(&self) -> &BaseHierarchical<N> {
            unsafe { &(*self.0.as_ptr()).h }
        }

        fn base_hierarchical_mut(&self) -> &mut BaseHierarchical<N> {
            unsafe { &mut (*self.0.as_ptr()).h }
        }
    }

    impl<N> Hierarchical<N> for Wrap<N>
        where N: RectBounds + Floating + From<i16>
    {}

    #[wasm_bindgen_test]
    fn parent_manual_getting_setting() {
        let w1 = Wrap::<f32>::new();
        assert!(w1.parent().is_none());
        let f = SceneGraphFrame::new();
        w1.set_parent(Some(f.clone()));
        assert!(w1.parent().is_some());
        assert_eq!(w1.parent().as_ref().unwrap(), &f);
    }


    #[derive(Clone, Debug)]
    pub struct Undrawable();

    impl Bounded<i16> for Undrawable {
        fn bounds_points(&self) -> Vec<Point<i16>> { Vec::new() }
        fn bounds(&self) -> Option<Rect<i16>> { None }
    }

    impl Drawable for Undrawable {
        fn draw(&self, _cnv: &CanvasRenderingContext2d) -> Result<(), JsValue> { Ok(()) }
        fn drawable(&self) -> bool { false }
        fn set_drawable(&mut self, _d: bool) {}
        fn alpha(&self) -> f64 { 1. }
        fn set_alpha(&mut self, _alpha: f64) {}
        fn blend_mode(&self) -> BlendMode { BlendMode::Normal }
        fn set_blend_mode(&mut self, _blend_mode: BlendMode) {}
    }


    #[wasm_bindgen_test]
    fn children_insertion_and_cumulative_world_transforms() {
        use crate::base::base_transformable::tests::transform_diff;
        //
        let u       = Undrawable();
        let mut l   = SceneGraphLeaf::<f32>::new_node(&u);
        let mut f1  = SceneGraphFrame::<f32>::new();
        let mut f2  = SceneGraphFrame::<f32>::new();
        assert!(l.parent().is_none());
        l.update_world_transform_from_top();
        assert_eq!(l.state(), TransformableState::GloballyUpdatedWithInverse);
        assert!(f1.parent().is_none());
        assert!(f2.parent().is_none());
        assert!(f1.append_child(l.clone()).is_ok());
        assert!(f2.append_child(SceneGraphNode::Frame(f1.clone())).is_ok());
        assert_eq!(l.parent(), &Some(f1.clone()));
        assert_eq!(l.state(), TransformableState::LocallyUpdated);
        assert_eq!(f1.parent(), &Some(f2.clone()));
        assert_eq!(f2.state(), TransformableState::GloballyUpdatedWithInverse);
        f2.set_x_scale(0.5);
        f2.set_y_scale(-0.25);
        f2.set_rotation(std::f32::consts::PI / 8.0);
        assert_eq!(f2.state(), TransformableState::Dirty);
        assert!(
            transform_diff(
                f2.update_transform(),
                &Transform::<f32>::new_from_set_transform_args(
                     0.4619397662556434,
                     0.1913417161825449,
                     0.09567085809127245,
                    -0.2309698831278217,
                     0.0,
                     0.0
                )
            ) < 0.00001
        );
        assert_eq!(f2.state(), TransformableState::LocallyUpdated);
        assert_eq!(f1.state(), TransformableState::LocallyUpdated);
        f1.set_x_skew(1.1);
        f1.set_x_pivot(-20);
        f1.set_y_pivot(-10);
        assert_eq!(f1.state(), TransformableState::Dirty);
        assert_eq!(
            f1.update_transform(),
            &Transform::<f32>::new_from_set_transform_args(
                1.0, 0.0, 1.1, 1.0, 31.0, 10.0
            )
        );
        assert_eq!(f1.state(), TransformableState::LocallyUpdated);
        assert_eq!(l.state(), TransformableState::LocallyUpdated);
        l.set_x_pivot(4);
        l.set_y_pivot(1);
        l.set_rotation(-std::f32::consts::PI / 2.0);
        l.set_x(8);
        l.set_y(2);
        assert_eq!(l.state(), TransformableState::Dirty);
        l.update_transform();
        l.update_transform();
        assert_eq!(l.state(), TransformableState::LocallyUpdated);
        assert!(
            transform_diff(
                l.transform(),
                &Transform::<f32>::new_from_set_transform_args(
                    0.0, -1.0, 1.0, 0.0, 7.0, 6.0
                )
            ) < 0.00001
        );
        assert!(
            transform_diff(
                l.update_world_transform_from_top().1,
                &Transform::<f32>::new_from_set_transform_args(
                    -0.6038046009724802,
                     0.02049399532702231,
                     0.4619397662556434,
                     0.1913417161825449,
                    22.13324730446206,
                     4.838322411696355
                )
            ) < 0.00001
        );
        assert!(
            transform_diff(
                l.world_transform(),
                &Transform::<f32>::new_from_set_transform_args(
                    -0.6038046009724802,
                     0.02049399532702231,
                     0.4619397662556434,
                     0.1913417161825449,
                    22.13324730446206,
                     4.838322411696355
                )
            ) < 0.00001
        );
        assert!(
            transform_diff(
                f1.world_transform(),
                &Transform::<f32>::new_from_set_transform_args(
                     0.4619397662556434,
                     0.1913417161825449,
                     0.6038046009724802,
                    -0.02049399532702231,
                    15.27684133483767,
	                 3.621894370380675
                )
            ) < 0.00001
        );
        assert_eq!(f2.transform(), f2.world_transform());
        assert_eq!(l.state(), TransformableState::GloballyUpdated);
        assert_eq!(f1.state(), TransformableState::GloballyUpdated);
        assert_eq!(f2.state(), TransformableState::GloballyUpdated);
        use crate::math::geom::point::RealPoint;
        assert!(
            (   l.global_to_local(&Point::new(2.0, 5.5))
            -   Point::new(33.26388225632759, -0.1046936319785168)
            ).dist() < 0.00001
        );
        assert_eq!(l.state(), TransformableState::GloballyUpdatedWithInverse);
        assert!(
            (   l.local_to_global(&l.global_to_local(&Point::new(2.0, 5.5)))
            -   Point::new(2.0, 5.5)
            ).dist() < 0.00001
        );
    }
}
