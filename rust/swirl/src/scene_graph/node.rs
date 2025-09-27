//! Module  :   Scene Graph Node
//! Purpose :   Utilitarian enum that can contain both
//!             a leaf and a frame. Mostly used to
//!             allow a frame children to be both leaves
//!             and frame, it has some other uses.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar 16 08:20:07 PM CST 2021
//! Modified:   Fri 04/29/2022 02:10 PM


use std::ops::{Neg, Add, Sub};

use crate::math::numeric::Floating;
use crate::math::geom::point::*;
use crate::math::geom::rect::*;
use crate::base::bounded::Bounded;
use crate::base::drawable::Drawable;
use crate::base::base_transformable::*;
use crate::base::transformable::*;

use super::base_hierarchical::*;
use super::hierarchical::*;
use super::SceneGraph;
use super::leaf::SceneGraphLeaf;
use super::frame::SceneGraphFrame;


/// A node in a graph can be either a leaf
/// or a frame. This type can enclose both.
#[derive(Clone, Debug)]
pub enum SceneGraphNode<N> {
    Leaf(SceneGraphLeaf<N>),
    Frame(SceneGraphFrame<N>),
}


impl<N> SceneGraphNode<N> {
    pub fn is_leaf(&self) -> bool {
        use SceneGraphNode::*;
        match self {
            &Leaf(_)    => true,
            _           => false,
        }
    }

    pub fn is_frame(&self) -> bool {
        use SceneGraphNode::*;
        match self {
            &Frame(_)   => true,
            _           => false
        }
    }

    pub fn run_either<T, FLeaf, FFrame>(&self, fn_leaf: FLeaf, fn_frame: FFrame) -> T
        where   FLeaf   :   FnOnce(&SceneGraphLeaf<N>) -> T,
                FFrame  :   FnOnce(&SceneGraphFrame<N>) -> T
    {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref l)    => fn_leaf(l),
            &Frame(ref f)   => fn_frame(f),
        }
    }

    #[inline(always)]
    pub fn new_leaf_dyn(d: Box<dyn Drawable>) -> Self
        where   N: Floating + From<i16>
    {
        SceneGraphNode::<N>::Leaf(SceneGraphLeaf::new_from_dyn(d))
    }

    #[inline(always)]
    pub fn new_leaf<D: Drawable + Clone + 'static>(d: &D) -> Self
        where   N: Floating + From<i16>
    {
        SceneGraphNode::<N>::new_leaf_dyn(Box::new(d.clone()))
    }

    #[inline(always)]
    pub fn new_frame() -> Self
        where   N: Floating + RectBounds + From<i16>
    {
        SceneGraphNode::Frame(SceneGraphFrame::<N>::new())
    }
}


/// Make the node bounded. The bounds of a node,
/// naturally, are those of its leaf or frame.
impl<N> Bounded<N> for SceneGraphNode<N>
    where   N   :   Floating + From<i16>
                +   RectBounds
                +   Neg<Output=N> + Add<Output=N> + Sub<Output=N>
                +   PartialOrd
                +   Copy
{
    #[inline]
    fn bounds_points(&self) -> Vec<Point<N>> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.bounds_points(),
            &Frame(ref frame)   => frame.bounds_points(),
        }
    }
}


impl<N> HasBaseTransformable<N> for SceneGraphNode<N>
    where N: Floating + From<i16>
{
    #[inline]
    fn base_transformable(&self) -> &BaseTransformable<N> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.base_transformable(),
            &Frame(ref frame)   => frame.base_transformable(),
        }
    }

    #[inline]
    fn base_transformable_mut(&self) -> &mut BaseTransformable<N> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.base_transformable_mut(),
            &Frame(ref frame)   => frame.base_transformable_mut(),
        }
    }
}

impl<N> Transformable<N> for SceneGraphNode<N>
    where N: From<i16> + Floating
{}


impl<N> HasBaseHierarchical<N> for SceneGraphNode<N> {
    #[inline]
    fn base_hierarchical(&self) -> &BaseHierarchical<N> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.base_hierarchical(),
            &Frame(ref frame)   => frame.base_hierarchical(),
        }
    }

    #[inline]
    fn base_hierarchical_mut(&self) -> &mut BaseHierarchical<N> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.base_hierarchical_mut(),
            &Frame(ref frame)   => frame.base_hierarchical_mut(),
        }
    }
}

impl<N> Hierarchical<N> for SceneGraphNode<N>
    where N: Floating + From<i16> + RectBounds
{}


impl<N> SceneGraph<N> for SceneGraphNode<N>
    where N: Floating + From<i16> + RectBounds
{
    fn unique_id(&self) -> usize {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.unique_id(),
            &Frame(ref frame)   => frame.unique_id(),
        }
    }
}


impl<N, SG> PartialEq<SG> for SceneGraphNode<N>
    where   N   :   Floating + From<i16>
                +   RectBounds,
            SG  :   SceneGraph<N>
{
    fn eq(&self, other: &SG) -> bool {
        self.unique_id() == other.unique_id()
    }
}


#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use super::*;
    use super::super::hierarchical::tests::Undrawable;

    #[wasm_bindgen_test]
    fn equality_test() {
        let f = SceneGraphFrame::<f32>::new();
        let nf = SceneGraphNode::Frame(f.clone());
        let l = SceneGraphLeaf::<f32>::new(&Undrawable());
        let nl = SceneGraphNode::Leaf(l.clone());
        assert_eq!(nf, f);
        assert_eq!(nf, nf);
        assert_eq!(nl, l);
        assert_eq!(nl, nl);
        assert_ne!(nf, l);
        assert_ne!(nl, f);
        assert_ne!(nl, nf);
    }

    #[wasm_bindgen_test]
    fn scenegraph_inheritance_test() {
        // Should be ok. Compilation itself is the test.
        let mut f = SceneGraphFrame::<f32>::new();
        let l = SceneGraphLeaf::<f32>::new_node(&Undrawable());
        assert!(f.append_child(l.clone()).is_ok());
        assert_eq!(l.parent(), &Some(f.clone()));
    }
}
