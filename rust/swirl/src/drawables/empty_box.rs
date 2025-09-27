//! # Module        :   _Empty Box_
//! - **Purpose**   :   A box that doesn't draw anything, but occupies an area.
//!                     Particularly useful for layouts and tests.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Wed 18 May 2022 09:28:37 AM CST
//! - **Modified**  :   Mon 06/06/2022 02:15 PM


use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::base::drawable::*;
use crate::base::bounded::Bounded;
use crate::math::geom::dimensions::Dimensions;
use crate::math::geom::rect::Rect;


#[derive(Clone, Debug)]
pub struct EmptyBox(Dimensions<i16>);

impl EmptyBox {
    #[inline(always)]
    pub fn new(width: i16, height: i16) -> Self {
        EmptyBox(Dimensions {width, height})
    }

    #[inline(always)]
    pub fn new_from_dims(dims: &Dimensions<i16>) -> Self {
        EmptyBox(dims.clone())
    }
}

impl Bounded<i16> for EmptyBox {
    fn bounds(&self) -> Option<Rect<i16>> { Some(Rect::new_by_dim(0, 0, self.0.height, self.0.width)) }
}

impl Drawable for EmptyBox {
    #[inline(always)]
    fn draw(&self, _ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> { Ok(()) }
}
