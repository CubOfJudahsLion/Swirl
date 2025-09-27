// Last modified: Mon 10/18/2021 10:55 AM

use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_val, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, Window};
use swirl::*;


fn err_handler<T>(result: Result<T, JsValue>) -> T {
    match result {
        Err(js_val) => throw_val(js_val),
        Ok(js_val)  => js_val,
    }
}


fn find_window_and_context() -> Result<(Window, HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    let wnd = window()
        .ok_or_else(|| JsValue::from_str("Couldn't find main Window."))?;
    let cnv = wnd
        .document()
        .ok_or_else(|| JsValue::from_str("Couldn't find Document element."))?
        .query_selector("canvas#demo-canvas")?
        .ok_or_else(|| JsValue::from_str("Couldn't find canvas."))?
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = cnv
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("Couldn't unwrap result of context request"))?
        .dyn_into::<CanvasRenderingContext2d>()?;
    Ok((wnd, cnv, ctx))
}


fn prepare_centered_message_style(ctx: &CanvasRenderingContext2d, erase: bool) {
    if erase {
        ctx.clear_rect(150., 152., 650., 372.);
    }
    ctx.set_font("small-caps 600 expanded 42pt monospace");
    ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.5)"));
    ctx.set_line_width(1.);
    ctx.set_stroke_style(&JsValue::from_str("rgba(0,0,0,0.85)"));
    ctx.set_text_align("center");
    ctx.set_text_baseline("alphabetic");
}


fn run(_wnd: Window, cnv: HtmlCanvasElement, ctx: CanvasRenderingContext2d, imgs: Vec<HtmlImageElement>) -> Result<(), JsValue> {
    let mut spr_mov     = SpriteMovie::new(33, &imgs)
        .map_err(|_| JsValue::from_str("SpriteMovie creation error"))?;
    let graph           = SceneGraphLeaf::<f32>::new_node(&spr_mov);
    let mut renderer    = Renderer::new(&cnv)?;
    renderer.render(&graph)?;
    prepare_centered_message_style(&ctx, false);
    let text            = "Click to Play";
    ctx.fill_text(text, 450., 222.)?;
    ctx.stroke_text(text, 450., 222.)?;
    let mut ev_drv      = PointerEventDriver::new(cnv.clone())?;
    let mut emitter     = Shared::new(ev_drv.add_emitter(&graph));
    {
        let mut first_click     = true;
        emitter
            .subscribe(PointerEventType::Click, move |_| {
                if first_click {
                    first_click = false;
                    err_handler(
                        renderer
                            .set_auto_rendering(33, &graph)
                            .and_then(|_| spr_mov.play(false))
                    );
                }
            })
            .ok_or_else(|| JsValue::from_str("Couldn't subscribe event"))?;
    }
    Ok(())
}


fn load() -> Result<(), JsValue> {
    //
    let (wnd, cnv, ctx) = find_window_and_context()?;
    let mut loader  = BasicAssetLoader::<usize>::new();
    for img_no in 1usize .. 200 {
        let img_name = format!("img/Khan{:03}.jpg", img_no);
        loader.add_image_request(img_no, img_name)?;
    }
    {
        let ctx = ctx.clone();
        loader.add_progress_listener(move |percent| {
            prepare_centered_message_style(&ctx, true);
            let text = format!("Loading: {:>5.1}%", percent);
            err_handler(ctx
                .fill_text(&text, 450., 222.)
                .and_then(|()| ctx.stroke_text(&text, 450., 222.))
            );
        })?;
    }
    loader.add_loaded_listener(move |hmap| {
        let mut im_v = Vec::<HtmlImageElement>::with_capacity(200);
        let mut broken: Option<usize> = None;
        for img_no in 1usize .. 200 {
            if let Some(Loaded::Image(img)) = hmap.remove(&img_no) {
                im_v.push(img);
            }
            else {
                broken = Some(img_no);
                break;
            }
        }
        if let Some(num_failed) = broken {
            throw_val(JsValue::from_str(&format!(
                "Resource error: cannot extract movie frame #{:03}.",
                num_failed
            )));
        }
        else {
            let text = "Click anywhere to play movie.";
            prepare_centered_message_style(&ctx, true);
            err_handler(ctx
                .fill_text(&text, 450., 222.)
                .and_then(|()| ctx.stroke_text(text, 450., 222.))
                .and_then(|()| run(wnd.clone(), cnv.clone(), ctx.clone(), im_v))
            );
        }
    })?;
    loader.load_all()
}


#[wasm_bindgen(start)]
pub fn start() {
    err_handler(load());
}
