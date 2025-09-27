//! Module  :   Sprite
//! Purpose :   A light image directly loaded from the corresponding HTML element.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   2021-02-22 14:51
//! Modified:   Fri 20 May 2022 02:31:48 PM CST


use crate::utils::Shared;
use crate::base::{Bounded, drawable::*};
use crate::math::geom::rect::*;

use wasm_bindgen::JsValue;
use web_sys::{HtmlImageElement, CanvasRenderingContext2d};


#[derive(Clone, Debug)]
struct SpriteInner {
    img     : HtmlImageElement,
    vis     : bool,
    alpha   : f64,
    mode    : BlendMode,
}

#[derive(Clone, Debug)]
pub struct Sprite(Shared<SpriteInner>);

impl Sprite {
    pub fn new(img: HtmlImageElement) -> Sprite {
        Sprite(Shared::new(SpriteInner {
            img,
            vis     : true,
            alpha   : 1.,
            mode    : BlendMode::Normal,
        }))
    }
}

impl Bounded<i16> for Sprite {
    #[inline(always)]
    fn bounds(&self) -> Option<Rect<i16>> {
        Some(Rect::new_by_dim(0, 0, self.0.img.height() as i16 - 1, self.0.img.width() as i16 - 1))
    }
}

impl Drawable for Sprite {
    fn drawable(&self) -> bool { self.0.vis }

    fn set_drawable(&mut self, drawable: bool) { self.0.vis = drawable; }

    fn alpha(&self) -> f64 { self.0.alpha }

    fn set_alpha(&mut self, alpha_p: f64) {
        self.0.alpha =
            if alpha_p < 0. {
                0.
            }
            else if alpha_p > 1. {
                1.
            }
            else {
                alpha_p
            };
    }

    fn blend_mode(&self) -> BlendMode { self.0.mode }

    fn set_blend_mode(&mut self, blend_mode: BlendMode) { self.0.mode = blend_mode; }

    fn draw(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if self.0.vis {
            context.draw_image_with_html_image_element_and_dw_and_dh(
                &self.0.img,
                0.0,
                0.0,
                self.0.img.width()  as f64,
                self.0.img.height() as f64
            )
        }
        else {
            Ok(())
        }
    }
}
