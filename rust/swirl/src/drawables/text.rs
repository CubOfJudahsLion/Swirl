//! Module  :   Text
//! Purpose :   A sufficiently configurable,modifiable text element.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 04/27/2021 08:15 AM
//! Modified:   Mon 06/06/2022 02:22 PM


use std::ops::DerefMut;
use std::string::ToString;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

use crate::math::geom::rect::Rect;
use crate::base::bounded::Bounded;
use crate::base::drawable::*;
use crate::style::fonts::FontStyle;
use crate::utils::shared::Shared;
use crate::utils::html::*;


//  The internal representation of the text string along
//  with its style.
#[derive(Clone, Debug)]
struct TextInner {
    text        : String,
    style       : FontStyle,
    vis         : bool,
    alpha       : f64,
    blend_mode  : BlendMode,
    context     : CanvasRenderingContext2d,
    bounds      : Rect<i16>,
}

/// A text string with a defined style.
#[derive(Clone, Debug)]
pub struct Text(Shared<TextInner>);

impl Text {
    //  Updates internal text metrics.
    fn calculate_bounds(ctx: &CanvasRenderingContext2d, style: &FontStyle, text: &str) -> Result<Rect<i16>, JsValue> {
        style.apply(ctx)?;
        let metrics = ctx.measure_text(text)?;
        Ok(Rect {
            top     : -metrics.actual_bounding_box_ascent().ceil()  as i16,
            left    : -metrics.actual_bounding_box_left().ceil()    as i16,
            bottom  :  metrics.actual_bounding_box_descent().ceil() as i16,
            right   :  metrics.actual_bounding_box_right().ceil()   as i16,
        })
    }

    /// Creates a new text string with a style specification. The style includes font, size. color,
    /// stroke, etc. The canvas is required for 
    pub fn new(text: &str, style: FontStyle) -> Result<Self, JsValue> {
        let common_canvas   = get_common_canvas()?;
        let context         = common_canvas.get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Couldn't allocate resource"))?
            .unchecked_into::<CanvasRenderingContext2d>();
        let bounds          = Self::calculate_bounds(&context, &style, text)?;
        Ok(Text(Shared::new(TextInner {
            text        : text.to_string(),
            style,
            vis         : true,
            alpha       : 1.,
            blend_mode  : BlendMode::Normal,
            context,
            bounds,
        })))
    }

    pub fn text(&self) -> &str {
        &self.0.text
    }

    pub fn set_text(&mut self, new_text: String) -> Result<(), JsValue> {
        let this        = self.0.deref_mut();
        this.bounds     = Self::calculate_bounds(&this.context, &this.style, &new_text)?;
        this.text       = new_text;
        Ok(())
    }

    pub fn style(&self) -> &FontStyle {
        &self.0.style
    }

    pub fn modify_style<F>(&mut self, modifier: F) -> Result<(), JsValue>
        where F: FnOnce(&mut FontStyle) -> Result<(), JsValue>
    {
        modifier(&mut self.0.style)?;
        self.0.bounds = Self::calculate_bounds(&self.0.context, &self.0.style, &self.0.text)?;
        Ok(())
    }
}

impl Bounded<i16> for Text {
    fn bounds(&self) -> Option<Rect<i16>> {
        Some(self.0.bounds.clone())
    }
}

impl Drawable for Text {
    #[inline]
    fn drawable(&self) -> bool {
        self.0.vis
    }

    fn set_drawable(&mut self, drawable: bool) {
        self.0.vis = drawable;
    }

    fn alpha(&self) -> f64 {
        self.0.alpha
    }

    fn set_alpha(&mut self, alpha: f64) {
        self.0.alpha = alpha;
    }

    fn blend_mode(&self) -> BlendMode {
        self.0.blend_mode
    }

    fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.0.blend_mode = blend_mode;
    }

    fn draw(
        &self,
        ctx: &CanvasRenderingContext2d
    ) -> Result<(), JsValue> {
        self.0.style.apply(ctx)?;
        if self.0.style.fill().is_some() {
            ctx.fill_text(&self.0.text, 0., 0.)?;
        }
        if self.0.style.stroke().is_some() {
            ctx.stroke_text(&self.0.text, 0., 0.)?;
        }
        Ok(())
    }
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    //use web_sys::console;
    use super::*;
    use crate::style::fonts::*;
    use crate::style::shorten::*;

    #[wasm_bindgen_test]
    fn getters_and_setters() {
        let mut t = Text::new(
            "Test Text",
            FontStyleBuilder::new()
                .with_family(Cursive)
                .with_size(XXLarge)
                .build()
        ).unwrap();
        t.set_alpha(0.25);
        assert_eq!(t.alpha(), 0.25);
        t.set_blend_mode(BlendMode::Erase);
        assert_eq!(t.blend_mode(), BlendMode::Erase);
        t.set_drawable(false);
        assert_eq!(t.drawable(), false);
        let another = "Another test text.";
        assert!(t.set_text(another.to_string()).is_ok());
        assert_eq!(t.text(), another);
        let bounds1 = t.bounds().unwrap();
        //console_log!("style before: {}", &*t.style().style_string());
        //console_log!("context style before modification: \"{}\"", t.0.context.font());
        assert!(t.modify_style(|style| {
            style.set_family(NamedFamily("Times New Roman".to_string()));
            style.set_size(Medium);
            Ok(())
        }).is_ok());
        assert!(!t.style().cache_incomplete());
        assert_eq!(t.style().family(), &NamedFamily("Times New Roman".to_string()));
        assert_eq!(t.style().size(), &Medium);
        //console_log!("style after: {}", &*t.style().style_string());
        //console_log!("context style after modification: \"{}\"", t.0.context.font());
        assert_ne!(t.bounds().unwrap(), bounds1);
    }

    // #[wasm_bindgen_test]
    // fn actually_drawn() {
    //     use crate::rendering::Renderer;
    //     use crate::scene_graph::leaf::SceneGraphLeaf;
    //     use crate::base::transformable::Transformable;
    //     use crate::style::color::*;
    //     let cnv = get_common_canvas().unwrap();
    //     assert!(cnv.style().set_property("visibility", "visible").is_ok());
    //     let txt = Text::new(
    //         "Text Actually<br/>Drawn",
    //         FontStyleBuilder::new()
    //             .with_size(FontSize::XXLarge)
    //             .with_fill(Some(Color::Named(NamedColor::Yellow)))
    //             .with_stroke(Some((1, Color::Named(NamedColor::Red))))
    //             .with_family(FontFamily::Cursive)
    //             .build()
    //     ).unwrap();
    //     let txt_bounds = txt.bounds().unwrap();
    //     if cnv.width() < txt_bounds.width() as u32 {
    //         cnv.set_width(txt_bounds.width() as u32);
    //     }
    //     if cnv.height() < txt_bounds.height() as u32 {
    //         cnv.set_height(txt_bounds.height() as u32);
    //     }
    //     let mut leaf = SceneGraphLeaf::<f32>::new_node(&txt);
    //     console_log!("bounds: {}", txt_bounds);
    //     console_log!("bounds width: {}, height: {}", txt_bounds.width(), txt_bounds.height());
    //     leaf.set_x(-txt_bounds.left);
    //     leaf.set_y(-txt_bounds.top);
    //     let renderer = Renderer::new(&cnv).unwrap();
    //     assert!(renderer.render(&leaf).is_ok());
    // }
}
