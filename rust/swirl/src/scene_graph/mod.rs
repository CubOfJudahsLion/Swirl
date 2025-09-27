//! Module  :   Scene Graph
//! Purpose :   A hierarchy of containers for Drawables.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar 16 08:20:07 PM CST 2021
//! Modified:   Tue 04/05/2022 11:26 AM


use std::cmp::PartialOrd;
use std::ops::{Neg, Add, Mul};

use crate::math::numeric::*;
use crate::math::geom::point::Point;
use crate::math::geom::rect::*;
use crate::base::bounded::Bounded;

pub mod base_hierarchical;
pub mod hierarchical;
pub mod leaf;
pub mod frame;
pub mod node;
pub mod layouts;

pub use base_hierarchical::*;
pub use hierarchical::*;
pub use leaf::*;
pub use frame::*;
pub use node::*;
pub use layouts::*;


/// A **Scene Graph** is a (mostly) hierarchical representation of a scene,
/// in the form of a
/// [Directed Acyclic Graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph).
/// This trait lists the *base* requirements for any single *node* in the
/// graph: having bounds, both a local and *global* transforms (i.e., the
/// result of cumulative transforms of its containers, and one that can be
/// applied directly to the canvas) and the ability to set an optional parent.
pub trait SceneGraph<N>: Hierarchical<N> + Bounded<N>
    where   N   :   Floating + From<i16>
                +   RectBounds
                +   Neg<Output=N> + Add<Output=N> + Mul<Output=N>
                +   PartialOrd
                +   Copy
{
    /// Returns an object's boundary point cloud relative to its parent.
    fn parent_bounds_points(&self) -> Vec<Point<N>> {
        let t = self.update_transform();
        self
            .bounds_points()
            .iter()
            .map(|p| p * t)
            .collect::<Vec<_>>()
    }

    /// Returns an object's bounds relative to its parent.
    fn parent_bounds(&self) -> Option<Rect<N>> {
        Rect::new_enclose_all(&self.parent_bounds_points())
    }

    #[inline]
    fn parent_width(&self) -> N {
        self
            .parent_bounds()
            .map(|b| b.right - b.left)
            .unwrap_or_else(|| 0i16.into())
    }

    #[inline]
    fn parent_height(&self) -> N {
        self
            .parent_bounds()
            .map(|b| b.bottom - b.top)
            .unwrap_or_else(|| 0i16.into())
    }


    /// Returns world-relative bounds for any `SceneGraph` object.
    fn world_bounds(&self) -> Option<Rect<N>> {
        let bpts    = self.bounds_points();
        let trans   = self.update_world_transform_from_top().1;
        Rect::new_enclose_all(
            &bpts
                .iter()
                .map(|pt| pt * trans)
                .collect::<Vec<_>>()
        )
    }

    #[inline]
    fn world_width(&self) -> N {
        self
            .world_bounds()
            .map(|b| b.right - b.left)
            .unwrap_or_else(|| 0i16.into())
    }

    #[inline]
    fn world_height(&self) -> N {
        self
            .world_bounds()
            .map(|b| b.bottom - b.top)
            .unwrap_or_else(|| 0i16.into())
    }

    /// Produces a unique number for each `SceneGraph` object. Two objects
    /// should have the same `unique_id` only if they're the same.
    fn unique_id(&self) -> usize;
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use crate::base::transformable::*;
    use super::super::leaf::tests::DrawnBox;
    use super::super::frame::tests::rect_diff;
    use super::*;

    #[wasm_bindgen_test]
    fn world_bounds() {
        if let Ok(boxy) = DrawnBox::new(20, 25) {
            let mut l = SceneGraphLeaf::<f32>::new_node(&boxy);
            l.set_pivot(10, 13);
            l.set_rotation(0.25);
            l.set_x_and_y(12, 10);
            let mut f = SceneGraphFrame::<f32>::new();
            f.set_uniform_scale(0.75);
            f.set_both_x_and_y_to(100);
            assert!(f.append_child(l.clone()).is_ok());
            assert!(
                rect_diff(
                    &l.world_bounds().unwrap(),
                    &Rect::new(
                         96.19757419391229,
                         99.50652120387946,
                        118.0757414898047,
                        118.6790317655614
                    )
                ) < 0.00001
            );
            assert!(rect_diff(&l.world_bounds().unwrap(), &f.world_bounds().unwrap()) < 0.00001);
            assert!((l.world_width()  - (118.6790317655614 - 99.50652120387946)).abs() < 0.00001);
            assert!((l.world_height() - (118.0757414898047 - 96.19757419391229)).abs() < 0.00001);
        }
        else {
            panic!("Failed to create DrawnBox.");
        }
    }
}
