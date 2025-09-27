//! # Module        :   _Fast Forwarding_
//! - **Purpose**   :   Allows a `Layout` to quick
//!                     rebuild the calculated graph.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Sun 06/05/2022 03:26 PM
//! - **Modified**  :   Fri 17 Jun 2022 11:12:23 PM CST


use std::ops::Deref;

#[cfg(debug_assertions)]
use wasm_bindgen::JsValue;
#[cfg(debug_assertions)]
use web_sys::console;

use crate::base::transformable::*;
use crate::scene_graph::frame::SceneGraphFrame;
use crate::scene_graph::node::SceneGraphNode;
use crate::scene_graph::hierarchical::Hierarchical;
use crate::style::fonts::FontStyle;
use crate::drawables::text::Text;


pub trait FastForwarding {
    fn restore(&mut self);
    fn node(&self) -> &SceneGraphNode<f32>;
}


#[derive(Clone)]
pub struct NodeFastForwarding {
    state_      : TransformState<f32>,
    node_       : SceneGraphNode<f32>,
    parent_     : Option<SceneGraphFrame<f32>>,
}

impl NodeFastForwarding {
    pub fn new(node: &SceneGraphNode<f32>) -> Self {
        NodeFastForwarding {
            state_      : node.save_transform(),
            node_       : node.clone(),
            parent_     : node.parent().clone(),
        }
    }
}

impl FastForwarding for NodeFastForwarding {
    fn restore(&mut self) {
        //#[cfg(debug_assertions)]
        //console::info_1(&JsValue::from_str(&format!(
        //    "node parent_ before setting: {:?}", self.node_.parent_().is_some()
        //)));
        //#[cfg(debug_assertions)]
        //console::info_1(&JsValue::from_str(&format!(
        //    "parent_ to be set: {:?}", self.parent_.is_some()
        //)));
        //self.node_.set_parent_(self.parent_.clone());
        if let Some(mut par) = self.node_.parent().as_ref().map(|p| p.clone()) {
            par
                .remove_child(&self.node_)
                .expect(
                    "swirl::scene_graph::layouts::fast_forwarding::restore():\n\
                     error removing from parent in previous configuration."
                );
        }
        if let &mut Some(ref mut par) = &mut self.parent_ {
            par
                .append_child(self.node_.clone())
                .expect(
                    "swirl::scene_graph::layouts::fast_forwarding::restore():\n\
                     error setting parent according to recorded layout."
                );
        }
        #[cfg(debug_assertions)]
        console::info_1(&JsValue::from_str(&format!(
            "node parent_ after setting: {:?}", self.node_.parent().is_some()
        )));
        #[cfg(debug_assertions)]
        console::info_1(&JsValue::from_str(&format!(
            "before transform applied: {:?}", self.node_.update_transform()
        )));
        self.node_.restore_transform(&self.state_);
        //self.node_.set_pivot(self.pivot_.0, self.pivot_.1);
        //self.node_.set_skew(self.skew_.0,  self.skew_.1);
        //self.node_.set_scale(self.scale_.0, self.scale_.1);
        //self.node_.set_rotation(self.rotation_);
        //self.node_.set_x_and_y(self.trans_.0, self.trans_.1);
        //self.node_.update_transform();
        #[cfg(debug_assertions)]
        console::info_1(&JsValue::from_str(&format!(
            "node transform after application: {:?}", self.node_.transform()
        )));
    }

    #[inline(always)]
    fn node(&self) -> &SceneGraphNode<f32> { &self.node_ }
}


#[derive(Clone)]
pub struct TextNodeFastForwarding {
    base    : NodeFastForwarding,
    text_   : Text,
    style_  : FontStyle,
}

impl TextNodeFastForwarding {
    pub fn new(node: &SceneGraphNode<f32>, text: &Text) -> Self {
        TextNodeFastForwarding {
            base    : NodeFastForwarding::new(node),
            text_   : text.clone(),
            style_  : text.style().clone(),
        }
    }

    #[inline(always)]
    pub fn text(&self) -> &Text { &self.text_ }
}

impl Deref for TextNodeFastForwarding {
    type Target = NodeFastForwarding;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl FastForwarding for TextNodeFastForwarding {
    fn restore(&mut self) {
        self.base.restore();
        let _ = self.text_.modify_style(|text_style| {
            *text_style = self.style_.clone();
            Ok(())
        });
    }

    #[inline(always)]
    fn node(&self) -> &SceneGraphNode<f32> { self.base.node() }
}
