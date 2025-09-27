//! # Module        :   _Layouts_
//! - **Purpose**   :   Visual organization utilities
//!                     for scene-building.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Tue 04/05/2022 11:10 AM
//! - **Modified**  :   Fri 06/24/2022 12:43 AM


use std::cmp::{PartialEq, Eq};
use std::fmt::{self, Display, Formatter, Write};
use std::collections::HashMap;
use std::hash::Hash;

use crate::base::transformable::Transformable;
use crate::math::geom::dimensions::*;
use crate::math::geom::rect::*;
use crate::scene_graph::frame::SceneGraphFrameError;
use crate::scene_graph::frame::*;
use crate::scene_graph::node::*;


pub mod padding;
pub use padding::*;

pub mod alignment;
pub use alignment::*;

pub mod scaling;
pub use scaling::*;

pub mod constraints;
pub use constraints::*;

pub mod fast_forwarding;

pub mod single;
pub use single::*;

pub mod superposition;
pub use superposition::*;

pub mod line_arrangement;
pub use line_arrangement::*;

pub mod mac;
pub use mac::*;


/// Errors resulting from `Layout` creation or its
/// corresponding `SceneGraph` calculation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LayoutError<K> {
    AssignedAreaTooSmall,
    ErrorInSettings,
    DuplicateName(K),
    SceneGraphConstructionFailure(SceneGraphFrameError),
}

impl<K: Display> Display for LayoutError<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[Layout Error: ")?;
        use LayoutError::*;
        match self {
            &AssignedAreaTooSmall                       =>
                f.write_str("Assigned area too small")?,
            &ErrorInSettings                            =>
                f.write_str("Error in settings")?,
            &DuplicateName(ref name)                    =>
                f.write_fmt(format_args!(r#"Duplicate name found: "{}""#, name))?,
            &SceneGraphConstructionFailure(ref sgf_err) =>
                f.write_fmt(format_args!(
                    "Scene-graph construction error: {}",
                    <&'static str as From<SceneGraphFrameError>>::from(sgf_err.clone())
                ))?,
        }
        f.write_char(']')
    }
}


/// Several layouts ask for a z-index for their elements.
/// In *SWIRL*, a lower z-index means being drawn earlier,
/// i.e., **below** elements with higher indexes.
///
/// If you don't care about the relative drawing order of
/// certain items, you can give them all the same z-order.
pub type ZOrder = u16;


/// The result produced by `Layout::calc_graph()`. It contains
/// the positioned, z-ordered scene graph as a `SceneGraphNode`.
/// It also returns three maps of "named" leaves/frames/nodes.
pub struct GraphCalcOutput<K: Eq + Hash> {
    pub graph       : SceneGraphNode<f32>,
    pub named_nodes : HashMap<K, SceneGraphNode<f32>>,
}


/// A `Layout` implements the ability to position objects
/// according to a shelf-like arrangement, using as little
/// space as possible, but as much as needed.
pub trait Layout<K: Eq + Hash> {
    /// Objects are optionally named. If they are,
    /// they can be retrieved through the dictionary
    /// in the result structure in `calc_graph`.
    fn name(&self) -> &Option<K>;

    /// Whether the layout or one of its contained
    /// elements has the referred name.
    fn contains_name(&self, name: &K) -> bool;

    /// Returns the padding for the object.
    fn padding(&self) -> &Padding;

    /// Indicates whether the element or one of its sub-elements
    /// requested the maximum width.
    #[inline(always)]
    fn max_width_requested(&self) -> bool { false }

    /// Indicates whether the element or one of its sub-elements
    /// requested the maximum height.
    #[inline(always)]
    fn max_height_requested(&self) -> bool { false }

    /// Clones the element as a trait object, and uses this
    /// very function to clone its subelements. It doesn't
    /// replicate the caches or fast-forwarding elements,
    /// which it's expected to leave empty.  In other
    /// words, it's a _deep-copy_ function, meant to create
    /// a _replica_ of the layout object and not a link to
    /// the same object. Implementations of `Clone` for
    /// layout objects in general use the function for its
    /// children, leading to deep-copying (sans caches
    /// or fast-forwarders) everywhere.
    fn deep_clone(&self) -> Box<dyn Layout<K>>;

    /// Calculates the naïve bounds (i.e., based solely on
    /// the width of its contained element(s)), and does so
    /// without involving the calculation of scale (though it
    /// should use `calc_naive_bounds` for its children
    /// layouts.) This allows to factor the scaling logic out
    /// into a default scaling implementation.
    fn calc_unscaled_naive_bounds(&self) -> &Dimensions<i16>;

    /// Calculates the scene graph without knowledge of
    /// scale, using the given area and parent node. This
    /// function is not to be implemented by layouts, but
    /// never called directly by either them or users;
    /// `calc_graph` should be used instead. This allows
    /// factoring out the scaling logic.
    fn calc_unscaled_graph(
        &mut self,
        usable_area : &Rect<i16>,
        parent      : Option<&mut SceneGraphFrame<f32>>
    ) -> Result<GraphCalcOutput<K>, LayoutError<K>>;

    /// Shows the scale to be used for the resulting node.
    fn scale(&self) -> &LayoutScaling;

    /// Sets the scale to be used in the resulting node.
    fn set_scale(&mut self, scale: LayoutScaling);

    /// Calculates the naïve bounds.
    fn calc_naive_bounds(&self) -> Dimensions<i16> {
        let dims = self.calc_unscaled_naive_bounds();
        Dimensions {
            width   : (dims.width  as f32 * self.scale().x1()).ceil() as i16,
            height  : (dims.height as f32 * self.scale().y1()).ceil() as i16,
        }
    }

    fn calc_graph(
        &mut self,
        scaled_area : &Rect<i16>,
        parent      : Option<&mut SceneGraphFrame<f32>>
    )
        -> Result<GraphCalcOutput<K>, LayoutError<K>>
    {
        let scale = self.scale().clone();
        let unscaled_area = Rect::new_by_dim(
            scaled_area.top,
            scaled_area.left,
            (scaled_area.height() as f32 / scale.y1()).ceil() as i16,
            (scaled_area.width()  as f32 / scale.x1()).ceil() as i16
        );
        let mut result = self.calc_unscaled_graph(&unscaled_area, parent)?;
        result.graph.set_x_scale(scale.x1());
        result.graph.set_y_scale(scale.y1());
        result.graph.set_x(((result.graph.x() - scaled_area.left) as f32 * scale.x1()).round() as i16 + scaled_area.left);
        result.graph.set_y(((result.graph.y() - scaled_area.top)  as f32 * scale.y1()).round() as i16 + scaled_area.top);
        self.save_ff_info(&result.graph);
        Ok(result)
    }

    fn save_ff_info(&mut self, graph: &SceneGraphNode<f32>);
    fn fast_forward(&mut self) -> Option<SceneGraphNode<f32>>;
}
