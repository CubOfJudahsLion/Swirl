//! Module  :   Main
//! Purpose :   Module structure and location of main function.
//!             For now, it just loads and displays a couple of images
//!             and allows a clickable button.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   2021-02-09 14:53
//! Modified:   Tue Jul  6 01:37:38 PM CST 2021


//  Please don't try to use this outside WASM.
#[cfg(not(target_arch = "wasm32"))]
mod failure {
    #[compiler_error("This program will work only in WebAssembly.")]
    struct WrongTargetArchitecture;
}


use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, throw_val};
use js_sys::Math;
use web_sys::{console, window};

use swirl::loader::manual::*;
use swirl::utils::*;
use swirl::{*, events::{*, interactive::pointer::{*, driver::*}}};
//use mingly::math::interpolation::hermite::*;

mod slot_reel;
mod assets;
mod res;

use slot_reel::*;
use assets::AssetRetrievalError;


const FPS   : f64 =   25.0;
const MSPF  : f64 = 1000.0 / FPS;

const REEL_COUNT    : usize =    5;
const H_RES         : i32   = 1611;
const SPACE_BETWEEN : i32   =    0;
const ICON_WIDTH    : u16   =  191;
const ICON_HEIGHT   : u16   =  180;
const X_START       : i32   = (H_RES - (ICON_WIDTH as i32 * REEL_COUNT as i32 + (SPACE_BETWEEN << 2))) >> 1;
const Y_START       : i32   =   81;
const VISIBLE_ICONS : u8    =    3;
const STRIP_LEN     : usize =  180;


//  We'll store icon information as a 
#[derive(Debug)]
struct IconInfo {
    symbol  : res::img::Icon,
    width   : u8,
    height  : u8,
    top     : i32,
    left    : i32,
}

const ICON_INFO: &[IconInfo] = &[
    IconInfo {symbol: res::img::Icon::A, width: 1, height: 1, top:   0, left:   0,},
    IconInfo {symbol: res::img::Icon::B, width: 1, height: 1, top:   0, left:   0,},
    IconInfo {symbol: res::img::Icon::C, width: 1, height: 3, top:   0, left:   0,},
    IconInfo {symbol: res::img::Icon::D, width: 1, height: 3, top:   0, left:   0,},
    IconInfo {symbol: res::img::Icon::E, width: 1, height: 1, top: -39, left: -18,},
    IconInfo {symbol: res::img::Icon::F, width: 1, height: 1, top: -20, left: -19,},
    IconInfo {symbol: res::img::Icon::G, width: 1, height: 1, top: -21, left: -19,},
    IconInfo {symbol: res::img::Icon::H, width: 1, height: 1, top: -36, left: -24,},
    IconInfo {symbol: res::img::Icon::I, width: 2, height: 2, top: -44, left: -39,},
    IconInfo {symbol: res::img::Icon::J, width: 2, height: 3, top: -12, left: -22,},
    IconInfo {symbol: res::img::Icon::W, width: 1, height: 1, top: -61, left: -16,},
    IconInfo {symbol: res::img::Icon::X, width: 1, height: 1, top:   0, left:   0,},
];


//  Creates and populates the spinning reels.
fn populate_reels_randomly(assets: &assets::All) -> Result<Vec<Reel>, JsValue> {
    //  Create the symbol vectors.
    let mut sym_vecs: Vec<Vec<Option<SymbolSpace>>> = Vec::with_capacity(REEL_COUNT);
    //  Populate them with "unfilled" values (`None`, in this case.)
    for _ in 0 .. REEL_COUNT {
        let mut vec = Vec::<Option<SymbolSpace>>::with_capacity(STRIP_LEN);
        for _ in 0 .. STRIP_LEN {
            vec.push(None);
        }
        //sym_vecs.push(vec![None; STRIP_LEN]);
        sym_vecs.push(vec);
    }
    //  The loop goes column by column randomly choosing symbols of various
    //  sizes. This means a symbol choice can fill up spaces in the next columns,
    //  so it has to check for filled spaces.
    for sym_col in 0 .. REEL_COUNT {
        let mut sym_row = 0_usize;
        while sym_row < STRIP_LEN {
            //  Skip filled spaces (by iterations in previous columns)
            if sym_vecs[sym_col][sym_row].is_some() {
                sym_row += 1;
            }
            else {
                //  Produce a random symbol. Repeat if it doesn't fit.
                'until_symbol_fits: loop {
                    let rand_idx    = (Math::random() * ICON_INFO.len() as f64) as usize;
                    let icon        = &ICON_INFO[rand_idx];
                    if      icon.width  as usize + sym_col  <= REEL_COUNT
                        &&  icon.height as usize + sym_row  <= STRIP_LEN
                        &&  sym_vecs[sym_col .. sym_col + icon.width as usize].iter().all(|vec|
                                vec[sym_row .. sym_row + icon.height as usize].iter().all(|sym|
                                    sym.is_none()
                                )
                            )
                    {
                        console::log_1(&JsValue::from_str("Found symbol"));
                        let img                     = assets.images.icon[icon.symbol].clone();
                        let node                    = SceneGraphLeaf::new_node(Sprite::new(img), None);
                        let ofs                     = Point::new(icon.left, icon.top);
                        match (icon.width, icon.height) {
                            (w, 1)  => {
                                for col in 1 .. w {
                                    //  We use blanks to the right of the original square of any
                                    //  symbols that are more than 1 square wide.
                                    //  It's been rendered already when found in the previous column.
                                    sym_vecs[sym_col + col as usize][sym_row] = Some(SymbolSpace::Blank);
                                }
                                sym_vecs[sym_col][sym_row] = Some(SymbolSpace::Single(icon.symbol, node, ofs));
                                sym_row += 1;
                            },
                            (w, h)  => {
                                for col in 1 .. w {
                                    for row in 0 .. h {
                                        sym_vecs[sym_col + col as usize][sym_row + row as usize] = Some(SymbolSpace::Blank);
                                    }
                                }
                                //  In the symbol's row, though, we place references to the
                                //  scene graph node and the row counts upward and downward.
                                //  The renderer needs this for when a multi-row symbol is
                                //  partially displayed.
                                for row in 0 .. h {
                                    sym_vecs[sym_col][sym_row + row as usize] = Some(
                                        SymbolSpace::Multiple(icon.symbol, node.clone(), row, h - row - 1, ofs)
                                    );
                                }
                                sym_row += h as usize;
                            },
                        }
                        // Symbol inserted. Next row.
                        break 'until_symbol_fits;
                    }
                }
            }
        }
    }
    //  The process should have left no slot with a "None" value. Check for that.
    if sym_vecs.iter().any(|reel| reel.iter().any(|sym| sym.is_none())) {
        return Err(JsValue::from_str("populate_reels_randomly(): not all symbols filled in."));
    }
    console::log_1(&JsValue::from_str("before debug reel dump"));
    for row in 0 .. STRIP_LEN {
        for col in 0 .. REEL_COUNT as usize {
            console::info_1(&JsValue::from_str(&format!("{:?}", sym_vecs[col][row])));
        }
    }
    console::log_1(&JsValue::from_str("after debug reel dump"));
    //  If the previous condition is met, we unwrap the values and use them
    //  to construct the reels.
    let mut reels = Vec::<Reel>::with_capacity(REEL_COUNT);
    for i in 0 .. REEL_COUNT as i32 {
        reels.push(
            Reel::new(
                X_START + i * (ICON_WIDTH as i32 + SPACE_BETWEEN), Y_START,
                ICON_HEIGHT, ICON_WIDTH,
                VISIBLE_ICONS,
                sym_vecs
                    .remove(0)
                    .drain(..)
                    .map(Option::unwrap)
                    .collect::<Vec<_>>()
            ).map_err(|_| JsValue::from_str("Unable to create reel"))?
        );
    }
    Ok(reels)
}


fn setup_rendering(assets: assets::All) -> Result<(), JsValue> {
    //console::log_1(&JsValue::from_str("reached setup_rendering"));
    //  Create shared resources.
    let canvas              = Rc::new(get_canvas()?);
    let mut renderer        = Shared::new(Renderer::new(canvas.clone())?);
    let mut scene           = SceneGraphFrame::new(None, None, None);
    scene.append_new_drawable(Sprite::new(assets.images.single.bkg.clone()), None);
    scene.append_new_drawable(Sprite::new(assets.images.single.interface.clone()), Some(Point::new(315, 660)));
    let mut play_btn        = Button::new(
        assets.images.play_button.normal.clone(),
        Some(assets.images.play_button.over.clone()),
        Some(assets.images.play_button.down.clone()),
        ButtonDisabledAppearance::Absent
    );
    let btn_node            = scene.append_new_drawable(play_btn.clone(), Some(Point::new(1136, 660)));
    //let mut reel0           = reels[0].clone();
    let reels               = populate_reels_randomly(&assets)?;
    for reel in reels.iter() {
        SceneGraphFrame::append(
            &mut scene,
            reel.scene_node()
        )
        .map_err(|_| JsValue::from_str(&"Failure adding reel to scene."))?;
    }
    //console::log_1(&JsValue::from_str("Done setting up scene"));
    // That button needs an event driver.
    let mut mouse_driver    = PointerEventDriver::new(canvas.clone())?;
    //console::log_1(&JsValue::from_str("Done creating event driver"));
    let mut mouse_events    = mouse_driver.add_emitter(&btn_node.global_bounds());
    //console::log_1(&JsValue::from_str("Done creating event dispatcher"));
    //  Initial render.
    let mut scene           = SceneGraphNode::Frame(scene);
    renderer.render(&mut scene)?;
    //console::log_1(&JsValue::from_str("Done with first render."));
    //  Set some event responses.
    let label               = Rc::new(get_label()?);
    {
        let play_btn    = play_btn.clone();
        let label       = label.clone();
        mouse_events.subscribe(PointerEventType::Move, move |mev: &PointerEvent| {
            if play_btn.button_state() != ButtonState::Disabled {
                label.set_inner_text(&format!("(Move {}, {})", mev.pos().x, mev.pos().y));
            }
        });
    }
    {
        let mut play_btn    = play_btn.clone();
        let label           = label.clone();
        mouse_events.subscribe(PointerEventType::Enter, move |_mev: &PointerEvent| {
            if play_btn.button_state() != ButtonState::Disabled {
                label.set_inner_text("Enter");
                play_btn.set_button_state(ButtonState::Hover);
            }
        });
    }
    {
        let mut play_btn    = play_btn.clone();
        let label           = label.clone();
        mouse_events.subscribe(PointerEventType::Leave, move |_mev: &PointerEvent| {
            if play_btn.button_state() != ButtonState::Disabled {
                label.set_inner_text("Leave");
                play_btn.set_button_state(ButtonState::Default);
            }
        });
    }
    {
        let mut play_btn    = play_btn.clone();
        let label           = label.clone();
        mouse_events.subscribe(PointerEventType::Down, move |_mev: &PointerEvent| {
            if play_btn.button_state() != ButtonState::Disabled {
                label.set_inner_text("Down");
                play_btn.set_button_state(ButtonState::Down);
            }
        });
    }
    {
        let mut play_btn    = play_btn.clone();
        let label           = label.clone();
        mouse_events.subscribe(PointerEventType::Up, move |_mev: &PointerEvent| {
            if play_btn.button_state() != ButtonState::Disabled {
                label.set_inner_text("Up");
                play_btn.set_button_state(ButtonState::Hover);
            }
        });
    }
    {
        let label = label.clone();
        //let mut renderer = renderer.clone();
        mouse_events.subscribe(PointerEventType::Click, move |_mev: &PointerEvent| {
            if play_btn.button_state() != ButtonState::Disabled {
                label.set_inner_text("Spinning reels.");
                //label.set_inner_text("Graphing.");
                //renderer.clear_auto_rendering().expect_throw("Couldn't clear auto-rendering");
                play_btn.set_button_state(ButtonState::Disabled);
                //
                //let context = renderer.borrow_canvas();
                //let bottom = 180.0f32 * 60.0 + 20.0;
                //let mspf = 25.0;
                //use HermiteTangentMode::*;
                //let mut anim_curve  = HermitePiecewiseInterpolator::<f32, f32>::new(
                //    //KonachekBartels {continuity: -1.0, tension: 1.0, bias: 0.0},
                //    //Cardinal(0.5),
                //    FiniteDifference,
                //    &[
                //        ( -mspf,    0.0,        None),
                //        (   0.0,    0.0,        None),
                //        (  mspf*10.0, 100.0,    Some(KonachekBartels {continuity: 0.0, tension: 0.5, bias: 0.0})),
                //        //( 300.0, -15.0),
                //        //( 450.0, -15.0),
                //        //( 600.0, -15.0),
                //        (3525.0, bottom + 20.0, Some(Linear)),
                //        //(3575.0, bottom +  5.0, None),
                //        //(3600.0, bottom +  2.5, None),
                //        //(3624.0, bottom +  1.0, None),
                //        (3625.0, bottom,        None),
                //        (3700.0, bottom,        None),
                //    ],
                //    //&[
                //    //    ( -mspf,   0.0,                 None),
                //    //    (   0.0,   0.0,                 None),
                //    //    (  mspf,  20.0,                 None),
                //    //    ( 200.0, 200.0 * (bottom + 20.0) / 3525.0, None), //Some(HermiteTangentMode::KonachekBartels {continuity: -1.0, tension: 1.0, bias: 0.0})),
                //    //    //( 300.0, -15.0),
                //    //    //( 450.0, -15.0),
                //    //    //( 600.0, -15.0),
                //    //    (3325.0, 3325.0 * (bottom + 20.0) / 3525.0, None), //Some(HermiteTangentMode::KonachekBartels {continuity: -1.0, tension: 1.0, bias: 0.0})), //HermiteTangentMode::Linear)),
                //    //    (3525.0, bottom + 20.0,         None),
                //    //    (3625.0, bottom,                None),
                //    //    (3700.0, bottom,                None),
                //    //]
                //).expect_throw("Couldn't create animation curve.");
                //HermitePiecewiseInterpolator::<f32, f32>::graph(&mut anim_curve, context)
                //    .expect_throw("Can't graph Hermite interpolator.");
                //
                let reels_stopped   = Rc::new(RefCell::new(0_u8));
                let closure_vec     = Rc::new(RefCell::new(Vec::<Closure<dyn FnMut()>>::with_capacity(5)));
                for i in 0_i32 .. 5 {
                    let reels           = reels.clone();
                    let reels_stopped   = reels_stopped.clone();
                    let play_btn        = play_btn.clone();
                    let closure_vec_cp  = closure_vec.clone();
                    let closure         = Closure::wrap(Box::new(move || {
                        let mut reels       = reels.clone();
                        let reels_stopped   = reels_stopped.clone();
                        let mut play_btn    = play_btn.clone();
                        let closure_vec_cp  = closure_vec_cp.clone();
                        reels[i as usize].start_spin(
                            MSPF as i32,
                            move || {
                                reels_stopped.deref().replace_with(|&mut reels_stopped| {
                                    let reels_stopped = reels_stopped + 1;
                                    if reels_stopped == 5 {
                                        closure_vec_cp.deref().borrow_mut().clear();
                                        play_btn.set_button_state(ButtonState::Default);
                                        0
                                    }
                                    else {
                                        reels_stopped
                                    }
                                });
                            }
                        ).expect_throw(&format!("Could not spin reel #{}.", i));
                    }) as Box<dyn FnMut()>);
                    match window()
                        .ok_or_else(|| JsValue::from_str("Cannot find main window."))
                        .and_then(|wnd| wnd.
                            set_timeout_with_callback_and_timeout_and_arguments_0(
                                closure.as_ref().unchecked_ref(),
                                0 //i * 150
                            )
                        )
                    {
                        Ok(_)       => { closure_vec.deref().borrow_mut().push(closure); },
                        Err(err)    => { throw_val(err); },
                    }
                }
            }
        });
    }
    //{
    //    mouse_events.subscribe(PointerEventType::DblClick, move |_mev: &PointerEvent| {
    //        if play_btn.button_state() != ButtonState::Disabled {
    //            label.set_inner_text(_mev.type_().into());
    //        }
    //    });
    //}
    //console::log_1(&JsValue::from_str("Done setting mouse event listeners"));
    //  Now set auto-rendering.
    console::log_1(&JsValue::from_str(&format!("FPS: {}, MSPF: {}.", FPS, MSPF)));
    renderer.set_auto_rendering(MSPF as i32, scene)
        .map_err(|_| JsValue::from_str("Cannot start auto-renderer."))?;
    //  If we made it this far, signal successful completion.
    //console::log_1(&JsValue::from_str("Done starting auto-renderer"));
    Ok(())
}


fn mise_en_scene(assets: &mut HashMap<res::All, Loaded>) -> Result<(), JsValue> {
    //console::log_1(&JsValue::from_str("Reached mise_en_scene"));
    match assets::All::new(assets) {
        Ok(images)                      =>
            setup_rendering(images),
        Err(AssetRetrievalError(file))  =>
            //console::error_1(&JsValue::from_str(&format!("Error: couldn't retrieve {}.", file))),
            Err(JsValue::from_str(&format!("Error: couldn't retrieve {}.", file))),
    }
}


fn load() -> Result<(), JsValue> {
    //console::log_1(&JsValue::from_str("Reached load"));
    let mut loader  = BasicAssetLoader::<res::All>::new();
    let label_1     = get_label()?;
    let label_2     = label_1.clone();
    {
        use res::ResourceSet;
        for variant in res::All::ordered_iter() {
            match *variant {
                res::All::Img(_)    => { loader.add_image_request(*variant, variant.file_for())?; },
                res::All::Font(_)   => { loader.add_font_request(*variant, variant.file_for())?; },
            }
        }
        loader
            .add_progress_listener(move |percentage| {
                let msg = format!("Loaded: {}%", percentage.round() as u16);
                console::log_1(&JsValue::from_str(&msg));
                label_1.set_inner_text(&msg);
            })?
            .add_loaded_listener(move |assets| {
                // Inform results
                //console::log_1(&JsValue::from_str(&"In loading callback."));
                label_2.set_inner_text("Loaded.");
                // Give success notice.
                if let Err(js_val) = mise_en_scene(assets) {
                    console::error_1(&js_val)
                }
            })?;
    }
    loader.load_all()
}


#[wasm_bindgen(start)]
pub extern "C" fn start() {
    console::info_1(&JsValue::from_str("WASM+Rust test\n©2021 Multislot Inc."));
    if let Err(jval) = load() {
        console::error_1(&jval);
    }
}
