//! # Module        :   _Single_
//! - **Purpose**   :   Layout implementation for a layout
//!                     containing a single object.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Mon 04/11/2022 08:22 PM
//! - **Modified**  :   Mon 07/04/2022 03:12 PM


use std::collections::HashMap;
use std::hash::Hash;
use std::cell::RefCell;

#[cfg(debug_assertions)]
use wasm_bindgen::JsValue;
#[cfg(debug_assertions)]
use web_sys::console;

use crate::base::bounded::Bounded;
use crate::base::drawable::Drawable;
use crate::base::transformable::*;
use crate::math::geom::dimensions::*;
use crate::math::geom::rect::*;
use crate::drawables::text::*;
use crate::scene_graph::SceneGraph;
use crate::scene_graph::node::*;
use crate::scene_graph::frame::*;
use crate::style::fonts::{FontStyle, TextAlign};

use super::{Layout, LayoutError, GraphCalcOutput};
use super::scaling::*;
use super::alignment::*;
use super::padding::*;
use super::fast_forwarding::*;


/// Holds a single `SceneGraphNode` item to be
/// placed in the layout, whether it is provided
/// as such or constructed from `Drawable`s. 
pub struct SingleObject<Name> {
    name_   : Option<Name>,
    padding_: Padding,
    h_align_: HorizontalAlign,
    v_align_: VerticalAlign,
    text_   : Option<Text>,
    node_   : RefCell<SceneGraphNode<f32>>,
    node_tr_: TransformState<f32>,
    scale_  : LayoutScaling,
    cached  : RefCell<Option<Dimensions<i16>>>,
    ff      : Option<Box<dyn FastForwarding>>,
}

impl<Name> SingleObject<Name> {
    /// Create a `SingleObject` directly from a `SceneGraphNode`.
    pub fn new(
        name_   : Option<Name>,
        padding_: Padding,
        h_align_: HorizontalAlign,
        v_align_: VerticalAlign,
        scale_  : LayoutScaling,
        node_   : SceneGraphNode<f32>
    )
        -> Self
    {
        let node_tr_ = node_.save_transform();
        SingleObject {
            name_,
            padding_,
            node_   : RefCell::new(node_),
            node_tr_,
            h_align_,
            v_align_,
            scale_,
            text_   : None,
            cached  : RefCell::new(None),
            ff      : None,
        }
    }

    /// Produces a single element, but not from a `SceneGraphNode` but
    /// from a `Drawable`.
    #[inline]
    pub fn new_from_drawable<D: Drawable + Clone + 'static>(
        name_   : Option<Name>,
        padding_: Padding,
        h_align_: HorizontalAlign,
        v_align_: VerticalAlign,
        scale_  : LayoutScaling,
        drawable: &D
    )
        -> (Self, SceneGraphNode<f32>)
    {
        let node_ = SceneGraphNode::<f32>::new_leaf(drawable);
        (Self::new(name_, padding_, h_align_, v_align_, scale_, node_.clone()), node_)
    }

    /// Produces almost the same effect as `new_from_drawable`, but it
    /// also justifies the text_ 
    pub fn new_from_text(
        name_   : Option<Name>,
        padding_: Padding,
        h_align_: HorizontalAlign,
        v_align_: VerticalAlign,
        scale_  : LayoutScaling,
        text_   : &Text
    )
        -> (Self, SceneGraphNode<f32>)
    {
        let node_       = SceneGraphNode::<f32>::new_leaf(text_);
        let node_tr_    = node_.save_transform();
        (   SingleObject {
                name_,
                padding_,
                node_   : RefCell::new(node_.clone()),
                node_tr_,
                h_align_,
                v_align_,
                text_   : Some(text_.clone()),
                scale_,
                cached  : RefCell::new(None),
                ff      : None,
            }
        ,   node_
        )
    }

    #[inline]
    fn clean_caches(&mut self) {
        self.ff = None;
        self.cached.replace(None);
    }
}


/// Unlike `SceneGraph` objects, cloning for `Layout` does not
/// create a reference to the same object.
impl<Name: Clone> Clone for SingleObject<Name> {
    fn clone(&self) -> SingleObject<Name> {
        SingleObject {
            name_   : self.name_.clone(),
            padding_: self.padding_.clone(),
            h_align_: self.h_align_,
            v_align_: self.v_align_,
            text_   : self.text_.as_ref().map(|t| t.clone()),
            node_   : self.node_.clone(),
            node_tr_: self.node_tr_.clone(),
            scale_  : self.scale_.clone(),
            cached  : RefCell::new(None),
            ff      : None,
        }
    }
}


impl<Name: Eq + Hash + Clone + 'static> Layout<Name> for SingleObject<Name> {
    #[inline]
    fn scale(&self) -> &LayoutScaling { &self.scale_ }

    #[inline(always)]
    fn set_scale(&mut self, scale: LayoutScaling) {
        self.scale_ = scale;
        self.clean_caches();
    }

    #[inline(always)]
    fn deep_clone(&self) -> Box<dyn Layout<Name>> {
        Box::new(self.clone())
    }

    #[inline(always)]
    fn name(&self) -> &Option<Name> { &self.name_ }

    #[inline(always)]
    fn padding(&self) -> &Padding { &self.padding_ }

    fn calc_unscaled_naive_bounds(&self) -> &Dimensions<i16> {
        let cache_empty = self.cached.borrow().is_none();
        if cache_empty {
            { self.node_.borrow_mut().restore_transform(&self.node_tr_); }
            let cache   =   Dimensions {
                                width   : { self.node_.borrow().parent_width().ceil() } as i16
                                        + self.padding_.left()
                                        + self.padding_.right(),
                                height  : { self.node_.borrow().parent_height().ceil() } as i16
                                        + self.padding_.top()
                                        + self.padding_.bottom()
                            };
            self.cached.replace(Some(cache));
        }
        //#[cfg(debug_assertions)]
        //console::log_1(&JsValue::from_str(&format!(
        //    "SingleObject dimensions: {:?}", self.cached.borrow().unwrap()
        //)));
        unsafe { &*self.cached.as_ptr() }.as_ref().unwrap()
    }

    #[inline(always)]
    fn contains_name(&self, name_: &Name) -> bool {
        self.name_
            .as_ref()
            .map(|self_name| *self_name == *name_)
            .unwrap_or(false)
    }

    fn calc_unscaled_graph(
        &mut self,
        use_area: &Rect<i16>,
        parent  : Option<&mut SceneGraphFrame<f32>>
    )
        -> Result<GraphCalcOutput<Name>, LayoutError<Name>>
    {
        //#[cfg(debug_assertions)]
        //console::log_1(&JsValue::from_str(&format!(
        //    "SingleObject::calc_graph({})", use_area
        //)));
        { self.node_.borrow_mut().restore_transform(&self.node_tr_); }
        let area_width  = use_area.width();
        let area_height = use_area.height();
        let item_width  = { self.node_.borrow().parent_width().ceil() }  as i16;
        //let full_width  = item_width  + self.padding_.left() + self.padding_.right();
        let item_height = { self.node_.borrow().parent_height().ceil() } as i16;
        //let full_height = item_height + self.padding_.top()  + self.padding_.bottom();
        //if area_width < full_width || area_height < full_height {
        //    Err(LayoutError::AssignedAreaTooSmall)
        //}
        //else {
            //  There are irregular objects that don't have their beginnings at (0,0), such
            //  as `Text`s. To include these objects, it's necessary to employ a more
            //  general method, and that is using their own bounds.
            let (x, y) =
                if let Some(ref mut text_) = self.text_ {
                    let _ = match self.h_align_ {
                        HorizontalAlign::Left   =>
                            text_.modify_style(|style: &mut FontStyle| {
                                style.set_align(TextAlign::AlignLeft);
                                Ok(())
                            }),
                        HorizontalAlign::Center =>
                            text_.modify_style(|style: &mut FontStyle| {
                                style.set_align(TextAlign::AlignCenter);
                                Ok(())
                            }),
                        HorizontalAlign::Right  =>
                            text_.modify_style(|style: &mut FontStyle| {
                                style.set_align(TextAlign::AlignRight);
                                Ok(())
                            }),
                    };
                    //console::log_1(&JsValue::from_str(&format!(
                    //    "Aligns: {:?} {}",
                    //    self.h_align_,
                    //    text_.style().align()
                    //)));
                    let node_bounds = {
                        let fbounds     = self.node_
                            .borrow()
                            .bounds()
                            .ok_or_else(|| LayoutError::ErrorInSettings)?;
                        //Rect::<i16>::new_by_dim(
                        //    fbounds.top.floor()     as i16,
                        //    fbounds.left.floor()    as i16,
                        //    fbounds.height().ceil() as i16,
                        //    fbounds.width().ceil()  as i16
                        //)
                        fbounds.try_map_type::<i16>().unwrap()
                    };
                    //console::log_1(&JsValue::from_str(&format!(
                    //    "bounds of node_ surrounding text_ object: {}", &node_bounds
                    //)));
                    let x = match self.h_align_ {
                        HorizontalAlign::Left   => {
                            self.node_.borrow_mut().set_x_pivot(node_bounds.left);
                            use_area.left + self.padding_.left()
                        },
                        HorizontalAlign::Center => {
                            self.node_.borrow_mut().set_x_pivot(node_bounds.right + node_bounds.left >> 1);
                                use_area.left
                            +   self.padding_.left()
                            +   (area_width - self.padding_.left() - self.padding_.right() >> 1)
                        },
                        HorizontalAlign::Right  => {
                            self.node_.borrow_mut().set_x_pivot(node_bounds.right);
                            use_area.right - self.padding_.right()
                        }
                    };
                    let y = match self.v_align_ {
                        VerticalAlign::Top   => {
                            self.node_.borrow_mut().set_y_pivot(node_bounds.top);
                            use_area.top + self.padding_.top()
                        },
                        VerticalAlign::Middle => {
                            self.node_.borrow_mut().set_y_pivot(node_bounds.bottom + node_bounds.top >> 1);
                                use_area.top
                            +   self.padding_.top()
                            +   (area_height - self.padding_.bottom() - self.padding_.top() >> 1)
                        },
                        VerticalAlign::Bottom  => {
                            self.node_.borrow_mut().set_y_pivot(node_bounds.bottom);
                            use_area.bottom - self.padding_.bottom()
                        },
                    };
                    //console::log_1(&JsValue::from_str(&format!(
                    //    "Pivot: {},{}", self.node_.y_pivot(), self.node_.x_pivot()
                    //)));
                    (x, y)
                }
                else {
                    let x =
                        match self.h_align_ {
                            HorizontalAlign::Left   =>
                                use_area.left + self.padding_.left(),
                            HorizontalAlign::Center =>
                                    use_area.left
                                +   self.padding_.left()
                                +   (       area_width
                                        -   self.padding_.left() 
                                        -   self.padding_.right()
                                        -   item_width
                                    >>  1),
                            HorizontalAlign::Right  =>
                                use_area.right - self.padding_.right() - item_width + 1,
                        };
                    let y =
                        match self.v_align_ {
                            VerticalAlign::Top   =>
                                use_area.top + self.padding_.top(),
                            VerticalAlign::Middle =>
                                    use_area.top
                                +   self.padding_.top()
                                +   (       area_height
                                        -   self.padding_.bottom()
                                        -   self.padding_.top()
                                        -   item_height
                                    >>  1),
                            VerticalAlign::Bottom  =>
                                use_area.bottom - self.padding_.bottom() - item_height + 1,
                        };
                    (x, y)
                };
            self.node_.borrow_mut().set_x_and_y(x, y);
            if let Some(par) = parent {
                par
                    .append_child(self.node_.borrow().clone())
                    .map_err(|err| LayoutError::SceneGraphConstructionFailure(err))?;
            }
            //#[cfg(debug_assertions)]
            //console::log_1(&JsValue::from_str(&format!(
            //    "Positioned at: {},{}", x, y
            //)));
            let mut named_nodes = HashMap::<Name, SceneGraphNode<f32>>::new();
            if let &Some(ref name_) = &self.name_ {
                named_nodes.insert(name_.clone(), self.node_.borrow().clone());
            }
            //self.ff = Some(
            //    match self.text_.as_ref() {
            //        Some(text_)  => Box::new(TextNodeFastForwarding::new(&self.node_, text_)),
            //        _           => Box::new(NodeFastForwarding::new(&self.node_)),
            //    }
            //);
            Ok(GraphCalcOutput {
                graph       : self.node_.borrow().clone(),
                named_nodes,
            })
        //}
    }

    fn save_ff_info(&mut self, _: &SceneGraphNode<f32>) {
        self.ff = Some(
            match self.text_.as_ref() {
                Some(text_) => Box::new(TextNodeFastForwarding::new(&*self.node_.borrow(), text_)),
                _           => Box::new(NodeFastForwarding::new(&*self.node_.borrow())),
            }
        );
    }

    fn fast_forward(&mut self) -> Option<SceneGraphNode<f32>> {
        #[cfg(debug_assertions)]
        console::log_1(&JsValue::from_str("In Single::fast_forward"));
        if let &mut Some(ref mut ff) = &mut self.ff {
            #[cfg(debug_assertions)]
            console::log_1(&JsValue::from_str("Restoring"));
            ff.restore();
            #[cfg(debug_assertions)]
            console::log_1(&JsValue::from_str("After restoration"));
            #[cfg(debug_assertions)]
            console::log_1(&JsValue::from_str("Leaving Single::fast_forward -> Some"));
            Some(ff.node().clone())
        }
        else {
            #[cfg(debug_assertions)]
            console::log_1(&JsValue::from_str("Leaving Single::fast_forward -> None"));
            None
        }
    }
}


#[cfg(test)]
pub mod tests {
    use std::default::Default;
    use wasm_bindgen::JsValue;
    use web_sys::CanvasRenderingContext2d;

    use wasm_bindgen_test::*;

    use crate::base::bounded::Bounded;
    use crate::base::drawable::Drawable;
    use crate::math::geom::rect::Rect;
    use crate::scene_graph::SceneGraph;

    use super::*;

    #[derive(Clone, Debug)]
    pub struct NoDraw;

    impl Bounded<i16> for NoDraw {
        fn bounds(&self) -> Option<Rect<i16>> { Some(Rect::<i16>::new(0, 0, 19, 19)) }
    }

    impl Drawable for NoDraw {
        fn draw(&self, _ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> { Ok(()) }
    }

    #[wasm_bindgen_test]
    fn calc_naive_and_node() {
        let mut s = SingleObject::<String>::new(
            None,
            Padding::Uniform(20),
            HorizontalAlign::default(),
            VerticalAlign::default(),
            LayoutScaling::NoScaling,
            SceneGraphNode::<f32>::new_leaf(&NoDraw)
        );
        assert_eq!(s.calc_naive_bounds(), Dimensions::<i16> {width: 60, height: 60,});
        let GraphCalcOutput{graph, named_nodes} = s.calc_graph(&Rect::<i16> {top: 10, left: 5, bottom: 69, right: 84,}, None)
            .expect("Unable to build test node_.");
        assert_eq!(graph.parent_bounds(), Some(Rect::<f32> {top:30., left:35., bottom: 50., right: 55.,}));
        assert!(!named_nodes.contains_key("Something"));
    }
}
