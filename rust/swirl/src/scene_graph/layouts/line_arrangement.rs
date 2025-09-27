/// # Module        :   _Lineup_
/// - **Purpose**   :   A collinear arrangement of items,
///                     either horizontal (i.e, a row) or
///                     vertical (a column.)
/// - **Version**   :   0.1.0
/// - **Author**    :   Alex Feterman
/// - **Created**   :   Sun 24 Apr 2022 11:01:34 AM CST
/// - **Modified**  :   Mon 05/30/2022 09:58 PM


use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

use crate::ConstraintType;
use crate::base::transformable::Transformable;

use crate::math::geom::dimensions::Dimensions;
use crate::math::geom::rect::Rect;

use super::{Layout, LayoutError, ZOrder, GraphCalcOutput};
use super::scaling::LayoutScaling;
use super::alignment::*;
use super::constraints::*;
use super::padding::Padding;
use super::fast_forwarding::{FastForwarding, NodeFastForwarding};

use crate::scene_graph::frame::*;
use crate::scene_graph::node::SceneGraphNode;


//  Internal caching structure. Keeps dimensions
//  and other pre-calculated widths.
#[derive(Clone)]
struct Cache {
    part            : f32,              //  Maximum part width of items constrained to HorizontalParts.
    part_cnt        : i16,              //  Total of parts.
    parts_ask_max   : bool,             //  Whether sub-layouts of a `*Parts`-constrained layout
                                        //  have requested for a maximum width.
    max_cnt         : i16,              //  Number of items constrained to MaxWidth.
    fixed           : i16,              //  Sum of the widths of fixed-width elements.
    dims            : Dimensions<i16>,  //  Naive dimensions, in case requests for them are repeated
                                        //  (which they often are.)
}


struct Element<Name, Constr> {
    constr  : Constr,
    z_order : ZOrder,
    layout  : Box<dyn Layout<Name>>,
}

impl<Name, Constr> Element<Name, Constr> {
    #[inline(always)]
    fn new(
        constr  : Constr,
        z_order : ZOrder,
        layout  : Box<dyn Layout<Name>>,
    )
        -> Self
    {
        Element {
            constr,
            z_order,
            layout,
        }
    }
}


pub struct LineArrangement<Name, Constraints> {
    name    :   Option<Name>,
    padding :   Padding,
    elems   :   Vec<Element<Name, Constraints>>,
    h_align :   HorizontalAlign,
    v_align :   VerticalAlign,
    scale_  :   LayoutScaling,
    ff      :   Option<NodeFastForwarding>,
    cache   :   RefCell<Option<Cache>>,    // Widest relative-part and dimension
}


impl<Name, Constraints> Clone for LineArrangement<Name, Constraints>
    where   Name        : Eq + Hash + Clone + 'static,
            Constraints : Clone,
            Self        : Layout<Name>
{
    fn clone(&self) -> Self {
        LineArrangement {
            name    : self.name.clone(),
            padding : self.padding.clone(),
            elems   : self.elems
                .iter()
                .map(|elem| Element {
                    constr  : elem.constr.clone(),
                    z_order : elem.z_order,
                    layout  : elem.layout.deep_clone(),
                })
                .collect::<Vec<_>>(),
            h_align : self.h_align.clone(),
            v_align : self.v_align.clone(),
            scale_  : self.scale_.clone(),
            ff      : None,
            cache   : RefCell::new(None),
        }
    }
}


impl<Name, Constraints> LineArrangement<Name, Constraints>
    where   Self        : Layout<Name>,
            Name        : Eq + Hash + Clone,
            Constraints : ConstraintType
{
    pub fn new(
        name    : Option<Name>,
        padding : Padding,
        h_align : HorizontalAlign,
        v_align : VerticalAlign,
        scale   : LayoutScaling
    )
        -> Self
    {
        LineArrangement {
            name,
            elems   : Vec::new(),
            padding,
            h_align,
            v_align,
            scale_  : scale,
            ff      : None,
            cache   : RefCell::new(None),
        }
    }

    fn clean_caches(&mut self) {
        self.ff = None;
        self.cache.replace(None);
    }

    pub fn with_dyn(
        mut self,
        c: Constraints,
        z: ZOrder,
        l: Box<dyn Layout<Name>>
    )
        -> Result<Self, LayoutError<Name>>
    {
        if let &Some(ref name) = l.name() {
            if self.contains_name(name) {
                return Err(LayoutError::DuplicateName(name.clone()));
            }
        }
        self.elems.push(Element::new(c, z, l));
        self.clean_caches();
        Ok(self)
    }

    #[inline(always)]
    pub fn with<L: Layout<Name> + Clone + 'static>(
        self,
        c: Constraints,
        z: ZOrder,
        l: &L
    )
        -> Result<Self, LayoutError<Name>>
    {
        self.with_dyn(c, z, Box::new(l.clone()))
    }

    pub fn add_dyn(
        &mut self,
        c: Constraints,
        z: ZOrder,
        l: Box<dyn Layout<Name>>
    )
        -> Result<&mut Self, LayoutError<Name>>
    {
        if let &Some(ref name) = l.name() {
            if self.contains_name(name) {
                return Err(LayoutError::DuplicateName(name.clone()));
            }
        }
        self.elems.push(Element::new(c, z, l));
        self.clean_caches();
        Ok(self)
    }

    #[inline(always)]
    pub fn add<L: Layout<Name> + Clone + 'static>(
        &mut self,
        c: Constraints,
        z: ZOrder,
        l: &L
    )
        -> Result<&mut Self, LayoutError<Name>>
    {
        self.add_dyn(c, z, Box::new(l.clone()))
    }

    #[inline(always)]
    fn padding_(&self) -> &Padding { &self.padding }

    #[inline(always)]
    fn name_(&self) -> &Option<Name> { &self.name }

    fn contains_name_(&self, name: &Name) -> bool {
            self.name
                .as_ref()
                .map(|self_name| *self_name == *name)
                .unwrap_or(false)
        ||  self.elems
                .iter()
                .any(|elem| elem.layout.contains_name(name))
    }

    fn scale__(&self) -> &LayoutScaling {
        &self.scale_
    }

    fn set_scale__(&mut self, scale: LayoutScaling) {
        self.scale_ = scale;
        self.clean_caches();
    }

    fn save_ff_info_(&mut self, graph: &SceneGraphNode<f32>) {
        self.ff = Some(NodeFastForwarding::new(graph));
    }

    fn fast_forward_(&mut self) -> Option<SceneGraphNode<f32>> {
        if let Some(ff) = &mut self.ff {
            for elem in self.elems.iter_mut() {
                if elem.layout.fast_forward().is_none() {
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


impl<Name: Eq + Hash + Clone + 'static> Layout<Name> for LineArrangement<Name, WidthConstraints> {
    #[inline(always)]
    fn deep_clone(&self) -> Box<dyn Layout<Name>> { Box::new(self.clone()) }

    #[inline(always)]
    fn scale(&self) -> &LayoutScaling { self.scale__() }

    #[inline(always)]
    fn set_scale(&mut self, scale: LayoutScaling) { self.set_scale__(scale); }

    #[inline(always)]
    fn save_ff_info(&mut self, graph: &SceneGraphNode<f32>) { self.save_ff_info_(graph); }

    #[inline(always)]
    fn fast_forward(&mut self) -> Option<SceneGraphNode<f32>> { self.fast_forward_() }

    #[inline(always)]
    fn name(&self) -> &Option<Name> { self.name_() }

    #[inline(always)]
    fn contains_name(&self, name: &Name) -> bool { self.contains_name_(name) }

    #[inline(always)]
    fn padding(&self) -> &Padding { self.padding_() }

    fn calc_unscaled_naive_bounds(&self) -> &Dimensions<i16> {
        let cache_empty = self.cache.borrow().is_none();
        if cache_empty {
            let pre_calc        = self.elems
                .iter()
                .map(|elem| 
                    (   elem.constr
                    ,   if let WidthConstraints::HorizontalParts(_) = elem.constr {
                            elem.layout.max_width_requested()
                        }
                        else {
                            false
                        }
                    ,   elem.layout.calc_naive_bounds()
                    )
                )
                .collect::<Vec<_>>();
            let ( part
                , part_cnt
                , parts_ask_max
                , max
                , max_cnt
                , fixed
                , height
                )               = pre_calc
                .iter()
                .fold(
                    (0., 0, false, 0, 0, 0, 0),
                    | (part, part_cnt, parts_ask_max, max, max_cnt, fixed, height)
                    , (constr, ask_max, dims)
                    | {
                        use WidthConstraints::*;
                        match constr {
                            HorizontalParts(parts)  =>
                                (   f32::max(part, dims.width as f32 / *parts as f32)
                                ,   part_cnt + *parts
                                ,   parts_ask_max || *ask_max
                                ,   max
                                ,   max_cnt
                                ,   fixed
                                ,   i16::max(height, dims.height),
                                ),
                            MaxWidth                =>
                                (   part
                                ,   part_cnt
                                ,   parts_ask_max
                                ,   i16::max(max, dims.width)
                                ,   max_cnt + 1
                                ,   fixed
                                ,   i16::max(height, dims.height),
                                ),
                            MinWidth                =>
                                (   part
                                ,   part_cnt
                                ,   parts_ask_max
                                ,   max
                                ,   max_cnt
                                ,   fixed + dims.width
                                ,   i16::max(height, dims.height),
                                ),
                            MinPixelWidth(pixels)   =>
                                (   part
                                ,   part_cnt
                                ,   parts_ask_max
                                ,   max
                                ,   max_cnt
                                ,   fixed + i16::max(*pixels, dims.width)
                                ,   i16::max(height, dims.height),
                                ),
                        }
                    }
                );
            let width           = fixed
                                + (part * part_cnt as f32).ceil() as i16
                                + max   * max_cnt;
            let dims            = Dimensions {
                width   : width  + self.padding.left() + self.padding.right(),
                height  : height + self.padding.top()  + self.padding.bottom(),
            };
            self.cache.replace(Some(Cache {
                part, part_cnt, parts_ask_max,
                max_cnt,
                dims: dims.clone(),
                fixed,
            }));
        }
        unsafe { &*self.cache.as_ptr() }.as_ref().map(|cache| &cache.dims).unwrap()
    }

    fn max_width_requested(&self) -> bool {
        self.elems
            .iter()
            .any(|elem| match elem.constr {
                WidthConstraints::MaxWidth              =>
                    true,
                WidthConstraints::HorizontalParts(_)    =>
                    elem.layout.max_width_requested(),
                _                                       =>
                    false,
            })
    }

    fn max_height_requested(&self) -> bool {
        self.elems.iter().any(|elem| elem.layout.max_height_requested())
    }

    fn calc_unscaled_graph(
        &mut self,
        usable_area : &Rect<i16>,
        parent      : Option<&mut SceneGraphFrame<f32>>
    )
        -> Result<GraphCalcOutput<Name>, LayoutError<Name>>
    {
        let naive_bounds        = self.calc_unscaled_naive_bounds();
        let cached              = self.cache.borrow().as_ref().unwrap().clone();
        let usable_area_width   = usable_area.width();
        let usable_area_height  = usable_area.height();
        if      usable_area_width  < naive_bounds.width
            ||  usable_area_height < naive_bounds.height
        {
            Err(LayoutError::AssignedAreaTooSmall)
        }
        else {
            let inner_area_width        =   usable_area_width  - self.padding.left() - self.padding.right();
            let inner_area_height       =   usable_area_height - self.padding.top()  - self.padding.bottom();
            //if inner_area_width < 0 || inner_area_height < 0 {
            //    Err(LayoutError::ErrorInSettings)
            //}
            //else {
            let (width_for_parts, new_part) =
                if cached.max_cnt == 0 && cached.parts_ask_max {
                    let full_parts  = (inner_area_width - cached.fixed) as f32;
                    (full_parts, full_parts / cached.part_cnt as f32)
                }
                else {
                    (cached.part * cached.part_cnt as f32, cached.part)
                };
            let (width_for_max, new_max)    =
                if cached.max_cnt == 0 {
                    (0.0f32, 0.0f32)
                }
                else {
                    let full_max    =   inner_area_width as f32 - width_for_parts - cached.fixed as f32;
                    (full_max, full_max / cached.max_cnt as f32)
                };
            let total_width         =   (   cached.fixed as f32
                                        +   width_for_parts
                                        +   width_for_max
                                        ).ceil() as i16;
            let (frame_x, frame_y)  =   {
                let inner_area_top      = usable_area.top  + self.padding.top();
                let inner_area_left     = usable_area.left + self.padding.left();
                let inner_area_bottom   = inner_area_top   + inner_area_height - 1;
                let inner_area_right    = inner_area_left  + inner_area_width  - 1;
                let left                =
                    match self.h_align {
                        HorizontalAlign::Left   =>
                            inner_area_left,
                        HorizontalAlign::Right  =>
                            inner_area_right - total_width + 1,
                        HorizontalAlign::Center =>
                            inner_area_left + (inner_area_width - total_width >> 1),
                    };
                let top                 =
                    match self.v_align {
                        VerticalAlign::Top      =>
                            inner_area_top,
                        VerticalAlign::Bottom   =>
                            inner_area_bottom - cached.dims.height + 1,
                        VerticalAlign::Middle   =>
                            inner_area_top + (inner_area_height - cached.dims.height >> 1),
                    };
                (left, top)
            };
            let mut frame           =   SceneGraphFrame::new();
            frame.set_x_and_y(frame_x, frame_y);
            let graph               =   SceneGraphNode::Frame(frame.clone());
            if let Some(par) = parent {
                par
                    .append_child(graph.clone())
                    .map_err(|err| LayoutError::SceneGraphConstructionFailure(err))?;
            }
            let mut named_nodes     =   HashMap::<Name, SceneGraphNode<f32>>::new();
            if let &Some(ref name) = &self.name {
                named_nodes.insert(name.clone(), graph.clone());
            }
            let mut start_pos       =   0i16;
            let mut accum_err       =   0.0f32;
            let mut elem_positions  =   self.elems
                .iter_mut()
                .map(|elem| {
                    let sub_dims                 = elem.layout.calc_naive_bounds();
                    let full_elem_width          =
                        match elem.constr {
                            WidthConstraints::MinWidth                  =>
                                sub_dims.width,
                            WidthConstraints::MinPixelWidth(pixels)     =>
                                i16::max(pixels, sub_dims.width),
                            WidthConstraints::HorizontalParts(h_parts)  => {
                                let width   = new_part * h_parts as f32 + accum_err;
                                accum_err   = width.fract();
                                width.floor() as i16
                            },
                            WidthConstraints::MaxWidth                  => {
                                let width   = new_max + accum_err;
                                accum_err   = width.fract();
                                width.floor() as i16
                            },
                        };
                    let (left, right)            = (start_pos, start_pos + full_elem_width - 1);
                    start_pos                   += full_elem_width;
                    let (top, bottom)            = (0, cached.dims.height - 1);
                    (elem, Rect {top, left, bottom, right})
                })
                .collect::<Vec<_>>();
            elem_positions.sort_by(|(e1, _), (e2, _)| ZOrder::cmp(&e1.z_order, &e2.z_order));
            for (elem, ref rect) in elem_positions.drain(..) {
                let GraphCalcOutput {
                    named_nodes : mut sub_named_nodes,
                    ..
                }   = elem.layout.calc_graph(rect, Some(&mut frame))?;
                for (sub_name, sub_node) in sub_named_nodes.drain() {
                    if named_nodes.contains_key(&sub_name) {
                        return Err(LayoutError::DuplicateName(sub_name));
                    }
                    else {
                        named_nodes.insert(sub_name, sub_node);
                    }
                }
            }
            //for &mut Element {constr, z_order, ref mut layout,} in self.elems.borrow_mut().iter_mut() {
            //    let sub_dims                 = layout.calc_naive_bounds();
            //    let full_elem_width          =
            //        match constr {
            //            WidthConstraints::MinWidth                  =>
            //                sub_dims.width,
            //            WidthConstraints::MinPixelWidth(pixels)     =>
            //                i16::max(pixels, sub_dims.width),
            //            WidthConstraints::HorizontalParts(h_parts)  => {
            //                let width   = new_part * h_parts as f32 + accum_err;
            //                accum_err   = width.fract();
            //                width.floor() as i16
            //            },
            //            WidthConstraints::MaxWidth                  => {
            //                let width   = new_max + accum_err;
            //                accum_err   = width.fract();
            //                width.floor() as i16
            //            },
            //        };
            //    let (left, right)            = (start_pos, start_pos + full_elem_width - 1);
            //    start_pos                   += full_elem_width;
            //    let (top, bottom)            = (0, cached.dims.height - 1);
            //    let GraphCalcOutput {
            //        named_nodes : mut sub_named_nodes,
            //        graph       : mut sub_graph,
            //    }                            = layout.calc_graph(&Rect {top, left, bottom, right,})?;
            //    if let &Some(ref name) = layout.name() {
            //        if named_nodes.contains_key(name) {
            //            return Err(LayoutError::DuplicateName(name.clone()));
            //        }
            //        else {
            //            named_nodes.insert(name.clone(), sub_graph.clone());
            //        }
            //    }
            //    for (sub_name, sub_node) in sub_named_nodes.drain() {
            //        if named_nodes.contains_key(&sub_name) {
            //            return Err(LayoutError::DuplicateName(sub_name));
            //        }
            //        else {
            //            named_nodes.insert(sub_name, sub_node);
            //        }
            //    }
            //    node_vec.push(
            //        (   z_order
            //        ,   sub_graph
            //        )
            //    );
            //}
            //node_vec.sort_by(|(z1, _), (z2, _)| ZOrder::cmp(z1, z2));
            //for (_, node) in node_vec.drain(..) {
            //    frame.append_child(node)
            //        .map_err(|err| LayoutError::SceneGraphConstructionFailure(err))?;
            //}
            Ok(GraphCalcOutput {
                graph,
                named_nodes,
            })
            //}
        }
    }
}


impl<Name: Eq + Hash + Clone + 'static> Layout<Name> for LineArrangement<Name, HeightConstraints> {
    #[inline(always)]
    fn deep_clone(&self) -> Box<dyn Layout<Name>> { Box::new(self.clone()) }

    #[inline(always)]
    fn scale(&self) -> &LayoutScaling { self.scale__() }

    #[inline(always)]
    fn set_scale(&mut self, scale: LayoutScaling) { self.set_scale__(scale); }

    #[inline(always)]
    fn save_ff_info(&mut self, graph: &SceneGraphNode<f32>) { self.save_ff_info_(graph); }

    #[inline(always)]
    fn fast_forward(&mut self) -> Option<SceneGraphNode<f32>> { self.fast_forward_() }

    #[inline(always)]
    fn name(&self) -> &Option<Name> { self.name_() }

    #[inline(always)]
    fn contains_name(&self, name: &Name) -> bool { self.contains_name_(name) }

    #[inline(always)]
    fn padding(&self) -> &Padding { self.padding_() }

    fn calc_unscaled_naive_bounds(&self) -> &Dimensions<i16> {
        let cache_empty = self.cache.borrow().is_none();
        if cache_empty {
            let pre_calc        = self.elems
                .iter()
                .map(|elem| 
                    (   elem.constr
                    ,   if let HeightConstraints::VerticalParts(_) = elem.constr {
                            elem.layout.max_height_requested()
                        }
                        else {
                            false
                        }
                    ,   elem.layout.calc_naive_bounds()
                    )
                )
                .collect::<Vec<_>>();
            let ( part
                , part_cnt
                , parts_ask_max
                , max
                , max_cnt
                , fixed
                , width
                )               = pre_calc
                .iter()
                .fold(
                    (0., 0, false, 0, 0, 0, 0),
                    | (part, part_cnt, parts_ask_max, max, max_cnt, fixed, width)
                    , (constr, ask_max, dims)
                    | {
                        use HeightConstraints::*;
                        match constr {
                            VerticalParts(parts)  =>
                                (   f32::max(part, dims.height as f32 / *parts as f32)
                                ,   part_cnt + *parts
                                ,   parts_ask_max || *ask_max
                                ,   max
                                ,   max_cnt
                                ,   fixed
                                ,   i16::max(width, dims.width),
                                ),
                            MaxHeight               =>
                                (   part
                                ,   part_cnt
                                ,   parts_ask_max
                                ,   i16::max(max, dims.height)
                                ,   max_cnt + 1
                                ,   fixed
                                ,   i16::max(width, dims.width),
                                ),
                            MinHeight               =>
                                (   part
                                ,   part_cnt
                                ,   parts_ask_max
                                ,   max
                                ,   max_cnt
                                ,   fixed + dims.height
                                ,   i16::max(width, dims.width),
                                ),
                            MinPixelHeight(pixels)  =>
                                (   part
                                ,   part_cnt
                                ,   parts_ask_max
                                ,   max
                                ,   max_cnt
                                ,   fixed + i16::max(*pixels, dims.height)
                                ,   i16::max(width, dims.width),
                                ),
                        }
                    }
                );
            let height          = fixed
                                + (part * part_cnt as f32).ceil() as i16
                                + max   * max_cnt;
            let dims            = Dimensions {
                width   : width  + self.padding.left() + self.padding.right(),
                height  : height + self.padding.top()  + self.padding.bottom(),
            };
            self.cache.replace(Some(Cache {
                part, part_cnt, parts_ask_max,
                max_cnt,
                dims: dims.clone(),
                fixed,
            }));
        }
        unsafe { &*self.cache.as_ptr() }.as_ref().map(|cache| &cache.dims).unwrap()
    }

    fn max_height_requested(&self) -> bool {
        self.elems
            .iter()
            .any(|elem| match elem.constr {
                HeightConstraints::MaxHeight        =>
                    true,
                HeightConstraints::VerticalParts(_) =>
                    elem.layout.max_height_requested(),
                _                                   =>
                    false,
            })
    }

    fn max_width_requested(&self) -> bool {
        self.elems.iter().any(|elem| elem.layout.max_width_requested())
    }

    fn calc_unscaled_graph(
        &mut self,
        usable_area : &Rect<i16>,
        parent      : Option<&mut SceneGraphFrame<f32>>
    )
        -> Result<GraphCalcOutput<Name>, LayoutError<Name>>
    {
        let naive_bounds        = self.calc_unscaled_naive_bounds();
        let cached              = self.cache.borrow().as_ref().unwrap().clone();
        let usable_area_width   = usable_area.width();
        let usable_area_height  = usable_area.height();
        if      usable_area_width  < naive_bounds.width
            ||  usable_area_height < naive_bounds.height
        {
            Err(LayoutError::AssignedAreaTooSmall)
        }
        else {
            let inner_area_width        =   usable_area_width  - self.padding.left() - self.padding.right();
            let inner_area_height       =   usable_area_height - self.padding.top()  - self.padding.bottom();
            //if inner_area_width < 0 || inner_area_height < 0 {
            //    Err(LayoutError::ErrorInSettings)
            //}
            //else {
            let (height_for_parts, new_part) =
                if cached.max_cnt == 0 && cached.parts_ask_max {
                    let full_parts  = (inner_area_height - cached.fixed) as f32;
                    (full_parts, full_parts / cached.part_cnt as f32)
                }
                else {
                    (cached.part * cached.part_cnt as f32, cached.part)
                };
            let (height_for_max, new_max)    =
                if cached.max_cnt == 0 {
                    (0.0f32, 0.0f32)
                }
                else {
                    let full_max    =   inner_area_height as f32 - height_for_parts - cached.fixed as f32;
                    (full_max, full_max / cached.max_cnt as f32)
                };
            let total_height        =   (   cached.fixed as f32
                                        +   height_for_parts
                                        +   height_for_max
                                        ).ceil() as i16;
            let (frame_x, frame_y)  =   {
                let inner_area_top      = usable_area.top  + self.padding.top();
                let inner_area_left     = usable_area.left + self.padding.left();
                let inner_area_bottom   = inner_area_top   + inner_area_height - 1;
                let inner_area_right    = inner_area_left  + inner_area_width  - 1;
                let left                =
                    match self.h_align {
                        HorizontalAlign::Left   =>
                            inner_area_left,
                        HorizontalAlign::Right  =>
                            inner_area_right - cached.dims.width + 1,
                        HorizontalAlign::Center =>
                            inner_area_left + (inner_area_width - cached.dims.width >> 1),
                    };
                let top                 =
                    match self.v_align {
                        VerticalAlign::Top      =>
                            inner_area_top,
                        VerticalAlign::Bottom   =>
                            inner_area_bottom - total_height + 1,
                        VerticalAlign::Middle   =>
                            inner_area_top + (inner_area_height - total_height >> 1),
                    };
                (left, top)
            };
            let mut frame           =   SceneGraphFrame::new();
            frame.set_x_and_y(frame_x, frame_y);
            let graph               =   SceneGraphNode::Frame(frame.clone());
            if let Some(par) = parent {
                par
                    .append_child(graph.clone())
                    .map_err(|err| LayoutError::SceneGraphConstructionFailure(err))?;
            }
            let mut named_nodes     =   HashMap::<Name, SceneGraphNode<f32>>::new();
            if let &Some(ref name) = &self.name {
                named_nodes.insert(name.clone(), graph.clone());
            }
            let mut start_pos       =   0i16;
            let mut accum_err       =   0.0f32;
            let mut elem_positions  =   self.elems
                .iter_mut()
                .map(|elem| {
                    let sub_dims                 = elem.layout.calc_naive_bounds();
                    let full_elem_height         =
                        match elem.constr {
                            HeightConstraints::MinHeight                =>
                                sub_dims.height,
                            HeightConstraints::MinPixelHeight(pixels)   =>
                                i16::max(pixels, sub_dims.height),
                            HeightConstraints::VerticalParts(parts)     => {
                                let height  = new_part * parts as f32 + accum_err;
                                accum_err   = height.fract();
                                height.floor() as i16
                            },
                            HeightConstraints::MaxHeight                => {
                                let height  = new_max + accum_err;
                                accum_err   = height.fract();
                                height.floor() as i16
                            },
                        };
                    let (top, bottom)            = (start_pos, start_pos + full_elem_height - 1);
                    start_pos                   += full_elem_height;
                    let (left, right)            = (0, cached.dims.width - 1);
                    (elem, Rect {top, left, bottom, right})
                })
                .collect::<Vec<_>>();
            elem_positions.sort_by(|(e1, _), (e2, _)| ZOrder::cmp(&e1.z_order, &e2.z_order));
            for (elem, ref rect) in elem_positions.drain(..) {
                let GraphCalcOutput {
                    named_nodes : mut sub_named_nodes,
                    ..
                }   = elem.layout.calc_graph(rect, Some(&mut frame))?;
                for (sub_name, sub_node) in sub_named_nodes.drain() {
                    if named_nodes.contains_key(&sub_name) {
                        return Err(LayoutError::DuplicateName(sub_name));
                    }
                    else {
                        named_nodes.insert(sub_name, sub_node);
                    }
                }
            }
            //for &mut Element {constr, z_order, ref mut layout,} in  self.elems.borrow_mut().iter_mut() {
            //    let sub_dims                 = layout.calc_naive_bounds();
            //    let full_elem_height         =
            //        match constr {
            //            HeightConstraints::MinHeight                =>
            //                sub_dims.height,
            //            HeightConstraints::MinPixelHeight(pixels)   =>
            //                i16::max(pixels, sub_dims.height),
            //            HeightConstraints::VerticalParts(parts)     => {
            //                let height  = new_part * parts as f32 + accum_err;
            //                accum_err   = height.fract();
            //                height.floor() as i16
            //            },
            //            HeightConstraints::MaxHeight                => {
            //                let height  = new_max + accum_err;
            //                accum_err   = height.fract();
            //                height.floor() as i16
            //            },
            //        };
            //    let (top, bottom)            = (start_pos, start_pos + full_elem_height - 1);
            //    start_pos                   += full_elem_height;
            //    let (left, right)            = (0, cached.dims.width - 1);
            //    let GraphCalcOutput {
            //        named_nodes : mut sub_named_nodes,
            //        graph       : mut sub_graph,
            //    }                            = layout.calc_graph(&Rect {top, left, bottom, right,})?;
            //    if let &Some(ref name) = layout.name() {
            //        if named_nodes.contains_key(name) {
            //            return Err(LayoutError::DuplicateName(name.clone()));
            //        }
            //        else {
            //            named_nodes.insert(name.clone(), graph.clone());
            //        }
            //    }
            //    for (sub_name, sub_node) in sub_named_nodes.drain() {
            //        if named_nodes.contains_key(&sub_name) {
            //            return Err(LayoutError::DuplicateName(sub_name));
            //        }
            //        else {
            //            named_nodes.insert(sub_name, sub_node);
            //        }
            //    }
            //    node_vec.push(
            //        (   z_order
            //        ,   graph
            //        )
            //    );
            //}
            //node_vec.sort_by(|(z1, _), (z2, _)| ZOrder::cmp(z1, z2));
            //for (_, node) in node_vec.drain(..) {
            //    frame.append_child(node)
            //        .map_err(|err| LayoutError::SceneGraphConstructionFailure(err))?;
            //}
            Ok(GraphCalcOutput {
                graph,
                named_nodes,
            })
            //}
        }
    }
}


/// Declare a simpler `Row` type based one `LineArrangement`
pub type Row<Name> = LineArrangement<Name, WidthConstraints>;

/// Declare a simpler `Col` type based one `LineArrangement`
pub type Column<Name> = LineArrangement<Name, HeightConstraints>;
