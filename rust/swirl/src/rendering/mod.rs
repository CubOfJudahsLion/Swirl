//! Module  :   Rendering
//! Purpose :   Renderer and related data structures.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Mon 03/22/2021 01:03 PM
//! Modified:   Fri 04/01/2022 08:39 PM


use std::cell::{Cell, RefCell};
use std::fmt::{self, Formatter, Debug};
use std::default::Default;
use std::ops::{Neg, Add, Mul};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

use crate::math::numeric::Floating;
use crate::base::drawable::*;
use crate::math::geom::transform::*;
use crate::math::geom::rect::RectBounds;
use crate::scene_graph::*;


/// Instructions on how to clean up the canvas 
/// before every rendering cycle. One can fill it
/// with a color, clear it, or leave it as-is.
#[derive(Debug)]
pub enum BeforeRendering {
    Fill(JsValue),
    Clear,
    Nothing,
}


//  Data for  auto-rendering, including no auto-rendering
//  at all (`NoAuto`). `Fixed` is an unchanging framerate
//  (rendering and timing are given by `setInterval`, while
//  `Adaptive` renders with `requestAnimationFrame` though
//  also providing a timer.
#[derive(Clone)]
enum AutoRendering {
    NoAuto,
    Fixed(
        i32,
        Rc<Cell<Option<Closure<dyn FnMut()>>>>
    ),
    Adaptive(
        Rc<Cell<Option<i32>>>,
        Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>
    ),
}

impl ToString for AutoRendering {
    fn to_string(&self) -> String {
        use AutoRendering::*;
        match self {
            NoAuto                              =>
                "AutoRendering::NoAuto".to_string(),
            Fixed(interval, _)                  =>
                format!("AutoRendering::Fixed(interval#: {}, <Closure>)", *interval),
            Adaptive(animation_frame_id, _)     =>
                format!(
                    "AutoRendering::Adaptive(animation_frame#: {}, <Closure>)",
                    animation_frame_id
                        .get()
                        .map(|n| format!("Some({})", n))
                        .unwrap_or("None".to_string())
                ),
        }
    }
}

impl Debug for AutoRendering {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.to_string())
    }
}


/// A descriptor for the type of auto-rendering.
#[derive(Clone, Copy, Debug)]
pub enum AutoRenderingType {
    NoAuto,
    Fixed,
    Adaptive,
}

impl From<&AutoRendering> for AutoRenderingType {
    fn from(ar: &AutoRendering) -> Self {
        match ar {
            &AutoRendering::NoAuto      => AutoRenderingType::NoAuto,
            &AutoRendering::Fixed(_, _) => AutoRenderingType::Fixed,
            _                           => AutoRenderingType::Adaptive,
        }
    }
}


#[derive(Clone)]
pub struct Renderer {
    context     : CanvasRenderingContext2d,
    auto        : AutoRendering,
}

impl Renderer {
    // Utility function. Gets us a context out of a canvas.
    fn get_2d_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, JsValue> {
        canvas.get_context("2d")?
            .ok_or(JsValue::from_str("Cannot acquire drawing context."))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("Object is not a CanvasRenderingContext2d."))
    }

    #[allow(dead_code)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Renderer, JsValue> {
        let context     = Self::get_2d_context(canvas)?;
        Ok(Renderer {context, auto: AutoRendering::NoAuto})
    }

    #[inline(always)]
    pub fn render<N>(&self, node: &SceneGraphNode<N>, before: &BeforeRendering) -> Result<(), JsValue>
        where   N   :   Default
                    +   From<i16>
                    +   PartialEq
                    +   Add<Output=N> + Mul<Output=N> + Neg<Output=N>
                    +   Floating
                    +   RectBounds
                    +   Copy,
                f64 :   From<N>
    {
        Self::render_worker(&self.context, None, node, before)?;
        Ok(())
    }

    fn render_worker<N>(
        context     : &CanvasRenderingContext2d,        //  Drawing context
        world_opt   : Option<(bool, &Transform<N>)>,    //  World tranform from ancestors, not applied
        node        : &SceneGraphNode<N>,               //  The node to render
        before      : &BeforeRendering,                 //  What to do before rendering another frame
    ) -> Result<(), JsValue>
        where   N   :   Default
                    +   From<i16>
                    +   PartialEq
                    +   Add<Output=N> + Mul<Output=N> + Neg<Output=N>
                    +   Floating
                    +   RectBounds
                    +   Copy,
                f64 :   From<N>
    {
        use SceneGraphNode::*;
        let update_res = node.update_world_transform_from_parents(world_opt);
        match before {
            &BeforeRendering::Clear         => {
                context.reset_transform()?;
                let canvas = context
                    .canvas()
                    .ok_or_else(|| JsValue::from_str(
                        "Renderer couldn't seize canvas data for clearing before rendering."
                    ))?;
                context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
            },
            &BeforeRendering::Fill(ref val) => {
                context.reset_transform()?;
                let canvas = context
                    .canvas()
                    .ok_or_else(|| JsValue::from_str(
                        "Renderer couldn't seize canvas data for filling before rendering."
                    ))?;
                context.set_fill_style(val);
                context.fill_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
            },
            _                               => {
                ()
            },
        }
        match node {
            &Leaf(ref leaf)     => {
                //  Draw only if flagged `drawable()` right now.
                if leaf.0.borrow().drawable.drawable() {
                    context.set_global_alpha(leaf.0.borrow().drawable.alpha());
                    context.set_global_composite_operation(
                        <&str as From<BlendMode>>::from(
                            leaf.0.borrow().drawable.blend_mode()
                        )
                    )?;
                    //context.reset_transform()?;
                    leaf.world_transform().apply(context)?;
                    leaf.0.borrow().drawable.draw(context)?;
                }
            },
            &Frame(ref frame)   => {
                //  Draw all children, passing the world transform.
                for child in frame.0.borrow().children.iter() {
                    Self::render_worker(context, update_res, child, &BeforeRendering::Nothing)?;
                }
            },
        }
        Ok(())
    }

    #[inline(always)]
    pub fn auto_rendering(&self) -> AutoRenderingType {
        AutoRenderingType::from(&self.auto)
    }

    pub fn clear_auto_rendering(&mut self) -> Result<(), JsValue> {
        let wnd = window()
            .ok_or(JsValue::from_str(&format!(
                "{}::Renderer::clear_auto_rendering:: no window.",
                module_path!()
            )))?;
        use AutoRendering::*;
        match &mut self.auto {
            NoAuto                  => {
                Ok(())
            },
            Fixed(intv, mem)        => {
                wnd.clear_interval_with_handle(*intv);
                mem.set(None);
                self.auto = NoAuto;
                Ok(())
            },
            Adaptive(fr_id, mem)    => {
                if let Some(fr_id) = fr_id.get() {
                    wnd.cancel_animation_frame(fr_id)?;
                }
                mem.replace(None);
                self.auto = NoAuto;
                Ok(())
            },
        }
    }

    /// Starts automatic rendering, either synchronous (i.e., every x milliseconds)
    /// or asynchronous (as soon as the last frame is rendered, render the next.)
    /// ### Arguments:
    /// - *millis*: either `Some()` amount of milliseconds (fixed framerate), or
    ///   `None` (asynchronous.)
    /// - *node*: what to render
    /// - *before*: what to do with the screen before rendering starts
    /// ### Returns:
    /// A `Result` with a `JsValue` error, if setting autorendering failed.
    pub fn set_auto_rendering<N>(
        &mut self,
        millis  : Option<i32>,
        node    : &SceneGraphNode<N>,
        before  : BeforeRendering
    )
    -> Result<(), JsValue>
        where   N   :   Default
                    +   From<i16>
                    +   PartialEq
                    +   Add<Output=N> + Mul<Output=N> + Neg<Output=N>
                    +   Floating
                    +   RectBounds
                    +   Copy
                    +   'static,
                f64 :   From<N>
    {
        self.clear_auto_rendering()?;
        let wnd = window()
            .ok_or(JsValue::from_str(&format!(
                "{}::Renderer::set_auto_rendering: no window.",
                module_path!()
            )))?;
        match millis {
            Some(millis)    => {
                let rend_mem    : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                                = Rc::new(Cell::new(None));
                let rend_fn     = {
                    let context     = self.context.clone();
                    let node        = node.clone();
                    Closure::wrap(Box::new(move || {
                        Self::render_worker(&context, None, &node, &before).unwrap_throw();
                    }) as Box<dyn FnMut()>)
                };
                let interval_id = wnd.set_interval_with_callback_and_timeout_and_arguments_0(
                    rend_fn.as_ref().unchecked_ref(),
                    millis
                )?;
                rend_mem.set(Some(rend_fn));
                self.auto = AutoRendering::Fixed(interval_id, rend_mem);
                Ok(())
            },
            None            => {
                let rend_mem        : Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>
                                    = Rc::new(RefCell::new(None));
                let frame_mem       : Rc<Cell<Option<i32>>>
                                    = Rc::new(Cell::new(None));
                let rend_fn         = {
                    let wnd         = wnd.clone();
                    let context     = self.context.clone();
                    let node        = node.clone();
                    let frame_mem   = frame_mem.clone();
                    let rend_mem    = rend_mem.clone();
                    Closure::wrap(Box::new(move |_js_time: JsValue| {
                        frame_mem.set(None);
                        Self::render_worker(&context, None, &node, &before).unwrap_throw();
                        rend_mem
                            .borrow()
                            .as_ref()
                            .map(|rend_fn| frame_mem.set(Some(
                                wnd.request_animation_frame(
                                    rend_fn.as_ref().unchecked_ref()
                                )
                                .unwrap_throw()
                            )));
                    }) as Box<dyn FnMut(JsValue)>)
                };
                let frame           = wnd.request_animation_frame(rend_fn.as_ref().unchecked_ref())?;
                frame_mem.set(Some(frame));
                rend_mem.replace(Some(rend_fn));
                self.auto = AutoRendering::Adaptive(frame_mem, rend_mem);
                Ok(())
            }
        }
    }
}
