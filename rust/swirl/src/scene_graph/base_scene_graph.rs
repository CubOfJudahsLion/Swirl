//! Module  :   Base Scene Graph
//! Purpose :   A helper type that simplifies the task of
//!             creating `SceneGraph`-based objects.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar 16 08:20:07 PM CST 2021
//! Modified:   Mon 07/04/2022 02:04 PM


use std::cmp::PartialEq;
use std::cell::RefCell;
use std::default::Default;
use std::ops::Neg;

use crate::math::numeric::Floating;
use crate::math::geom::transform::Transform;
use crate::base::base_transformable::*;
use super::frame::SceneGraphFrame;


/// Base implementation of features for a `SceneGraph` object,
/// with the exception of `Bounded`, which is specific to
/// *frames* or *leaves*. The features are not directly implemented
/// for this `BaseSceneGraph<N>`, but for any type that implements
/// `HasBaseSceneGraph`. Thus, functionality is granted through
/// contained component.
#[derive(Clone, Debug)]
pub struct BaseSceneGraph<N> {
    pub(crate)  par         : Option<SceneGraphFrame<N>>,
    pub(crate)  trans       : BaseTransformable<N>,
    pub(crate)  world       : Transform<N>,
    pub(crate)  world_inv   : Transform<N>,
}


impl<N> Default for BaseSceneGraph<N>
    where   N   :   Default
                +   Floating + From<i16>
                +   Neg<Output=N>
                +   PartialEq
                +   Copy
{
    /// Produces a default `BaseSceneGraph`: it has no parent (yet),
    /// its local transform is neutral and non-state and equivalent
    /// to its world transform.
    fn default() -> Self {
        BaseSceneGraph {
            par         : None,
            trans       : BaseTransformable::<N>::new(),
            world       : Transform::new_identity(),
            world_inv   : Transform::new_identity(),
        }
    }
}


impl<N> BaseSceneGraph<N> {
    //  `SceneGraph` objects will be found often as children
    //  of `SceneGraphFrame` objects. Searches and deletions
    //  require these items to be *comparable*. For this, we will
    //  use their memory addresses as key values. **NOTE**: this
    //  equality mechanism will work only as far as the containing
    //  objects use *shared pointers*, as `RefCell` by itself lacks
    //  sharing mechanisms.
    pub(crate) fn addr_as_num(&self) -> usize {
        //  Note that `borrow` produces a temporary `Ref` object.
        //  Here we dereference it to obtain the actual content,
        //  reference it again to get its address.
        &*self.0.borrow() as *const BaseSceneGraphInner<N> as usize
    }

    //  Returns a reference to the `BaseSceneGraphInner`. Along with
    //  the `inner_mut` function below, it allows quick access
    //  to internals for implementing several interfaces.
    #[inline]
    pub(crate) fn inner(&self) -> &BaseSceneGraphInner<N> {
        //  The pointer here is dereferenced, then
        //  the reference taken out of it.
        unsafe { &*self.0.as_ptr() }
    }

    //  Returns a mutable reference to the `BaseSceneGraphInner`.
    #[inline]
    pub(crate) fn inner_mut(&self) -> &mut BaseSceneGraphInner<N> {
        unsafe { &mut *self.0.as_ptr() }
    }
}


impl<N> BaseSceneGraph<N>
    where   N   :   Floating + From<i16>
                +   PartialEq
                +   Neg<Output=N>
                +   Default
                +   Copy
{
    /// Creates a basic `BaseSceneGraph`. Equivalent to calling
    /// <BaseSceneGraph<N> as Default>::default().
    #[inline(always)]
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}


/// Helper macro to help containers of a `SceneGraph` implement it as a calls
/// relayed to the contained element.
///
/// The `$base_scene_graph` is a method of `&self` implemented by the `$container` type,
/// which returns a reference to the `BaseSceneGraph`.
#[macro_export]
macro_rules! scene_graph_from_base {
    ($base_scene_graph:ident in $container:ident) => {
        //  First, implement requirements, i.e., `CanHaveParent` and `Transformable`.
        //  These are made possible by the the composition of `BaseSceneGraph`.
        //  `Bounded` is not included here as different `SceneGraph` implementations
        //  are too fundamentally dissimilar.

        impl<N> crate::scene_graph::CanHaveParent<N> for $container<N> {
            fn parent(&self) -> &Option<crate::scene_graph::frame::SceneGraphFrame<N>> {
                &self.$base_scene_graph().inner().par
            }

            fn parent_mut(&self) -> &mut Option<crate::scene_graph::frame::SceneGraphFrame<N>> {
                &mut self.$base_scene_graph().inner_mut().par
            }
        }

        impl<N> $container<N> {
            #[inline(always)]
            fn base_transformable(&self) -> &crate::base::base_transformable::BaseTransformable<N> {
                &self.$base_scene_graph().inner().trans
            }

            fn base_transformable_mut(&self) -> &mut crate::base::base_transformable::BaseTransformable<N> {
                &mut self.$base_scene_graph().inner_mut().trans
            }
        }

        crate::impl_transformable_for!{base_transformable, mut base_transformable_mut for $container}

        //  The `SceneGraph` itself, assuming `Bounded` has been satisified.
        impl<N> crate::scene_graph::SceneGraph<N> for $container<N>
            where   N   :   crate::math::numeric::Floating + From<i16>
                        +   std::default::Default
                        +   crate::math::rect::RectBounds
                        +   std::ops::Neg<Output=N> + std::ops::Add<Output=N> + std::ops::Mul<Output=N>
                        +   std::cmp::PartialOrd
                        +   Copy
        {
            #[inline]
            fn world_transform(&self) -> &crate::math::transform::Transform<N> {
                 &self.$base_scene_graph().inner().world
            }

            fn set_world_transform(&self, t: &crate::math::transform::Transform<N>) {
                self.$base_scene_graph().inner_mut().world.copy_from(t);
            }

            #[inline(always)]
            fn unique_id(&self) -> usize {
                self.$base_scene_graph().addr_as_num()
            }

            fn global_to_local(&self, p: &crate::math::geom::point::Point<N>) -> crate::math::geom::point::Point<N> {
                let innerm = self.$base_scene_graph().inner_mut();
                if self.state() < crate::base::transformable::TransformableState::GloballyUpdatedWithInverse {
                    let world           = self.update_world_transform_from_top().1;
                    innerm.world_inv    = world
                        .inverse()
                        .unwrap_or_else(|_| crate::math::geom::transform::Transform::new_with_all_set_to(N::from(0)));
                    self.set_state(crate::base::transformable::TransformableState::GloballyUpdatedWithInverse);
                }
                p * &innerm.world_inv
            }
        }

        //  An extra benefit: [partial] equality operators, with the semantics being physical
        //  equality, i.e., that the two `SceneGraph` objects have the same address.
        impl<N, S> std::cmp::PartialEq<S> for $container<N>
            where   N   :   crate::math::numeric::Floating + From<i16>
                        +   crate::math::rect::RectBounds
                        +   std::ops::Neg<Output=N> + std::ops::Add<Output=N> + std::ops::Mul<Output=N>
                        +   std::cmp::PartialOrd
                        +   Copy,
                    S   :   crate::scene_graph::SceneGraph<N>
        {
            #[inline]
            fn eq(&self, rhs: &S) -> bool {
                self.$base_scene_graph().addr_as_num() == rhs.unique_id()
            }
        }
    };
}

//  Make the macro exportable from here as well.
pub(crate) use crate::scene_graph_from_base;


//  Module tests
#[cfg(test)]
mod tests {
    use std::cmp::PartialOrd;
    use std::ops::{Add, Sub};
    use std::rc::Rc;
    use super::*;
    use crate::math::numeric::Floating;
    use crate::math::geom::rect::*;
    use crate::math::transform::Transform;
    use crate::base::bounded::Bounded;
    use crate::base::transformable::*;
    use crate::base::drawable::Drawable;
    use super::super::can_have_parent::CanHaveParent;
    use super::super::leaf::SceneGraphLeaf;
    use super::super::frame::SceneGraphFrame;
    use super::super::node::SceneGraphNode;
    use wasm_bindgen::JsValue;
    use web_sys::CanvasRenderingContext2d;
    use wasm_bindgen_test::*;

    #[derive(Clone, Debug)]
    struct Wrap<N>(Rc<BaseSceneGraph<N>>);

    impl<N> Wrap<N> {
        pub fn new() -> Self
            where   N   :   Floating + From<i16>
        {
            Wrap(Rc::new(BaseSceneGraph::new()))
        }

        pub(crate) fn base_ref(&self) -> &BaseSceneGraph<N> { &self.0 }
    }

    impl<N> Bounded<N> for Wrap<N>
        where   N   :   Floating + From<i16>
                    +   PartialOrd
                    +   RectBounds
                    +   Add<Output=N> + Sub<Output=N>
                    +   Copy
    {
        fn bounds(&self) -> Option<Rect<N>> { None }
    }

    scene_graph_from_base!(base_ref in Wrap);


    #[wasm_bindgen_test]
    fn equality() {
        let w1 = Wrap::<f32>::new();
        let w1copy = w1.clone();
        assert_eq!(w1.0.addr_as_num(), w1copy.0.addr_as_num());
        assert_eq!(w1, w1copy);
        let w2 = Wrap::<f32>::new();
        assert_ne!(w1, w2);
    }

    #[wasm_bindgen_test]
    fn parent_manual_getting_setting() {
        let w1 = Wrap::<f32>::new();
        assert!(w1.parent().is_none());
        use super::super::frame::SceneGraphFrame;
        let f = SceneGraphFrame::new();
        *w1.parent_mut() = Some(f.clone());
        assert!(w1.parent().is_some());
        assert_eq!(w1.parent().as_ref().unwrap(), &f);
    }

    #[wasm_bindgen_test]
    fn children_insertion_and_cumulative_world_transforms() {
        use crate::base::base_transformable::tests::transform_diff;
        #[derive(Clone, Debug)]
        struct Invisible();
        impl Bounded<i16> for Invisible {
            fn bounds(&self) -> Option<Rect<i16>> { None }
        }
        impl Drawable for Invisible {
            fn draw(&self, _cnv: &CanvasRenderingContext2d) -> Result<(), JsValue> {
                Ok(())
            }
            fn drawable(&self) -> bool { false }
            fn set_drawable(&mut self, _drawable: bool) {}
        }
        //
        let i       = Invisible();
        let l       = SceneGraphLeaf::<f32>::new_node(&i);
        let mut f1  = SceneGraphFrame::<f32>::new();
        let mut f2  = SceneGraphFrame::<f32>::new();
        assert!(l.parent().is_none());
        assert!(f1.parent().is_none());
        assert!(f2.parent().is_none());
        assert!(f1.append_node(l.clone()).is_ok());
        assert!(f2.append_node(SceneGraphNode::Frame(f1.clone())).is_ok());
        assert_eq!(l.parent(), &Some(f1.clone()));
        assert_eq!(f1.parent(), &Some(f2.clone()));
        assert_eq!(f2.state(), TransformableState::Clear);
        f2
            .set_x_scale(0.5)
            .set_y_scale(-0.25)
            .set_rotation(std::f32::consts::PI / 8.0);
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
        assert_eq!(f2.state(), TransformableState::Updated);
        assert_eq!(f1.state(), TransformableState::Clear);
        f1
            .set_x_skew(1.1)
            .set_x_pivot(-20)
            .set_y_pivot(-10);
        assert_eq!(f1.state(), TransformableState::Dirty);
        assert_eq!(
            f1.update_transform(),
            &Transform::<f32>::new_from_set_transform_args(
                1.0, 0.0, 1.1, 1.0, 31.0, 10.0
            )
        );
        assert_eq!(f1.state(), TransformableState::Updated);
        assert_eq!(l.state(), TransformableState::Clear);
        l
            .set_x_pivot(4)
            .set_y_pivot(1)
            .set_rotation(-std::f32::consts::PI / 2.0)
            .set_x(8)
            .set_y(2);
        assert_eq!(l.state(), TransformableState::Dirty);
        l.update_transform();
        l.update_transform();
        assert_eq!(l.state(), TransformableState::Updated);
        assert!(
            transform_diff(
                l.transform(),
                &Transform::<f32>::new_from_set_transform_args(
                    0.0, -1.0, 1.0, 0.0, 7.0, 6.0
                )
            ) < 0.00001
        );
    }
}
