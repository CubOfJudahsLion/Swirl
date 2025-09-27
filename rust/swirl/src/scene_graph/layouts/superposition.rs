//! # Module        :   _Superposition_
//! - **Purpose**   :   Objects stacked on top of one another.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Tue 04/19/2022 02:16 PM
//! - **Modified**  :   Fri 06/24/2022 12:43 AM


use std::collections::HashMap;
use std::hash::Hash;
use std::cell::RefCell;

//#[cfg(debug_assertions)]
//use wasm_bindgen::JsValue;
//#[cfg(debug_assertions)]
//use web_sys::console;

use crate::base::transformable::Transformable;
use crate::math::dimensions::Dimensions;
use crate::math::geom::rect::Rect;
use crate::scene_graph::frame::SceneGraphFrame;
use crate::scene_graph::node::SceneGraphNode;

use super::alignment::{HorizontalAlign, VerticalAlign};
use super::padding::Padding;
use super::{GraphCalcOutput, Layout, LayoutError, LayoutScaling};
use super::fast_forwarding::{FastForwarding, NodeFastForwarding};


pub struct Superposition<K> {
    name_   : Option<K>,
    padding_: Padding,
    h_align_: HorizontalAlign,
    v_align_: VerticalAlign,
    scale_  : LayoutScaling,
    elems   : Vec<Box<dyn Layout<K>>>,
    cache   : RefCell<Option<Dimensions<i16>>>,
    ff      : Option<NodeFastForwarding>,
}

impl<Name: Eq + Hash + Clone> Clone for Superposition<Name> {
    fn clone(&self) -> Self {
        Superposition {
            name_   : self.name_.clone(),
            padding_: self.padding_.clone(),
            h_align_: self.h_align_,
            v_align_: self.v_align_,
            scale_  : self.scale_.clone(),
            elems   : {
                let mut elems_v = Vec::<Box<dyn Layout<Name>>>::with_capacity(self.elems.len());
                for elem in self.elems.iter() {
                    elems_v.push(elem.deep_clone());
                }
                elems_v
            },
            cache   : RefCell::new(None),
            ff      : None,
        }
    }
}

impl<Name: Eq + Hash + Clone + 'static> Superposition<Name> {
    pub fn new(
        name_   : Option<Name>,
        padding_: Padding,
        h_align_: HorizontalAlign,
        v_align_: VerticalAlign,
        scale_  : LayoutScaling
    )
        -> Self
    {
        Superposition {
            name_,
            padding_,
            h_align_,
            v_align_,
            scale_,
            elems   : Vec::new(),
            cache   : RefCell::new(None),
            ff      : None,
        }
    }

    fn clean_caches(&mut self) {
        self.cache.replace(None);
        self.ff = None;
    }

    pub fn with_dyn(mut self, l: Box<dyn Layout<Name>>) -> Result<Self, LayoutError<Name>> {
        if let &Some(ref name) = l.name() {
            if self.contains_name(name) {
                return Err(LayoutError::DuplicateName(name.clone()));
            }
        }
        self.clean_caches();
        self.elems.push(l);
        Ok(self)
    }

    #[inline(always)]
    pub fn with<L: Layout<Name> + Clone + 'static>(self, l: &L) -> Result<Self, LayoutError<Name>> {
        self.with_dyn(Box::new(l.clone()) as Box<dyn Layout<Name>>)
    }

    pub fn add_dyn(&mut self, elem: Box<dyn Layout<Name>>) -> Result<&mut Self, LayoutError<Name>> {
        if let &Some(ref name) = elem.name() {
            if self.contains_name(name) {
                return Err(LayoutError::DuplicateName(name.clone()));
            }
        }
        self.clean_caches();
        self.elems.push(elem);
        Ok(self)
    }

    #[inline(always)]
    pub fn add<L: Layout<Name> + Clone + 'static>(&mut self, elem: &L) -> Result<&mut Self, LayoutError<Name>> {
        self.add_dyn(Box::new(elem.clone()))
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding_ = padding;
        self.clean_caches();
    }
}

impl<Name: Eq + Hash + Clone + 'static> Layout<Name> for Superposition<Name> {
    #[inline(always)]
    fn name(&self) -> &Option<Name> { &self.name_ }

    fn contains_name(&self, name_: &Name) -> bool {
            Some(name_) == self.name_.as_ref()
        ||  self.elems
                .iter()
                .any(|layout| layout.contains_name(name_))
    }

    fn max_height_requested(&self) -> bool {
        self.elems
            .iter()
            .any(|layout| layout.max_height_requested())
    }

    fn max_width_requested(&self) -> bool {
        self.elems
            .iter()
            .any(|layout| layout.max_width_requested())
    }

    #[inline(always)]
    fn deep_clone(&self) -> Box<dyn Layout<Name>> {
        Box::new(self.clone())
    }

    #[inline(always)]
    fn padding(&self) -> &Padding { &self.padding_ }

    #[inline(always)]
    fn scale(&self) -> &LayoutScaling { &self.scale_ }

    fn set_scale(&mut self, scale: LayoutScaling) {
        self.scale_ = scale;
        self.clean_caches();
    }

    fn calc_unscaled_naive_bounds(&self) -> &Dimensions<i16> {
        let no_cache = self.cache.borrow().is_none();
        if no_cache {
            let (h, v) = self
                .elems
                .iter()
                .map(|layout| layout.calc_naive_bounds())   // use the scale-aware version for children elements
                .fold((0, 0), |(acc_h, acc_v), dims| {
                    (i16::max(acc_h, dims.width), i16::max(acc_v, dims.height))
                });
            let cache_dims  = Dimensions::new(
                h + self.padding_.left() + self.padding_.right(),
                v + self.padding_.top()  + self.padding_.bottom()
            );
            self.cache.replace(Some(cache_dims));
        }
        unsafe { &*self.cache.as_ptr() }.as_ref().unwrap()
    }

    fn calc_unscaled_graph(
        &mut self,
        usable_area : &Rect<i16>,
        parent      : Option<&mut SceneGraphFrame<f32>>
    )
        -> Result<GraphCalcOutput<Name>, LayoutError<Name>>
    {
        //#[cfg(debug_assertions)]
        //console::log_1(&JsValue::from_str(&format!(
        //    "Superposition::calc_graph({})", usable_area
        //)));
        let usable_area_width   = usable_area.width();
        let usable_area_height  = usable_area.height();
        if      usable_area_width  < self.calc_unscaled_naive_bounds().width
            ||  usable_area_height < self.calc_unscaled_naive_bounds().height
        {
            Err(LayoutError::AssignedAreaTooSmall)
        }
        else {
            let inner_width     = usable_area_width  - self.padding_.left() - self.padding_.right();
            let inner_height    = usable_area_height - self.padding_.top()  - self.padding_.bottom();
            if inner_width < 0 || inner_height < 0 {
                Err(LayoutError::ErrorInSettings)
            }
            else {
                let ( left
                    , width
                    , top
                    , height
                    )               =
                {
                    let inner_top       =   usable_area.top   + self.padding_.top();
                    let inner_left      =   usable_area.left  + self.padding_.left();
                    let dims            =   self.calc_unscaled_naive_bounds();
                    let (left, width)   =   if self.max_width_requested() {
                        (inner_left, inner_width)
                    }
                    else {
                        (   match self.h_align_ {
                                HorizontalAlign::Left   =>
                                    inner_left,
                                HorizontalAlign::Right  => {
                                    let inner_right_1past = inner_left + inner_width;
                                    inner_right_1past - dims.width
                                },
                                HorizontalAlign::Center =>
                                    inner_left + (inner_width - dims.width >> 1),
                            }
                        ,   dims.width
                        )
                    };
                    let (top, height)   =   if self.max_height_requested() {
                        (inner_top, inner_height)
                    }
                    else {
                        (   match self.v_align_ {
                                VerticalAlign::Top      =>
                                    inner_top,
                                VerticalAlign::Bottom   => {
                                    let inner_bottom_1past = inner_top + inner_height;
                                    inner_bottom_1past - dims.height
                                },
                                VerticalAlign::Middle   =>
                                    inner_top + (inner_height - dims.height >> 1),
                            }
                        ,   dims.height
                        )
                    };
                    (left, width, top, height)
                };
                let mut frame       =   SceneGraphFrame::<f32>::new();
                frame.set_x_and_y(left, top);
                let graph           =   SceneGraphNode::Frame(frame.clone());
                if let Some(par) = parent {
                    par
                        .append_child(graph.clone())
                        .map_err(|err| LayoutError::SceneGraphConstructionFailure(err))?;
                }
                let mut named_nodes =   HashMap::<Name, SceneGraphNode<f32>>::new();
                if let &Some(ref name_) = &self.name_ {
                    named_nodes.insert(name_.clone(), graph.clone());
                }
                let sub_area        =   Rect::new_by_dim(0, 0, height, width);
                for sub_layout in self.elems.iter_mut() {
                    let mut sub_out = sub_layout.calc_graph(&sub_area, Some(&mut frame))?;
                    for (k, v) in sub_out.named_nodes.drain() {
                        if named_nodes.contains_key(&k) {
                            return Err(LayoutError::DuplicateName(k));
                        }
                        named_nodes.insert(k, v);
                    }
                }
                //self.ff = Some(NodeFastForwarding::new(&graph));
                Ok(GraphCalcOutput {
                    graph,
                    named_nodes,
                })
            }
        }
    }

    fn save_ff_info(&mut self, graph: &SceneGraphNode<f32>) {
        self.ff = Some(NodeFastForwarding::new(graph));
    }

    fn fast_forward(&mut self) -> Option<SceneGraphNode<f32>> {
        if let Some(ff) = self.ff.as_mut() {
            for elem in self.elems.iter_mut() {
                if elem.fast_forward().is_none() {
                    return None;
                }
            }
            ff.restore();
            Some(ff.node().clone())
        }
        else {
            None
        }
    }
}


#[cfg(test)]
pub mod testing {
    use wasm_bindgen_test::*;
    use wasm_bindgen::JsValue;
    use web_sys::CanvasRenderingContext2d;

    use crate::base::bounded::Bounded;
    use crate::base::drawable::Drawable;
    use crate::style::color;
    use crate::scene_graph::SceneGraph;

    use crate::math::rect::Rect;

    use super::*;
    use super::super::LayoutScaling;
    use super::super::single::SingleObject;


    #[derive(Clone, Debug)]
    pub struct ThickFrame {
        thickness: i16,
        width: i16,
        height: i16,
        color: color::Color,
    }

    impl Bounded<i16> for ThickFrame {
        fn bounds(&self) -> Option<Rect<i16>> {
            Some(Rect {
                top     : 0,
                left    : 0,
                right   : self.width  - 1,
                bottom  : self.height - 1,
            })
        }
    }

    impl Drawable for ThickFrame {
        fn draw(&self, cnv: &CanvasRenderingContext2d) -> Result<(), JsValue> {
            cnv.set_fill_style(&JsValue::from_str(&self.color.to_string()));
            let mut bounds = self.bounds().ok_or_else(|| {
                JsValue::from_str("Unable to extract concrete bounds for ThickFrame")
            })?;
            cnv.fill_rect(
                bounds.left     as f64,
                bounds.top      as f64,
                bounds.width()  as f64,
                bounds.height() as f64,
            );
            if self.thickness << 1 < self.width {
                bounds.left     += self.thickness;
                bounds.top      += self.thickness;
                bounds.right    -= self.thickness;
                bounds.bottom   -= self.thickness;
                cnv.clear_rect(
                    bounds.left     as f64,
                    bounds.top      as f64,
                    bounds.width()  as f64,
                    bounds.height() as f64,
                );
            }
            Ok(())
        }
    }


    #[wasm_bindgen_test]
    fn dims() {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        enum RGB {
            Red, Green, Blue,
        }
        //
        let red             =   ThickFrame {thickness: 4, width: 30, height: 10, color: color::Color::RGB(255, 0, 0),};
        let green           =   ThickFrame {thickness: 4, width: 15, height: 23, color: color::Color::RGB(0, 255, 0),};
        let blue            =   ThickFrame {thickness: 4, width: 20, height: 20, color: color::Color::RGB(0, 0, 255),};
        let mut sup         =   Superposition::new(
            None,
            Padding::NoPadding,
            HorizontalAlign::Center,
            VerticalAlign::Middle,
            LayoutScaling::NoScaling
        )
            .with(&SingleObject::new(
                Some(RGB::Red),
                Padding::NoPadding,
                HorizontalAlign::Center,
                VerticalAlign::Bottom,
                LayoutScaling::NoScaling,
                SceneGraphNode::new_leaf(&red)
            ))
            .and_then(|layout| layout.with(&SingleObject::new_from_drawable(
                Some(RGB::Green),
                Padding::Uniform(3),
                HorizontalAlign::Left,
                VerticalAlign::Middle,
                LayoutScaling::NoScaling,
                &green
            ).0 ))
            .unwrap();
        sup
            .add(
                &SingleObject::new_from_drawable(
                    Some(RGB::Blue),
                    Padding::Directional(Dimensions {width: 8, height: 2}),
                    HorizontalAlign::Right,
                    VerticalAlign::Top,
                    LayoutScaling::NoScaling,
                    &blue
                ).0,
            )
            .unwrap();
        let dims            =   sup.calc_naive_bounds();
        assert_eq!(dims, Dimensions {width: 36i16, height: 29,});
        let calc_out        =   sup.calc_graph(&Rect::<i16> {left: 25, top: -8, right: 64, bottom: 24,}, None).unwrap();
        assert_eq!(calc_out.graph.parent_bounds(), Some(Rect::<f32> {left: 28., top: -6., right: 60., bottom: 25.}));
    }
}
