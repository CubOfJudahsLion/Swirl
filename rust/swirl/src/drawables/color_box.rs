//! # Module        :   _Color Box_
//! - **Purpose**   :   A colored rectangle
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Mon 06/13/2022 09:37 PM
//! - **Modified**  :   ~


use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::base::drawable::*;
use crate::base::bounded::Bounded;
use crate::math::geom::dimensions::Dimensions;
use crate::math::geom::rect::Rect;
use crate::style::color::*;


#[derive(Clone, Debug)]
pub struct ColorBox(Dimensions<i16>, JsValue);

impl ColorBox {
    #[inline(always)]
    pub fn new(width: i16, height: i16, color: Color) -> Self {
        ColorBox(Dimensions {width, height}, JsValue::from_str(&color.to_string()))
    }

    #[inline(always)]
    pub fn new_from_dims(dims: &Dimensions<i16>, color: Color) -> Self {
        ColorBox(dims.clone(), JsValue::from_str(&color.to_string()))
    }
}

impl Bounded<i16> for ColorBox {
    fn bounds(&self) -> Option<Rect<i16>> { Some(Rect::new_by_dim(0, 0, self.0.height, self.0.width)) }
}

impl Drawable for ColorBox {
    #[inline(always)]
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.set_fill_style(&self.1);
        ctx.fill_rect(0., 0., self.0.width as f64, self.0.height as f64);
        Ok(())
    }
}
