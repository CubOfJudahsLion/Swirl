//! Module  :   App
//! Purpose :   Application component.
//!             Eases setting up a SWIRL application by
//!             grouping several components and simplifying
//!             resource and asset management.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu 10/14/2021 02:48 PM
//! Modified:   Mon 10/18/2021 02:53 PM


use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsCast};
use js_sys::{Function, Promise};
use web_sys::{
    CanvasRenderingContext2d,
    Document,
    HtmlCanvasElement,
    HtmlElement,
    HtmlHeadElement,
    Window
};

use crate::math::numeric::{TryAsFrom, Floating};
use crate::math::geom::rect::RectBounds;
use crate::utils::html::*;
use crate::utils::enum_set::*;
use crate::loader::*;
use crate::scene_graph::frame::SceneGraphFrame;
use crate::scene_graph::node::SceneGraphNode;
use crate::rendering::Renderer;
use crate::events::interactive::pointer::driver::PointerEventDriver;


//  These five structures serve to denote parts of a transformation
//  for a `Transformable`. 
struct P(i16, i16);
struct K<N>(N, N);
struct S<N>(N, N);
struct R<N>(N);
struct T(i16, i16);

pub struct InitialTransform<N>(P, K<N>, S<N>, R<N>, T);

pub enum SceneConstruction<D> {
    Frame(Option<String>, Box<SceneConstruction<D>>),
    Leaf(Option<String>, D),
}


/// An application possessing the inner resources
/// (and logic) to easily start a game or bonus.
#[derive(Clone)]
pub struct App<ResourceKey: Hash + Eq, N: Floating + 'static> {
    //  Pre-loaded HTML structure.
    wnd             : Window,
    doc             : Document,
    head            : HtmlHeadElement,
    body            : HtmlElement,
    //  Own canvas and context.
    canvas          : HtmlCanvasElement,
    context         : CanvasRenderingContext2d,
    //  Resource set.
    resources       : Rc<RefCell<HashMap<ResourceKey, Loaded>>>,
    //  Scene structure and representation.
    scene_graph     : SceneGraphNode<N>,
    renderer        : Renderer,
    //  Events system base.
    event_driver    : PointerEventDriver<N>,
}


impl<K: EnumSet + Display, N: Floating + 'static> App<K, N>
    where   K:  EnumSet + Display, N: Floating + 'static,
            N:  Floating + RectBounds + From<i16> + TryAsFrom<u32> + TryAsFrom<i32> + TryAsFrom<f64>
{
    fn load<F>(target_dict: Rc<RefCell<HashMap<K, Loaded>>>, progress_callback: F) -> Result<Promise, JsValue>
        where   F:  FnMut(f64) + 'static,
    {
        let mut loader  = BasicAssetLoader::<K>::new();
        let load_errs   = K::ORDERED_VARIANTS
            .iter()
            .map(|&variant| -> Result<(), JsValue> {
                let path        = variant.to_string();
                let lower_path  = path.to_lowercase();
                if [".png", ".jpg", ".jpeg", ".svg"].iter().any(|ext| lower_path.ends_with(ext)) {
                    loader.add_image_request(variant, path)?;
                    Ok(())
                }
                else if lower_path.ends_with(".woff") {
                    loader.add_font_request(variant, path)?;
                    Ok(())
                }
                else {
                    Err(JsValue::from_str(&path))
                }
            })
            .filter_map(|result|
                match result {
                    Err(err)    => Some(err),
                    _           => None,
                }
            )
            .collect::<Vec<_>>();
        if load_errs.len() > 0 {
            Err(JsValue::from_str(&format!(
                "Loading error: extension not handled for file(s) {}.",
                load_errs
                    .iter()
                    .map(|jserr| jserr.as_string().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ")
            )))
        }
        else {
            loader.add_progress_listener(progress_callback)?;
            Ok(Promise::new(&mut move |resolve: Function, reject: Function| {
                let dict = target_dict.clone();
                if loader.add_loaded_listener(move |res_map| {
                    if K::ORDERED_VARIANTS.iter().all(|variant| res_map.get(variant).is_some()) {
                        dict.deref().replace(res_map.clone());
                        if resolve.call0(&JsValue::NULL).is_err() {
                            throw_str("Cannot resolve loader promise.");
                        }
                    }
                    else {
                        if reject.call0(&JsValue::NULL).is_err() {
                            throw_str("Cannot reject loader promise.");
                        }
                    }
                }).is_err() {
                    throw_str("Failed to add loader listener.");
                }
                if loader.load_all().is_err() {
                    throw_str("Failed to start asset loading.");
                }
            }))
        }
    }

    fn extract_context(cnv: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, JsValue> {
        cnv
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Cannot extract 2d Context"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("cannot contextualize as rendering context"))
    }

    pub fn new<F>(canvas_name: &str, ) -> Result<(Self, Promise), JsValue>
        where F: FnMut(f64) + 'static,
    {
        let wnd             = get_window()?;
        let doc             = wnd
            .document()
            .ok_or_else(|| JsValue::from_str("Couldn't find Document element."))?;
        let body            = doc
            .body()
            .ok_or_else(|| JsValue::from_str("Couldn't find document's BODY."))?;
        let head            = doc
            .head()
            .ok_or_else(|| JsValue::from_str("Couldn't find HEAD element."))?;
        let canvas          = get_canvas(canvas_name)?;
        let context         = Self::extract_context(&canvas)?;
        let resources       = Rc::new(RefCell::new(HashMap::<K, Loaded>::new()));
        let scene_graph     = SceneGraphFrame::<N>::new_node();
        let renderer        = Renderer::new(&canvas)?;
        let event_driver    = PointerEventDriver::<N>::new(canvas.clone())?;
        let load_promise    = {
            let canvas  = canvas.clone();
            let context = context.clone();
            context.set_font("small-caps 48pt monospace");
            context.set_fill_style(&JsValue::from_str("black"));
            context.set_line_width(1.5);
            context.set_stroke_style(&JsValue::from_str("white"));
            context.set_text_align("center");
            Self::load(resources.clone(), move |percentage| {
                let loading_msg = format!("Loading: {:>5.1}%", percentage);
                context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
                let half_x = (canvas.width()  >> 1) as f64;
                let half_y = (canvas.height() >> 1) as f64;
                let _ = context.fill_text(&loading_msg, half_x, half_y);
                let _ = context.stroke_text(&loading_msg, half_x, half_y);
            })?
        };
        Ok((    App {
                    wnd,
                    doc,
                    body,
                    head,
                    canvas,
                    context,
                    resources,
                    scene_graph,
                    renderer,
                    event_driver,
                }
        ,       load_promise
        ))
    }
}
