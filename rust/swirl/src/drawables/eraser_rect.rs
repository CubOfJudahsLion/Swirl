//! Module  :   EraserRect
//! Purpose :   A `Drawable` that simply erases part of the canvas.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Fri 21 Jan 2022 11:22:00 AM CST
//! Modified:   ~


use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::utils::Shared;
use crate::base::{Bounded, drawable::*};
use crate::math::geom::rect::*;


#[derive(Debug)]
struct EraserRectInner {
    area    : Rect<i16>,
    vis     : bool,
}


#[derive(Clone, Debug)]
pub struct EraserRect(Shared<EraserRectInner>);


impl EraserRect {
    pub fn new(r: &Rect<i16>) -> Self {
        EraserRect(Shared::new(EraserRectInner {
            area    : r.clone(),
            vis     : true,
        }))
    }
}


impl Bounded<i16> for EraserRect {
    #[inline(always)]
    fn bounds(&self) -> Option<Rect<i16>> {
        Some(self.0.area.clone())
    }
}


impl Drawable for EraserRect {
    fn drawable(&self) -> bool { self.0.vis }

    fn set_drawable(&mut self, drawable: bool) { self.0.vis = drawable; }

    /// Alpha is always 1.0, i.e., full erasing.
    fn alpha(&self) -> f64 { 1.0 }

    /// Can't be changed. The function does nothing.
    fn set_alpha(&mut self, _: f64) { }

    /// Always returns `Erase`.
    fn blend_mode(&self) -> BlendMode { BlendMode::Erase }

    /// Can't be changed. The function does nothing.
    fn set_blend_mode(&mut self, _: BlendMode) { }

    fn draw(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if self.0.vis {
            context.clear_rect(
                self.0.area.left     as f64,
                self.0.area.top      as f64,
                self.0.area.width()  as f64,
                self.0.area.height() as f64
            );
        }
        Ok(())
    }
}
