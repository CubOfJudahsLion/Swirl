//! Module  :   Scene Graph
//! Purpose :   A hierarchy of containers for Drawables.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar 16 08:20:07 PM CST 2021
//! Modified:   Sat 06/11/2022 08:58 PM


use std::cell::{RefCell, Ref, RefMut};
use std::cmp::PartialEq;
use std::convert::From;
use std::ops::{Add, Sub};
use std::rc::Rc;

use crate::math::numeric::*;
use crate::math::geom::rect::*;
use crate::base::bounded::Bounded;
use crate::base::base_transformable::*;
use crate::base::transformable::*;
use crate::base::drawable::*;

use super::SceneGraph;
use super::base_hierarchical::*;
use super::hierarchical::*;
use super::node::SceneGraphNode;


//  The base structure contains a `BaseSceneGraph` of course,
//  along with a *init-only* drawable.
#[derive(Debug)]
pub(crate) struct SceneGraphLeafInner<N> {
    pub(crate) trans    : BaseTransformable<N>,
    pub(crate) hier     : BaseHierarchical<N>,
    pub(crate) drawable : Box<dyn Drawable>,
}

#[derive(Clone, Debug)]
pub struct SceneGraphLeaf<N>(pub(crate) Rc<RefCell<SceneGraphLeafInner<N>>>);


impl<N: From<i16> + Floating + PartialEq> SceneGraphLeaf<N> {
    pub fn new_from_dyn(drawable: Box<dyn Drawable>) -> Self {
        SceneGraphLeaf(Rc::new(RefCell::new(SceneGraphLeafInner {
            trans   : BaseTransformable::new(),
            hier    : BaseHierarchical::new(),
            drawable,
        })))
    }

    #[inline(always)]
    pub fn new<D>(drawable: &D) -> Self
        where D: Drawable + Clone + 'static
    {
        Self::new_from_dyn(Box::new(drawable.clone()) as Box<dyn Drawable>)
    }

    #[inline(always)]
    pub fn new_node<D>(drawable: &D) -> SceneGraphNode<N>
        where D: Drawable + Clone + 'static
    {
        SceneGraphNode::Leaf(Self::new(drawable))
    }

    #[inline(always)]
    pub fn drawable(&self) -> Ref<Box<dyn Drawable>> {
        Ref::map(self.0.borrow(), |inner| &inner.drawable)
    }

    #[inline(always)]
    pub fn drawable_mut(&self) -> RefMut<Box<dyn Drawable>> {
        RefMut::map(self.0.borrow_mut(), |inner| &mut inner.drawable)
    }
}


impl<N> Bounded<N> for SceneGraphLeaf<N>
    where   N   :   Floating + From<i16> + PartialOrd
                +   RectBounds
                +   Add<Output=N> + Sub<Output=N>
                +   Copy
{
    //fn bounds_points(&self) -> Vec<Point<N>> {
    //    self
    //        .0
    //        .borrow()
    //        .drawable
    //        .bounds()
    //        .map(|b| b.map_type::<N>().clockwise_corners().to_vec())
    //        .unwrap_or_else(|| Vec::new())
    //}

    fn bounds(&self) -> Option<Rect<N>> {
        let draw_bounds = self.0
            .borrow()
            .drawable
            .bounds();
        draw_bounds
            .map(|draw_rect|
                //  Conversion by dimension frees us from doing
                //  `RectBounds::INCLUSION` substraction and
                //  addition between the two types.
                //Rect::<N>::new_by_dim(
                //    draw_rect.top.into(),
                //    draw_rect.left.into(),
                //    draw_rect.height().into(),
                //    draw_rect.width().into()
                //)
                draw_rect.map_type::<N>()
            )
    }
}


impl<N> HasBaseTransformable<N> for SceneGraphLeaf<N> {
    #[inline]
    fn base_transformable(&self) -> &BaseTransformable<N> {
        unsafe { &(*self.0.as_ptr()).trans }
    }

    #[inline]
    fn base_transformable_mut(&self) -> &mut BaseTransformable<N> {
        unsafe { &mut (*self.0.as_ptr()).trans }
    }
}

impl<N> Transformable<N> for SceneGraphLeaf<N>
    where N: From<i16> + Floating
{}


impl<N> HasBaseHierarchical<N> for SceneGraphLeaf<N> {
    #[inline]
    fn base_hierarchical(&self) -> &BaseHierarchical<N> {
        unsafe { &(*self.0.as_ptr()).hier }
    }

    #[inline]
    fn base_hierarchical_mut(&self) -> &mut BaseHierarchical<N> {
        unsafe { &mut (*self.0.as_ptr()).hier }
    }
}

impl<N> Hierarchical<N> for SceneGraphLeaf<N>
    where N: Floating + From<i16> + RectBounds
{}


impl<N> SceneGraph<N> for SceneGraphLeaf<N>
    where N: Floating + From<i16> + RectBounds
{
    #[inline]
    fn unique_id(&self) -> usize {
        self.0.as_ptr() as usize
    }
}


impl<N, SG> PartialEq<SG> for SceneGraphLeaf<N>
    where   N   :   Floating + From<i16> + TryAsFrom<i16>
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
    use wasm_bindgen::JsValue;
    use web_sys::{console, CanvasRenderingContext2d};
    use crate::math::geom::transform::*;
    use crate::math::geom::point::*;
    use crate::base::drawable::Drawable;
    use crate::base::bounded::Bounded;
    use crate::scene_graph::hierarchical::tests::Undrawable;
    use crate::base::base_transformable::tests::transform_diff;
    use super::*;


    #[wasm_bindgen_test]
    fn equality_test() {
        let l1 = SceneGraphLeaf::<f32>::new(&Undrawable());
        let l2 = SceneGraphLeaf::<f32>::new(&Undrawable());
        assert_eq!(l1, l1);
        assert_eq!(l2, l2);
        assert_ne!(l1, l2);
        let wl1 = SceneGraphNode::Leaf(l1.clone());
        assert_eq!(l1, wl1);
    }


    #[derive(Clone, Debug)]
    pub struct DrawnBox {
        width       : i16,
        height      : i16,
    }

    impl DrawnBox {
        pub fn new(width: i16, height: i16) -> Result<Self, ()> {
            if width > 0 && height > 0 {
                Ok(DrawnBox { width, height, })
            }
            else {
                Err(())
            }
        }
    }

    impl Bounded<i16> for DrawnBox {
        fn bounds(&self) -> Option<Rect<i16>> {
            Some(Rect::new(0, 0, self.height - 1, self.width - 1))
        }
    }

    impl Drawable for DrawnBox {
        fn draw(&self, cnv: &CanvasRenderingContext2d) -> Result<(), JsValue> {
            cnv.stroke_rect(0.0, 0.0, (self.width - 1) as f64, (self.height - 1) as f64);
            Ok(())
        }
    }


    #[wasm_bindgen_test]
    fn bounds_test() {
        if let Ok(dbox) = DrawnBox::new(10, 8) {
            let mut l = SceneGraphLeaf::<f32>::new_node(&dbox);
            l.set_pivot(5, 4);
            l.set_rotation(std::f32::consts::PI / 3.);
            l.set_x_and_y(20, 15);
            console::log_1(&JsValue::from_str("Before first assert"));
            assert!(
                transform_diff(
                    l.update_transform(),
                    &Transform::new_from_set_transform_args(
                         0.5000000000000001,
	                       0.8660254037844386,
                        -0.8660254037844386,
	                       0.5000000000000001,
                        20.96410161513775,
	                       8.669872981077807
                    )
                ) < 0.00001
            );
            console::log_1(&JsValue::from_str("After first assert"));
            if let Some(corn) = l.parent_bounds().map(|b| b.clockwise_corners()) {
                console::log_1(&JsValue::from_str("Got corners"));
                assert!(
                    (   &corn[0] /*top-left*/
                     -   Point::new(14.03589838486224, 8.669872981077807)
                    ).dist() < 0.00001,
                    "top-left differs"
                );
                assert!(
                    (   &corn[2] /*bottom-right*/
                    -   Point::new(25.96410161513775, 21.33012701892219)
                    ).dist() < 0.00001,
                    "bottom-right differs"
                );
            }
            else {
                console::log_1(&JsValue::from_str("No bounds received"));
                panic!("No bounds received");
            }
        }
        else {
            console::log_1(&JsValue::from_str("Couldn't create DrawnBox"));
            panic!("Couldn't create DrawnBox");
        }
    }

    #[wasm_bindgen_test]
    fn bounds_points_test() {
        if let Ok(dbox) = DrawnBox::new(10, 8) {
            assert_eq!(
                dbox.bounds(),
                Some(Rect {top: 0, left: 0, bottom: 7, right: 9}),
                "Incorrect DrawnBox bounds"
            );
            let mut l2 = SceneGraphLeaf::<f32>::new(&dbox);
            l2.set_pivot(3, 11);
            l2.set_y_skew(-1.5);
            l2.set_rotation(0.48);
            assert!(
                transform_diff(
                    l2.update_transform(),
                    &Transform::new_from_set_transform_args(
		                     1.579663686091509,
                        -0.8687132086274434,
		                    -0.4617791755414828,
                         0.8869949227792842,
		                     0.3405798726817864,
                        -7.150804524689796
                    )
                ) < 0.00001
            );
            console::log_1(
                &JsValue::from_str(
                    &l2
                        .parent_bounds_points()
                        .iter()
                        .fold(
                            "bounds points: ".to_string(),
                            |s, p| s + &format!("\n\t{}", p)
                        )
                )
            );
            assert!(
                [   Point::<f32>::new( 0.3405798726817864,  -7.150804524689796  )
                ,   Point::<f32>::new(16.13721673359687,   -15.83793661096423   )
                ,   Point::<f32>::new(12.44298332926501,    -8.741977228729958  )
                ,   Point::<f32>::new(-3.353653531650076,   -0.05484514245552319)
                ]
                    .iter()
                    .map(|p| {
                        console::log_1(&JsValue::from_str(&format!("corner: {}", p)));
                        p
                    })
                    .zip(&l2.parent_bounds_points()[..])
                    .map(|(p1, p2)| {
                        console::log_1(&JsValue::from_str(&format!("pair: {}, {}", p1, p2)));
                        (p1, p2)
                    })
                    .map(|(p1, p2)| (p1 - p2).dist())
                    .fold(0., |acc, next| acc + next) < 0.0001
            );
        }
        else {
            panic!("Couldn't crate DrawnBox");
        }
    }
}
