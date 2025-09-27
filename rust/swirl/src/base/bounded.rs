//! Module  :   Bounded
//! Purpose :   Object that have a local space and know
//!             the boundaries they occupy within it.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   2021-02-22 14:51
//! Modified:   Sat 07 May 2022 08:08:48 PM CST


use std::cmp::PartialOrd;
use std::ops::{Add, Sub};

use crate::math::numeric::Numeric;
use crate::math::geom::rect::*;
use crate::math::geom::point::*;


/// Any element that can describe its bounds in its own
/// coordinate system (or rather, its parent's.) `scene_graph`
/// hierarchies will use it to compute global ones.
///
/// **NOTE** that `bounds_points` and `bounds` are meant to
/// return coordinates that are in the object's coordinate
/// system, *not it's parent's* (strictly speaking, `Bounded`
/// is *not* aware of parental relationships.) In the same vein,
/// `width` and `height` use `bounds` for their calculations,
/// so again, these result in object-local coordinates,
/// hierarchically unaware. For bounds relative to a parent,
/// use the `parent_*` methods in the `SceneGraph`
/// trait.
///
/// **WARNING**: any implementor of `Bounded` **must**
/// provide either `bounds_points` or `bounds` (or both) as
/// default implementations for these functions depend on
/// each other and would incur in mutual recursion otherwise.
pub trait Bounded<N>
    where   N   :   Numeric + From<i16> + PartialOrd
                +   RectBounds
                +   Add<Output=N> + Sub<Output=N>
                +   Copy
{
    fn bounds_points(&self) -> Vec<Point<N>> {
        self
            .bounds()
            .map(|b| b
                 .clockwise_corners()[..]
                 .iter()
                 .map(|&n| n)
                 .collect::<Vec<_>>()
            )
            .unwrap_or_else(|| Vec::new())
    }

    fn bounds(&self) -> Option<Rect<N>> {
        Rect::new_enclose_all(&self.bounds_points()[..])
    }

    fn width(&self) -> N {
        self
            .bounds()
            .map(|b| b.width())
            .unwrap_or_else(|| 0.into())
    }

    fn height(&self) -> N {
        self
            .bounds()
            .map(|b| b.height())
            .unwrap_or_else(|| 0.into())
    }
}
