//! Simple test for Affine Transformations.
//! Nested just a couple of levels deep.
//! Author  : Alex Feterman-Naranjo
//! Created : Tue 09/28/2021 07:21 PM


use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, throw_str};
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use swirl::*;


fn find_canvas_and_context() -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    //
    fn find_canvas() -> Result<HtmlCanvasElement, JsValue> {
        window()
            .ok_or_else(|| JsValue::from_str("cannot find main window"))?
            .document()
            .ok_or_else(|| JsValue::from_str("cannot find document element"))?
            .query_selector("canvas#swirl-render")?
            .ok_or_else(|| JsValue::from_str("cannot find canvas element"))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str(
                r#""canvas#swirl-render" somehow fails to read as a canvas"#
            ))
    }
    //
    fn extract_context(cnv: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, JsValue> {
        cnv
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("cannot extract 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("cannot contextualize as rendering context"))
    }
    //
    let cnv = find_canvas()?;
    let ctx = extract_context(&cnv)?;
    Ok((cnv, ctx))
}


#[derive(Clone, Copy, PartialEq, Eq)]
enum RedOrBlue {
    Red,
    Blue,
}

fn rect_color(rb: RedOrBlue, inside: bool) -> &'static str {
    use RedOrBlue::*;
    match (rb, inside) {
        (Red,  false)   => "#FF0000",
        (Red,  true )   => "#FF6060",
        (Blue, false)   => "#0000FF",
        (Blue, true )   => "#6060FF",
    }
}


fn run_demo(background_pattern: HtmlImageElement) -> Result<(), JsValue> {
    use std::f32::consts::PI;
    //
    const DRAWN_RECT_WIDTH  : i16 = 300;
    const DRAWN_RECT_HEIGHT : i16 = 150;
    const MARGIN            : i16 = 100;
    //
    #[derive(Debug)]
    struct DrawnRectInternal {drawable: bool, alpha: f64, blend_mode: BlendMode,}
    #[derive(Debug, Clone)]
    struct DrawnRect(Shared<DrawnRectInternal>);
    impl DrawnRect {
        fn new() -> Self {
            DrawnRect(Shared::new(DrawnRectInternal {
                drawable    : true,
                alpha       : 1.,
                blend_mode  : BlendMode::Normal,
            }))
        }
    }
    impl Bounded<i16> for DrawnRect {
        fn bounds(&self) -> Option<Rect<i16>> { Some(Rect::new(0, 0, DRAWN_RECT_HEIGHT, DRAWN_RECT_WIDTH)) }
    }
    impl Drawable for DrawnRect {
        fn drawable(&self) -> bool { self.0.drawable }
        fn set_drawable(&mut self, drawable_p: bool) { self.0.drawable = drawable_p; }
        fn alpha(&self) -> f64 { self.0.alpha }
        fn set_alpha(&mut self, alpha_p: f64) { self.0.alpha = alpha_p; }
        fn blend_mode(&self) -> BlendMode { self.0.blend_mode }
        fn set_blend_mode(&mut self, blend_mode_p: BlendMode) { self.0.blend_mode = blend_mode_p; }
        fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
            ctx.stroke_rect(0., 0., DRAWN_RECT_WIDTH as f64, DRAWN_RECT_HEIGHT as f64);
            ctx.fill_rect(0., 0., DRAWN_RECT_WIDTH as f64, DRAWN_RECT_HEIGHT as f64);
            Ok(())
        }
    }
    //
    //  Create the assets
    let wnd             = window()
        .ok_or_else(|| JsValue::from_str("couldn't find main window"))?;
    let (cnv, ctx)      = find_canvas_and_context()?;
    let back_pattern    = ctx.create_pattern_with_html_image_element(
        &background_pattern,
        "repeat"
    )?
        .ok_or(JsValue::from_str("cannot extract background image"))?;
    let mut drawn_rect  = DrawnRect::new();
    let mut leaf        = SceneGraphLeaf::new(&drawn_rect);
    leaf.set_pivot(DRAWN_RECT_WIDTH >> 1, DRAWN_RECT_HEIGHT >> 1);
    let leaf_node       = SceneGraphNode::<f32>::Leaf(leaf.clone());
    let mut frame       = SceneGraphFrame::new();
    let margin          = (std::cmp::max(DRAWN_RECT_WIDTH, DRAWN_RECT_HEIGHT) >> 1) + MARGIN;
    frame.set_x_and_y(margin, margin);
    let frame_node      = SceneGraphNode::<f32>::Frame(frame.clone());
    //  Group them
    frame.append_node(leaf_node).map_err(|_| JsValue::from_str("cannot insert leaf node"))?;
    //  Set up the event system
    let rb_mouse        = Rc::new(RefCell::new(RedOrBlue::Red));
    let inside_mouse    = Rc::new(RefCell::new(false));
    let rb_anim         = rb_mouse.clone();
    let inside_anim     = inside_mouse.clone(); // These two are copies for the other handle below.
    let mut driver      = PointerEventDriver::<f32>::new(cnv.clone())?;
    let mut evs         = driver.add_emitter(&leaf);
    let mouse           = move |ev: &PointerEvent<f32>| {
        //console::log_1(&JsValue::from_str(&format!("Mouse event fired: {}", ev.type_())));
        use PointerEventType::*;
        use RedOrBlue::*;
        match ev.type_() {
            Enter   =>  {
                *inside_mouse.borrow_mut() = true;
            },
            Leave   =>  {
                *inside_mouse.borrow_mut() = false;
            },
            Click   =>  {
                let next_color =
                    if *rb_mouse.borrow() == Blue {
                        Red
                    }
                    else {
                        Blue
                    };
                *rb_mouse.borrow_mut() = next_color;
            },
            _       =>  (),
        }
    };
    evs.subscribe_multiple({use PointerEventType::*; &[Leave, Enter, Click]}, mouse);
    //  Now set up rendering.
    ctx.set_line_width(1.);
    let renderer        = Renderer::new(&cnv)?;
    let mut angle_1     = 0_f32;
    let mut angle_2     = 0_f32;
    let anim            = Closure::wrap(Box::new(
        move || {
            let _   = ctx.reset_transform();
            let _   = ctx.set_global_composite_operation("source-over");
            //ctx.clear_rect(0., 0., 800., 600.);
            ctx.set_fill_style(back_pattern.as_ref());
            ctx.fill_rect(0., 0., 800., 600.);
            leaf.set_x_skew(angle_1.sin() / 2.);
            angle_1 = (angle_1 + PI / 50.) % (6. * PI);
            if angle_1 <= 2. * PI {
                drawn_rect.set_blend_mode(BlendMode::Normal);
                drawn_rect.set_alpha(0.5);
            }
            else if angle_1 <= 4. * PI {
                drawn_rect.set_blend_mode(BlendMode::Add);
                drawn_rect.set_alpha(1.);
            }
            else {
                drawn_rect.set_blend_mode(BlendMode::Multiply);
                drawn_rect.set_alpha(1.);
            }
            frame.set_rotation(angle_2.sin() / 4.);
            angle_2 = (angle_2 + PI / 50. / 5.) % (2. * PI);
            ctx.set_stroke_style(&JsValue::from_str("#663"));
            ctx.set_fill_style(&JsValue::from_str(rect_color(*rb_anim.borrow(), *inside_anim.borrow())));
            let _   = renderer.render(&frame_node);
            if let Some(wb) = frame_node.world_bounds() {
                let _   = ctx.reset_transform();
                ctx.set_global_alpha(1.);
                let _   = ctx.set_global_composite_operation("source-over");
                ctx.set_stroke_style(&JsValue::from_str("#244"));
                ctx.stroke_rect(
                    wb.left as f64,
                    wb.top as f64,
                    wb.width() as f64,
                    wb.height() as f64
                );
            }
        }
    ) as Box<dyn FnMut()>);
    wnd.set_interval_with_callback_and_timeout_and_arguments_0(
        anim.as_ref().unchecked_ref(),
        20
    )?;
    anim.forget();
    Ok(())
}


fn load() -> Result<(), JsValue> {
    let mut b = BasicAssetLoader::<u8>::new();
    b.add_image_request(0, "img/ugly_pattern.jpg".to_string())?;
    b.add_loaded_listener(|res_map| {
        if let Some(Loaded::Image(ugly_pattern)) = res_map.get(&0) {
            if let Err(err) = run_demo(ugly_pattern.clone()) {
                throw_str(&format!(
                    "Failed to run demo: {}.",
                    err.as_string().unwrap_or_else(|| "runtime error".to_string())
                ))
            }
        }
        else {
            throw_str("Failed to run demo: couldn't find background image.");
        }
    })?;
    b.load_all()
}


#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&JsValue::from_str("SWIRL ©2020-21 Multislot Inc."));
    if let Err(err) = load() {
        throw_str(&format!(
            "Failed to run demo: {}.",
            err.as_string().unwrap_or_else(|| "runtime error".to_string())
        ))
    }
}
