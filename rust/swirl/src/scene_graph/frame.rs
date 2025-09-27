//! Module  :   Scene Graph
//! Purpose :   A hierarchy of containers for Drawables.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar 16 08:20:07 PM CST 2021
//! Modified:   Sat 04/23/2022 09:58 PM


use std::cell::RefCell;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};
use std::rc::Rc;

use wasm_bindgen::{throw_str};

use crate::math::numeric::*;
use crate::math::rect::*;
use crate::math::geom::point::Point;
use crate::base::bounded::Bounded;
use crate::base::base_transformable::*;
use crate::base::transformable::*;

use super::base_hierarchical::*;
use super::hierarchical::*;
use super::SceneGraph;
use super::node::SceneGraphNode;


#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum SceneGraphFrameError {
    ChildAlreadyPresent,
    ChildMissing,
    OpenShare,
    BadIndex,
    CannotBeSelfParent,
}

impl From<SceneGraphFrameError> for &'static str {
    fn from(err: SceneGraphFrameError) -> Self {
        use SceneGraphFrameError::*;
        match err {
            ChildAlreadyPresent => "Child already present",
            ChildMissing        => "Child missing",
            OpenShare           => "Open share",
            BadIndex            => "Bad index",
            CannotBeSelfParent  => "Cannot be self-parent",
        }
    }
}


#[derive(Debug)]
pub(crate) struct SceneGraphFrameInner<N> {
    pub(crate) trans    : BaseTransformable<N>,
    pub(crate) hier     : BaseHierarchical<N>,
    pub(crate) children : Vec<SceneGraphNode<N>>,
}

#[derive(Clone, Debug)]
pub struct SceneGraphFrame<N>(pub(crate) Rc<RefCell<SceneGraphFrameInner<N>>>);


impl<N> Bounded<N> for SceneGraphFrame<N>
    where   N   :   Floating + From<i16>
                +   PartialOrd
                +   RectBounds
                +   Neg<Output=N> + Add<Output=N> + Sub<Output=N>
                +   Copy
{
    fn bounds_points(&self) -> Vec<Point<N>> {
        //let trans = self.update_transform();
        self.0.borrow().children
            .iter()
            .map(|ch| {
                match ch {
                    SceneGraphNode::Leaf(leaf)      => {
                        leaf
                            .bounds_points()
                            .iter()
                            .map(|p| p * leaf.update_transform())
                            .collect::<Vec<_>>()
                    },
                    SceneGraphNode::Frame(frame)    => {
                        frame
                            .bounds_points()
                            .iter()
                            .map(|p| p * frame.update_transform())
                            .collect::<Vec<_>>()
                    },
                }
            })
            .collect::<Vec<Vec<Point<N>>>>()
            .concat()
    }
}


impl<N> HasBaseTransformable<N> for SceneGraphFrame<N>
    where N: Floating + From<i16>
{
    fn base_transformable(&self) -> &BaseTransformable<N> {
        unsafe { &(*self.0.as_ptr()).trans }
    }

    fn base_transformable_mut(&self) -> &mut BaseTransformable<N> {
        unsafe { &mut (*self.0.as_ptr()).trans }
    }
}

impl<N> Transformable<N> for SceneGraphFrame<N>
    where N: From<i16> + Floating
{}


impl<N> HasBaseHierarchical<N> for SceneGraphFrame<N> {
    fn base_hierarchical(&self) -> &BaseHierarchical<N> {
        unsafe { &(*self.0.as_ptr()).hier }
    }

    fn base_hierarchical_mut(&self) -> &mut BaseHierarchical<N> {
        unsafe { &mut (*self.0.as_ptr()).hier }
    }
}

impl<N> Hierarchical<N> for SceneGraphFrame<N>
    where N: Floating + From<i16> + RectBounds
{}


impl<N> SceneGraph<N> for SceneGraphFrame<N>
    where N: Floating + From<i16> + RectBounds
{
    fn unique_id(&self) -> usize {
        self.0.as_ptr() as usize
    }
}


impl<N> SceneGraphFrame<N>
    where   N   :   Floating + From<i16>
                +   RectBounds
                +   Neg<Output=N> + Add<Output=N> + Mul<Output=N>
                +   PartialOrd
                +   Copy
{
    const LOAD_ESTIMATE: usize = 64;

    pub fn new() -> Self {
        SceneGraphFrame(Rc::new(RefCell::new(SceneGraphFrameInner {
            trans   : BaseTransformable::new(),
            hier    : BaseHierarchical::new(),
            children: Vec::with_capacity(Self::LOAD_ESTIMATE),
        })))
    }

    #[inline]
    pub fn new_node() -> SceneGraphNode<N> {
        SceneGraphNode::Frame(Self::new())
    }

    pub fn children_count(&self) -> usize {
        self.0.borrow().children.len()
    }

    pub fn child_position(&self, node: &SceneGraphNode<N>) -> Option<usize> {
        if let &Some(ref par) = node.parent() {
            if par.unique_id() == self.unique_id() {
                Some(par
                    .0
                    .borrow()
                    .children
                    .iter()
                    .enumerate()
                    .find(|(_, child)| *child == node)?
                    .0
                )
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    #[inline(always)]
    pub fn contains_child(&self, node: &SceneGraphNode<N>) -> bool {
        self.child_position(node).is_some()
    }

    pub fn insert_child(
        &mut self,
        at  : usize,
        node: SceneGraphNode<N>
    ) -> Result<&mut Self, SceneGraphFrameError>
    {
        if at > self.0.borrow().children.len() {
            return Err(SceneGraphFrameError::BadIndex);
        }
        else if *self == node {
            return Err(SceneGraphFrameError::CannotBeSelfParent);
        }
        if let &Some(ref par) = node.parent() {
            if par == self {
                return Err(SceneGraphFrameError::ChildAlreadyPresent);
            }
            let children    = &mut par.0.borrow_mut().children;
            let (index, _)  = children
                .iter()
                .enumerate()
                .find(|(_, ch)| ch.unique_id() == node.unique_id())
                .ok_or(SceneGraphFrameError::ChildMissing)?;
            children.remove(index);
        }
        node.set_parent(Some(self.clone()));
        self.0.borrow_mut().children.insert(at, node);
        Ok(self)
    }

    #[inline]
    pub fn append_child(&mut self, node: SceneGraphNode<N>) -> Result<&mut Self, SceneGraphFrameError> {
        self.insert_child(self.children_count(), node)
    }

    #[inline]
    pub fn prepend_child(&mut self, node: SceneGraphNode<N>) -> Result<&mut Self, SceneGraphFrameError> {
        self.insert_child(0, node)
    }

    pub fn swap_children_by_index(&mut self, a: usize, b: usize) -> Result<&mut Self, SceneGraphFrameError> {
        let ch_len          = self.0.borrow().children.len();
        if a > ch_len || b > ch_len {
            Err(SceneGraphFrameError::BadIndex)
        }
        else {
            self.0.borrow_mut().children.swap(a, b);
            Ok(self)
        }
    }

    pub fn swap_children(&mut self, n1: &SceneGraphNode<N>, n2: &SceneGraphNode<N>) -> Result<&mut Self, SceneGraphFrameError> {
        let num1 = self.child_position(n1).ok_or(SceneGraphFrameError::ChildMissing)?;
        let num2 = self.child_position(n2).ok_or(SceneGraphFrameError::ChildMissing)?;
        self.swap_children_by_index(num1, num2)
    }

    pub fn remove_child(&mut self, node: &SceneGraphNode<N>) -> Result<&mut Self, SceneGraphFrameError> {
        let search_res = self.0.borrow().children
            .iter()
            .enumerate()
            .find(|(_, ch)| *ch == node)
            .map(|(idx, _)| idx);
        match search_res {
            Some(idx)   =>  {
                self.0.borrow_mut().children.remove(idx);
                node.set_parent(None);
                Ok(self)
            },
            _           =>  {
                Err(SceneGraphFrameError::ChildMissing)
            },
        }
    }

    pub fn remove_all_children(&mut self) -> &mut Self {
        self.0.borrow_mut().children.iter_mut().for_each(|child|
            child.set_parent(None)
        );
        self.0.borrow_mut().children.clear();
        self
    }
}


impl<N> Index<usize> for SceneGraphFrame<N> {
    type Output = SceneGraphNode<N>;

    fn index(&self, index: usize) -> &Self::Output {
        let children = unsafe { &(*self.0.as_ptr()).children };
        if index >= children.len() {
            throw_str("<ScenGraphFrame::Index>::index(): index too large.");
        }
        &children[index]
    }
}

impl<N> IndexMut<usize> for SceneGraphFrame<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let children = unsafe { &mut (*self.0.as_ptr()).children };
        if index >= children.len() {
            throw_str("<ScenGraphFrame as IndexMut<usize>>::index_mut(): index too large.");
        }
        &mut children[index]
    }
}


impl<N, SG> PartialEq<SG> for SceneGraphFrame<N>
    where   N   :   Floating + From<i16>
                +   RectBounds,
            SG  :   SceneGraph<N>
{
    fn eq(&self, other: &SG) -> bool {
        self.unique_id() == other.unique_id()
    }
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    //use wasm_bindgen::JsValue;
    //use web_sys::console;
    use crate::math::numeric::Signed;
    use crate::math::geom::point::RealPoint;
    use crate::math::geom::transform::*;
    use crate::base::base_transformable::tests::transform_diff;
    use super::super::hierarchical::tests::Undrawable;
    use super::super::leaf::SceneGraphLeaf;
    use super::super::leaf::tests::DrawnBox;
    use super::*;

    #[wasm_bindgen_test]
    fn children_operations() {
        let l1 = SceneGraphLeaf::<f32>::new_node(&Undrawable());
        let l2 = SceneGraphLeaf::<f32>::new_node(&Undrawable());
        let l3 = SceneGraphLeaf::<f32>::new_node(&Undrawable());
        let mut f1 = SceneGraphFrame::<f32>::new();
        let mut f2 = SceneGraphFrame::<f32>::new();
        assert!(f1.append_child(l1.clone()).is_ok());
        assert!(f1.append_child(l2.clone()).is_ok());
        assert!(f1.append_child(l3.clone()).is_ok());
        assert_eq!(f1.child_position(&l1), Some(0));
        assert_eq!(f1.child_position(&l2), Some(1));
        assert_eq!(f1.child_position(&l3), Some(2));
        assert_eq!(Some(f1.clone()), *l1.parent());
        assert!(f1.contains_child(&l1));
        assert!(f1.remove_child(&l1).is_ok());
        assert!(!f1.contains_child(&l1));
        assert!(f1.child_position(&l1).is_none());
        assert!(l1.parent().is_none());
        assert_eq!(f1.child_position(&l2), Some(0));
        assert_eq!(f1.child_position(&l3), Some(1));
        assert!(f2.append_child(l2.clone()).is_ok());
        assert_eq!(&Some(f2.clone()), l2.parent());
        assert!(!f1.contains_child(&l2));
        assert!(f2.contains_child(&l2));
        assert_eq!(f1.child_position(&l2), None);
        assert_eq!(f2.child_position(&l2), Some(0));
        assert!(f2.prepend_child(l1.clone()).is_ok());
        assert_eq!(l1.parent(), &Some(f2.clone()));
        assert_eq!(l2.parent(), &Some(f2.clone()));
        assert_eq!(f2.child_position(&l1), Some(0));
        assert_eq!(f2.child_position(&l2), Some(1));
        assert!(f2.insert_child(1, l3.clone()).is_ok());
        assert!(!f1.contains_child(&l3));
        assert!(f1.child_position(&l3).is_none());
        assert_eq!(l3.parent(), &Some(f2.clone()));
        assert_eq!(f2.child_position(&l3), Some(1));
        assert_eq!(f2.child_position(&l2), Some(2));
        assert_eq!(f2.child_position(&l1), Some(0));
        assert!(f2.swap_children_by_index(1, 2).is_ok());
        assert_eq!(f2.child_position(&l3), Some(2));
        assert_eq!(f2.child_position(&l2), Some(1));
        assert!(f2.swap_children(&l2, &l3).is_ok());
        assert_eq!(f2.child_position(&l3), Some(1));
        assert_eq!(f2.child_position(&l2), Some(2));
        f2.remove_all_children();
        assert_eq!(l2.parent(), &None);
        assert_eq!(l3.parent(), &None);
    }

    #[wasm_bindgen_test]
    fn indexing_test() {
        let mut l1 = SceneGraphLeaf::<f32>::new_node(&Undrawable());
        let mut l2 = SceneGraphLeaf::<f32>::new_node(&Undrawable());
        let mut f1 = SceneGraphFrame::<f32>::new();
        assert!(f1.append_child(l1.clone()).is_ok());
        assert!(f1.append_child(l2.clone()).is_ok());
        assert_eq!(&l1, &f1[0]);
        assert_eq!(&mut l1, &mut f1[0]);
        assert_eq!(&l2, &f1[1]);
        assert_eq!(&mut l2, &mut f1[1]);
    }

    #[wasm_bindgen_test]
    fn equality_test() {
        let f1 = SceneGraphFrame::<f32>::new();
        let f2 = SceneGraphFrame::<f32>::new();
        assert_eq!(f1, f1);
        assert_eq!(f2, f2);
        assert_ne!(f1, f2);
        let l1 = SceneGraphLeaf::<f32>::new(&Undrawable());
        let l2 = SceneGraphLeaf::<f32>::new_node(&Undrawable());
        assert_ne!(f1, l1);
        assert_ne!(f2, l1);
        assert_ne!(f1, l2);
        assert_ne!(f2, l2);
        let wf1 = SceneGraphNode::Frame(f1.clone());
        assert_eq!(f1, wf1);
    }

    /// Get the cumulative component-wise difference
    /// between two rectangles.
    pub fn rect_diff<N>(r1: &Rect<N>, r2: &Rect<N>) -> N
        where N: Add<Output=N> + Sub<Output=N> + Signed
    {
            (r1.top    - r2.top   ).abs()
        +   (r1.left   - r2.left  ).abs()
        +   (r1.bottom - r2.bottom).abs()
        +   (r1.right  - r2.right ).abs()
    }

    #[wasm_bindgen_test]
    fn bounds_test() {
        if  let (Ok(b1), Ok(b2)) =
                (DrawnBox::new(50, 12), DrawnBox::new(20, 30))
        {
            assert_eq!(b1.bounds(), Some(Rect::new(0, 0, 11, 49)));
            assert_eq!(b2.bounds(), Some(Rect::new(0, 0, 29, 19)));
            let mut l1 = SceneGraphLeaf::<f32>::new_node(&b1);
            l1.set_pivot(10, -20);
            l1.set_y_skew(12.);
            assert!(
                transform_diff(
                    l1.update_transform(),
                    &Transform::new_from_set_transform_args(
                           1.,
                          12.,
                           0.,
                           1.,
                         -10.,
                        -100.
                    )
                ) < 0.00001
            );
            assert_eq!(l1.parent_bounds(), Some(Rect {top: -100., left: -10., bottom: 512., right:40.,}));
            let mut l2 = SceneGraphLeaf::<f32>::new_node(&b2);
            l2.set_rotation(std::f32::consts::PI * 7. / -8.);
            l2.set_both_x_and_y_to(60);
            assert!(
                transform_diff(
                    l2.update_transform(),
                    &Transform::new_from_set_transform_args(
                        -0.9238795325112867,
                        -0.3826834323650899,
                         0.3826834323650899,
                        -0.9238795325112867,
                        60.,
                        60.
                    )
                ) < 0.00001
            );
            if let Some(l2_b) = l2.parent_bounds() {
                assert!(
                    rect_diff(
                        &l2_b,
                        &Rect::<f32>::new(24.6299453773596, 41.52240934977426, 60., 71.4805029709527)
                    ) < 0.00001
                );
            }
            else {
                panic!("l2 incorrectly reports no bounds");
            }
            let mut f1 = SceneGraphFrame::new();
            assert!(f1.append_child(l1).is_ok());
            f1.set_x_scale(-0.25);
            f1.set_x_skew(3.);
            f1.set_pivot(10, 12);
            assert!(
                transform_diff(
                    f1.update_transform(),
                    &Transform::new_from_set_transform_args(
                         -0.25,
                          0.,
                         -0.75,
                          1.,
                         11.5,
                        -12.
                    )
                ) < 0.00001
            );
            let mut f2 = SceneGraphFrame::new();
            assert!(f2.append_child(SceneGraphNode::Frame(f1.clone())).is_ok());
            assert!(f2.append_child(l2).is_ok());
            f2.set_y_pivot(25);
            f2.set_rotation(std::f32::consts::PI * 5. / 4.);
            f2.set_uniform_scale(1./3.);
            f2.set_x_and_y(-44, 80);
            assert!(
                transform_diff(
                    f2.update_transform(),
                    &Transform::new_from_set_transform_args(
                        -0.2357022603955159,
                        -0.2357022603955158,
                         0.2357022603955158,
                        -0.2357022603955159,
                        -49.8925565098879,
                         85.8925565098879
                    )
                ) < 0.00001
            );
            //  Key test. The transformed points should fit inside a
            //  rectangle with certain minimums and maximums.
            if let Some(f2_bounds) = f2.parent_bounds() {
                //console::log_1(&JsValue::from_str(&format!("{:?}", f2_bounds)));
                let f2_bounds = f2_bounds.clockwise_corners();
                assert!(
                    (   &f2_bounds[0]
                    -   Point::new(-97.26871084938658, 57.60828526242599)
                    ).dist() < 1.
                );
                assert!(
                    (   &f2_bounds[2] 
                    -   Point::new(158.1146882891549, 91.31370849898477)
                    ).dist() < 1.
                );
            }
            else {
                panic!("Failed to calculate bounds.");
            }
        }
        else {
            panic!("Couldn't create DrawnBox instances.");
        }
    }
}
