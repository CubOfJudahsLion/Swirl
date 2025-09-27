//! Module  :   Lib (Main)
//! Purpose :   Module structure and location of main function.
//!             For now, it just loads and displays a couple of images
//!             and allows a clickable button.
//! Version :   0.2.0
//! Author  :   Alex Feterman
//! Created :   2021-02-09 14:53
//! Modified:   Fri 01/21/2022 02:38 PM


//  Please don't try to use this outside WASM.
#[cfg(not(target_arch = "wasm32"))]
mod failure {
    #[compiler_error("This program will work only in WebAssembly.")]
    struct WrongTargetArchitecture;
}


use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, throw_val, UnwrapThrowExt};
use js_sys::Math;
use web_sys::{console, window, HtmlDivElement, HtmlIFrameElement, HtmlLabelElement};

use swirl::*;
use swirl::style::shorten::*;

mod assets;
use assets::*;

mod slot_reel;
use slot_reel::*;

mod payout_lines;
use payout_lines::*;


pub const FPS           : f64   =   25.0;
pub const MSPF          : f64   = 1000.0 / FPS;

const REEL_COUNT        : usize =    5;
const H_RES             : i16   = 1016;
const V_RES             : i16   =  796;
const SPACE_BETWEEN     : i16   =    0;
pub const ICON_WIDTH    : u16   =  180;
pub const ICON_HEIGHT   : u16   =  180;
pub const X_START       : i16   = (H_RES - (ICON_WIDTH as i16 * REEL_COUNT as i16 + (SPACE_BETWEEN << 2))) >> 1;
pub const Y_START       : i16   =    0;
const VISIBLE_ICONS     : u8    =    3;
const STRIP_LEN         : usize =   61;

const CANVAS_ID         : &str  = "renderTarget";
const LABEL_ID          : &str  = "runInfo";
const IFRAME_ID         : &str  = "info";
const DIV_ID            : &str  = "backgroundHolder";

const PAY_LINES         : usize =   25;


//  Icon information allowing for multiple-row/column icons.
#[derive(Debug)]
struct IconInfo {
    symbol  : Icon,
    width   : u8,
    height  : u8,
    top     : i16,
    left    : i16,
}

const ICON_INFO: &[IconInfo] = &[
    IconInfo {symbol: Icon::A, width: 1, height: 1, top: 0, left: 0,},
    IconInfo {symbol: Icon::B, width: 1, height: 1, top: 0, left: 0,},
    IconInfo {symbol: Icon::C, width: 1, height: 1, top: 0, left: 0,},
    IconInfo {symbol: Icon::D, width: 1, height: 1, top: 0, left: 0,},
    IconInfo {symbol: Icon::W, width: 1, height: 1, top: 0, left: 0,},
];


//  Creates and populates the spinning reels.
fn populate_reels_randomly(assets: &AssetsStruct) -> Result<Vec<Reel>, JsValue> {
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
                        //console::log_1(&JsValue::from_str("Found symbol"));
                        let img     = assets[&icon.symbol.into()].clone();
                        let node    = SceneGraphLeaf::new_node(&Sprite::new(img));
                        let ofs     = Point::new(icon.left, icon.top);
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
    //console::log_1(&JsValue::from_str("before debug reel dump"));
    //for row in 0 .. STRIP_LEN {
    //    for col in 0 .. REEL_COUNT as usize {
    //        console::info_1(&JsValue::from_str(&format!("{:?}", sym_vecs[col][row])));
    //    }
    //}
    //console::log_1(&JsValue::from_str("after debug reel dump"));
    //  If the previous condition is met, we unwrap the values and use them
    //  to construct the reels.
    let mut reels = Vec::<Reel>::with_capacity(REEL_COUNT);
    for i in 0 .. REEL_COUNT as i16 {
        reels.push(
            Reel::new(
                X_START + i * (ICON_WIDTH as i16 + SPACE_BETWEEN), Y_START,
                ICON_HEIGHT, //ICON_WIDTH,
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


fn setup_rendering(assets: AssetsStruct) -> Result<(), JsValue> {
    //console::log_1(&JsValue::from_str("reached setup_rendering"));
    //  Create shared resources.
    let canvas                          = get_canvas(CANVAS_ID)?;
    let mut renderer                    = Renderer::new(&canvas)?;
    let mut scene                       = SceneGraphFrame::<f32>::new();
    let mut mouse_driver                = PointerEventDriver::<f32>::new(canvas.clone())?;
    let info_frame                      = get_html_element::<HtmlIFrameElement>(Some("iframe"), Some(IFRAME_ID), &[])?;
    //  Start with the assets.
    //  Background art for the winnings
    let winnings_bkg                    = Sprite::new(assets.imgs.interface.winnings_ornament.clone());
    let mut winnings_bkg_node           = SceneGraphLeaf::new_node(&winnings_bkg);
    winnings_bkg_node.set_x_and_y(300, 5);
    //  Buttons
    //let mut play_btn                = swirl::Button::new(
    //    assets.imgs.interface.button.play.normal.clone(),
    //    Some(assets.imgs.interface.button.play.over.clone()),
    //    Some(assets.imgs.interface.button.play.down.clone()),
    //    ButtonDisabledAppearance::Absent
    //);
    //let mut play_btn_node           = SceneGraphLeaf::<f32>::new_node(&play_btn);
    //let mut play_btn_node           = SceneGraphLeaf::<f32>::new_node(&play_btn);
    let ( mut play_btn
        , mut play_btn_node
        , mut play_events
        )                               =
        swirl::Button::new_node_with_logic(
            assets.imgs.interface.button.play.normal.clone(),
            Some(assets.imgs.interface.button.play.over.clone()),
            Some(assets.imgs.interface.button.play.down.clone()),
            ButtonDisabledAppearance::Absent,
            &mut mouse_driver
        );
    play_events.set_enabled(false);
    play_btn_node.set_x_and_y(838, 0);
    //let mut stop_btn                = swirl::Button::new(
    //    assets.imgs.interface.button.stop.normal.clone(),
    //    Some(assets.imgs.interface.button.stop.over.clone()),
    //    Some(assets.imgs.interface.button.stop.down.clone()),
    //    ButtonDisabledAppearance::Absent
    //);
    //let mut stop_btn_node           = SceneGraphLeaf::<f32>::new_node(&stop_btn);
    let ( mut stop_btn
        , mut stop_btn_node
        , mut stop_events
        )                               =
        swirl::Button::new_node_with_logic(
            assets.imgs.interface.button.stop.normal.clone(),
            Some(assets.imgs.interface.button.stop.over.clone()),
            Some(assets.imgs.interface.button.stop.down.clone()),
            ButtonDisabledAppearance::Absent,
            &mut mouse_driver
        );
    stop_events.set_enabled(false);
    stop_btn.set_button_state(ButtonState::Disabled);
    stop_btn_node.set_x_and_y(838, 0);
    //let mut info_btn                = swirl::Button::new(
    //    assets.imgs.interface.button.info.normal.clone(),
    //    Some(assets.imgs.interface.button.info.over.clone()),
    //    Some(assets.imgs.interface.button.info.down.clone()),
    //    ButtonDisabledAppearance::Absent
    //);
    //let mut info_btn_node           = SceneGraphLeaf::<f32>::new_node(&info_btn);
    let ( info_btn
        , mut info_btn_node
        , mut info_btn_events
        )                               =
        swirl::Button::new_node_with_logic(
            assets.imgs.interface.button.info.normal.clone(),
            Some(assets.imgs.interface.button.info.over.clone()),
            Some(assets.imgs.interface.button.info.down.clone()),
            ButtonDisabledAppearance::Alpha(0.5),
            &mut mouse_driver
        );
    info_btn_events.set_enabled(false);
    info_btn_node.set_x_and_y(177, 20);
    //let mut coin_value_btn          = swirl::Button::new(
    //    assets.imgs.interface.button.coin_value.normal.clone(),
    //    Some(assets.imgs.interface.button.coin_value.over.clone()),
    //    Some(assets.imgs.interface.button.coin_value.down.clone()),
    //    ButtonDisabledAppearance::Absent
    //);
    //let mut coin_value_btn_node     = SceneGraphLeaf::<f32>::new_node(&coin_value_btn);
    let ( mut coin_value_btn
        , mut coin_value_btn_node
        , mut coin_value_events  
        )                               =
        swirl::Button::new_node_with_logic(
            assets.imgs.interface.button.coin_value.normal.clone(),
            Some(assets.imgs.interface.button.coin_value.over.clone()),
            Some(assets.imgs.interface.button.coin_value.down.clone()),
            ButtonDisabledAppearance::Alpha(0.5),
            &mut mouse_driver
        );
    //let mut coin_value_btn_node     = SceneGraphLeaf::<f32>::new_node(&coin_value_btn);
    coin_value_events.set_enabled(false);
    coin_value_btn_node.set_x_and_y(38, 5);
    //  Texts
    let mut coin_value                  = 0.25_f32;
    let allowed_coin_values             = [0.25, 0.5, 1., 2., 5.];
    let mut coin_value_val              = Text::new(
        &format!("{:.2}", coin_value),
        FontStyleBuilder::new()
            .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
            .with_size(FontSizeAsUnits(24., Pt))
            .with_fill(Some(Named(White)))
            .with_align(AlignCenter)
            .build()
    )?;
    let mut coin_value_val_node         = SceneGraphLeaf::<f32>::new_node(&coin_value_val);
    coin_value_val_node.set_x_and_y(90, 67);
    let winnings_lbl                    = Text::new(
        "WIN",
        FontStyleBuilder::new()
            .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
            .with_size(FontSizeAsUnits(24., Pt))
            .with_fill(Some(RGB(0xfe, 0xff, 0x11)))
            .with_align(AlignCenter)
            .build()
    )?;
    let mut winnings_lbl_node           = SceneGraphLeaf::<f32>::new_node(&winnings_lbl);
    winnings_lbl_node.set_x_and_y(511, 25);
    let winnings_val                    = Text::new(
        "$~~,~~~.~~",
        FontStyleBuilder::new()
            .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
            .with_size(FontSizeAsUnits(40., Pt))
            .with_fill(Some(Named(White)))
            .with_align(AlignCenter)
            .build()
    )?;
    let mut winnings_val_node           = SceneGraphLeaf::new_node(&winnings_val);
    winnings_val_node.set_x_and_y(511, 87);
    let total_bet_lbl                   = Text::new(
        "TOTAL BET",
        FontStyleBuilder::new()
            .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
            .with_size(FontSizeAsUnits(18.75, Pt))
            .with_fill(Some(Named(White)))
            .build()
    )?;
    let mut total_bet_lbl_node          = SceneGraphLeaf::<f32>::new_node(&total_bet_lbl);
    total_bet_lbl_node.set_x_and_y(38, 133);
    let mut total_bet_val               = Text::new(
        &format!("{:.2}", (PAY_LINES as f32 * coin_value)),
        FontStyleBuilder::new()
            .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
            .with_size(FontSizeAsUnits(16.875, Pt))
            .with_fill(Some(Named(White)))
            .with_align(AlignRight)
            .build()
    )?;
    let mut total_bet_val_node          = SceneGraphLeaf::<f32>::new_node(&total_bet_val);
    total_bet_val_node.set_x_and_y(256, 133);
    let credits_lbl                     = Text::new(
        "CREDITS",
        FontStyleBuilder::new()
            .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
            .with_size(FontSizeAsUnits(18.75, Pt))
            .with_fill(Some(RGB(0xfe, 0xff, 0x11)))
            .build()
    )?;
    let mut credits_lbl_node            = SceneGraphLeaf::<f32>::new_node(&credits_lbl);
    credits_lbl_node.set_x_and_y(400, 133);
    let credits_val                     = Text::new(
        "~~~.~~~",
        FontStyleBuilder::new()
            .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
            .with_size(FontSizeAsUnits(16.875, Pt))
            .with_fill(Some(Named(White)))
            .with_align(AlignRight)
            .build()
    )?;
    let mut credits_val_node            = SceneGraphLeaf::<f32>::new_node(&credits_val);
    credits_val_node.set_x_and_y(610, 133);
    //
    let line_anims                      = Rc::new(PaylineAnimations::new(&assets).map_err(|err| -> JsValue { err.into() })?);
    let interrupt_fn: Rc<RefCell<Option<Box<dyn FnOnce()>>>> = Rc::new(RefCell::new(None));
    //let rand_lines                      = get_random_lines();
    //console::log_1(&JsValue::from_str(&format!("Random pay line rows: {:?}", rand_lines[0])));
    //let mut payline_anims               = line_anims.line_layout_to_movies(&rand_lines[0], X_START, Y_START).map_err(|_| JsValue::from_str("Invalid layout rows"))?;
    //for (node, mov) in &mut payline_anims {
    //    scene.append_child(node.clone()).map_err(|_| JsValue::from_str("Cannot insert payline movie into scene"))?;
    //    mov.play(true)?;
    //}
    //  They'll all be placed inside an interface subgraph.
    let mut interface_section           = SceneGraphFrame::<f32>::new();
    interface_section.append_child(winnings_bkg_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(play_btn_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(stop_btn_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(info_btn_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(coin_value_btn_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(coin_value_val_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(winnings_lbl_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(winnings_val_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(total_bet_lbl_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(total_bet_val_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(credits_lbl_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.append_child(credits_val_node.clone()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    interface_section.set_x_and_y(0, 575);
    scene.append_child(SceneGraphNode::Frame(interface_section)).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    play_events.set_enabled(true);
    stop_events.set_enabled(true);
    info_btn_events.set_enabled(true);
    coin_value_events.set_enabled(true);
    //
    let reels                           = populate_reels_randomly(&assets)?;
    for (pos, reel) in reels.iter().enumerate() {
        scene.insert_child(pos, reel.scene_node()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
        //scene.prepend_child(reel.scene_node()).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    }
    //  The reels in place, we need to erase below them so they won't clutter the interface view.
    let reel_clipper                    = EraserRect::new(&Rect::<i16>::new(575, 0, V_RES, H_RES));
    let reel_clipper_leaf               = SceneGraphLeaf::<f32>::new_node(&reel_clipper);
    scene.insert_child(reels.len(), reel_clipper_leaf).map_err(|_| JsValue::from_str("Failure building scene graph."))?;
    //
    //console::log_1(&JsValue::from_str("Done setting up scene"));
    //  Initial render.
    let scene_node                      = SceneGraphNode::Frame(scene.clone());
    renderer.render(&scene_node, &BeforeRendering::Clear)?;
    //console::log_1(&JsValue::from_str("Done with first render."));
    //  Set some event responses.
    let label                           = get_label(LABEL_ID)?;
    for (button_name, emitter) in
        &mut[   (&"Play",       &mut play_events)
            ,   (&"Stop",       &mut stop_events)
            ,   (&"CoinValue",  &mut coin_value_events)
            ,   (&"Info",       &mut info_btn_events)
            ]
    {
        {
            //let play_btn    = play_btn.clone();
            let label       = label.clone();
            let button_name = button_name.to_string();
            emitter.subscribe(PointerEventType::Move, move |mev: &PointerEvent<f32>| {
                //if play_btn.button_state() != ButtonState::Disabled {
                    label.set_inner_text(&format!("({}: Move {}, {})", button_name, mev.pos().x, mev.pos().y));
                    ActionAfterEvent::NoAction
                //}
            });
        }
        {
            //let mut play_btn    = play_btn.clone();
            let label           = label.clone();
            let button_name = button_name.to_string();
            emitter.subscribe(PointerEventType::Enter, move |_mev: &PointerEvent<f32>| {
                //if play_btn.button_state() != ButtonState::Disabled {
                    label.set_inner_text(&format!("{}: Enter", button_name));
                    //play_btn.set_button_state(ButtonState::Hover);
                    ActionAfterEvent::NoAction
                //}
            });
        }
        {
            //let mut play_btn    = play_btn.clone();
            let label           = label.clone();
            let button_name = button_name.to_string();
            emitter.subscribe(PointerEventType::Leave, move |_mev: &PointerEvent<f32>| {
                //if play_btn.button_state() != ButtonState::Disabled {
                    label.set_inner_text(&format!("{}: Leave", button_name));
                    //play_btn.set_button_state(ButtonState::Default);
                    ActionAfterEvent::NoAction
                //}
            });
        }
        {
            //let mut play_btn    = play_btn.clone();
            let label           = label.clone();
            let button_name = button_name.to_string();
            emitter.subscribe(PointerEventType::Down, move |_mev: &PointerEvent<f32>| {
                //if play_btn.button_state() != ButtonState::Disabled {
                    label.set_inner_text(&format!("{}: Down", button_name));
                    //play_btn.set_button_state(ButtonState::Down);
                    ActionAfterEvent::NoAction
                //}
            });
        }
        {
            //let mut play_btn    = play_btn.clone();
            let label           = label.clone();
            let button_name = button_name.to_string();
            emitter.subscribe(PointerEventType::Up, move |_mev: &PointerEvent<f32>| {
                //if play_btn.button_state() != ButtonState::Disabled {
                    label.set_inner_text(&format!("{}: Up", button_name));
                    //play_btn.set_button_state(ButtonState::Hover);
                    ActionAfterEvent::NoAction
                //}
            });
        }
    }
    {
        let label = label.clone();
        coin_value_events.subscribe(PointerEventType::Click, move |_mev: &PointerEvent<f32>| {
            let idx     =   (   allowed_coin_values
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(i, cv)| {
                                        if cv == &coin_value {
                                            Some(i)
                                        }
                                        else {
                                            None
                                        }
                                    })
                                    .fold(0, |_acc, next| next)
                            +   1
                            )
                        %   allowed_coin_values.len();
            coin_value  =   allowed_coin_values[idx];
            let _       =   coin_value_val.set_text(format!("{}", coin_value));
            let _       =   total_bet_val.set_text(format!("{}", coin_value * PAY_LINES as f32));
            label.set_inner_text("Spinning reels.");
            ActionAfterEvent::NoAction
        });
    }
    {
        let label               = label.clone();
        //let play_btn            = play_btn.clone();
        //let play_events         = play_events.clone();
        //let mut stop_btn        = stop_btn.clone();
        //let mut coin_value_btn  = coin_value_btn.clone();
        let mut reels           = reels.clone();
        stop_events.subscribe(PointerEventType::Click, move |_mev: &PointerEvent<f32>| {
            label.set_inner_text("Forced reel stop.");
            for reel in &mut reels {
                let _ = reel.stop_spin();
            }
            //coin_value_btn.set_button_state(ButtonState::Default);
            //stop_btn.set_button_state(ButtonState::Disabled);
            //{
            //    get_window()
            //        .and_then(|wnd| {
            //            let mut play_btn        = play_btn.clone();
            //            let mut play_events     = play_events.clone();
            //            let maybe_closure: Rc<Cell<Option<Closure<dyn FnMut()>>>> = Rc::new(Cell::new(None));
            //            let closure = {
            //                let maybe_closure_copy = maybe_closure.clone();
            //                Closure::wrap(
            //                    Box::new(move || {
            //                        play_btn.set_button_state(ButtonState::Default);
            //                        play_events.set_enabled(true);
            //                        maybe_closure_copy.set(None);
            //                    }) as Box<dyn FnMut()>
            //                )
            //            };
            //            wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
            //                closure.as_ref().unchecked_ref(),
            //                500
            //            )?;
            //            maybe_closure.set(Some(closure));
            //            Ok(())
            //        })
            //        .unwrap_throw();
            //}
            ActionAfterEvent::NoAction
        });
    }
    {
        let label           = label.clone();
        let container       = get_html_element::<HtmlDivElement>(Some("div"), Some(DIV_ID), &[])?;
        let info_frame      = info_frame.clone();
        let mut info_btn    = info_btn.clone();
        {
            info_btn_events.subscribe(PointerEventType::Click, move |_mev: &PointerEvent<f32>| {
                label.set_inner_text("Showing mock payout table/info.");
                info_frame.style().set_property("visibility", "visible").unwrap_throw();
                info_btn.set_button_state(ButtonState::Disabled);
                let wnd = get_window().unwrap_throw();
                let maybe_timeout_closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
                let timeout_closure = {
                    let maybe_timeout_closure   = maybe_timeout_closure.clone();
                    let container               = container.clone();
                    let info_frame              = info_frame.clone();
                    let info_btn                = info_btn.clone();
                    Closure::wrap(
                        Box::new(move || {
                            let maybe_closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
                            let closure         = {
                                let mut info_btn    = info_btn.clone();
                                let info_frame      = info_frame.clone();
                                let maybe_closure   = maybe_closure.clone();
                                let container       = container.clone();
                                Closure::wrap(
                                    Box::new(move || {
                                        info_btn.set_button_state(ButtonState::Default);
                                        info_frame.style().set_property("visibility", "hidden").unwrap_throw();
                                        container
                                            .remove_event_listener_with_callback(
                                                "click",
                                                maybe_closure.borrow().as_ref().unwrap_throw().as_ref().unchecked_ref()
                                            )
                                            .unwrap_throw();
                                    }) as Box<dyn FnMut()>
                                )
                            };
                            container
                                .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                                .unwrap_throw();
                            maybe_closure.replace(Some(closure));
                            maybe_timeout_closure.replace(None);
                        }) as Box<dyn FnMut()>
                    )
                };
                wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_closure.as_ref().unchecked_ref(),
                    200
                ).unwrap_throw();
                maybe_timeout_closure.replace(Some(timeout_closure));
                ActionAfterEvent::NoAction
            });
        }
    }
    {
        //let mut renderer = renderer.clone();
        let mut play_events_cp  = play_events.clone();
        let line_anims          = line_anims.clone();
        play_events_cp.subscribe(PointerEventType::Click, move |_mev: &PointerEvent<f32>| {
            //if play_btn.button_state() != ButtonState::Disabled {
                if let Some(interrupt_fn) = interrupt_fn.replace(None) {
                    //console::log_1(&JsValue::from_str("Stopping any line animations."));
                    interrupt_fn();
                }
                label.set_inner_text("Spinning reels.");
                //label.set_inner_text("Graphing.");
                //renderer.clear_auto_rendering().expect_throw("Couldn't clear auto-rendering");
                play_btn.set_button_state(ButtonState::Disabled);
                coin_value_btn.set_button_state(ButtonState::Disabled);
                stop_btn.set_button_state(ButtonState::Default);
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
                for i in 0_i16 .. 5 {
                    let line_anims      = line_anims.clone();
                    let interrupt_fn    = interrupt_fn.clone();
                    let mut reels       = reels.clone();
                    let reels_stopped   = reels_stopped.clone();
                    let play_btn        = play_btn.clone();
                    let play_events     = play_events.clone();
                    let coin_value_btn  = coin_value_btn.clone();
                    let stop_btn        = stop_btn.clone();
                    let closure_vec_cp  = closure_vec.clone();
                    let scene           = scene.clone();
                    let closure         = Closure::wrap(Box::new(move || {
                        //let mut reels           = reels.clone();
                        let line_anims      = line_anims.clone();
                        let interrupt_fn    = interrupt_fn.clone();
                        let reels_stopped   = reels_stopped.clone();
                        let play_btn        = play_btn.clone();
                        let coin_value_btn  = coin_value_btn.clone();
                        let play_events     = play_events.clone();
                        let stop_btn        = stop_btn.clone();
                        let closure_vec_cp  = closure_vec_cp.clone();
                        let scene           = scene.clone();
                        reels[i as usize].start_spin(
                            MSPF as i32,
                            move || {
                                let line_anims          = line_anims.clone();
                                let closure_vec_cp      = closure_vec_cp.clone();
                                let interrupt_fn        = interrupt_fn.clone();
                                let play_events         = play_events.clone();
                                let play_btn            = play_btn.clone();
                                let mut stop_btn        = stop_btn.clone();
                                let mut coin_value_btn  = coin_value_btn.clone();
                                let mut scene           = scene.clone();
                                reels_stopped.deref().replace_with(move |&mut reels_stopped| {
                                    let reels_stopped = reels_stopped + 1;
                                    if reels_stopped == 5 {
                                        interrupt_fn.replace(Some(line_anims.play_random_line_movie_sets(&mut scene, X_START, Y_START).unwrap_throw()));
                                        closure_vec_cp.deref().borrow_mut().clear();
                                        //play_btn.set_button_state(ButtonState::Default);
                                        coin_value_btn.set_button_state(ButtonState::Default);
                                        stop_btn.set_button_state(ButtonState::Disabled);
                                        {
                                            get_window()
                                                .and_then(|wnd| {
                                                    let mut play_btn    = play_btn.clone();
                                                    let mut play_events = play_events.clone();
                                                    let maybe_closure: Rc<Cell<Option<Closure<dyn FnMut()>>>> = Rc::new(Cell::new(None));
                                                    let closure         = {
                                                        let maybe_closure_copy = maybe_closure.clone();
                                                        Closure::wrap(
                                                            Box::new(move || {
                                                                play_btn.set_button_state(ButtonState::Default);
                                                                play_events.set_enabled(true);
                                                                maybe_closure_copy.set(None);
                                                            }) as Box<dyn FnMut()>
                                                        )
                                                    };
                                                    wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
                                                        closure.as_ref().unchecked_ref(),
                                                        200
                                                    )?;
                                                    maybe_closure.set(Some(closure));
                                                    Ok(())
                                                })
                                                .unwrap_throw();
                                        }
                                        //coin_value_btn.set_button_state(ButtonState::Default);
                                        //stop_btn.set_button_state(ButtonState::Disabled);
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
                                MSPF as i32 / 2 //i * 150
                            )
                        )
                    {
                        Ok(_)       => { closure_vec.deref().borrow_mut().push(closure); },
                        Err(err)    => { throw_val(err); },
                    }
                }
            //}
            ActionAfterEvent::NoAction
        });
    }
    //{
    //    mouse_events.subscribe(PointerEventType::DblClick, move |_mev: &PointerEvent<f32>| {
    //        if play_btn.button_state() != ButtonState::Disabled {
    //            label.set_inner_text(_mev.type_().into());
    //        }
    //    });
    //}
    //console::log_1(&JsValue::from_str("Done setting mouse event listeners"));
    //  Now set auto-rendering.
    //console::log_1(&JsValue::from_str(&format!("FPS: {}, MSPF: {}.", FPS, MSPF)));
    renderer.set_auto_rendering(MSPF as i32, &scene_node, BeforeRendering::Clear)
        .map_err(|_| JsValue::from_str("Cannot start auto-renderer."))?;
    //  If we made it this far, signal successful completion.
    //console::log_1(&JsValue::from_str("Done starting auto-renderer"));
    Ok(())
}


fn load() -> Result<(), JsValue> {
    //console::log_1(&JsValue::from_str("Reached load"));
    let mut loader  = BasicAssetLoader::<Assets>::new();
    let label_1     = get_html_element::<HtmlLabelElement>(Some("label"), Some(LABEL_ID), &[])?;
    let label_2     = label_1.clone();
    {
        for variant in Assets::ordered_iter() {
            match *variant {
                Assets::FontTonicLiver  => {
                    loader.add_font_request(*variant, variant.to_string())?;
                },
                Assets::SndTheme        => {
                    loader.add_sound_request(
                        *variant,
                        vec![
                            variant.to_string(),
                            <Assets as Into<&str>>
                                ::into(*variant)
                                .replace(".mp3", ".ogg")
                        ]
                    )?;
                },
                img                     => {
                    loader.add_image_request(*variant, img.to_string())?;
                },
            }
        }
        loader
            .add_progress_listener(move |percentage| {
                let msg = format!("Loaded: {}%", percentage.round() as u16);
                //console::log_1(&JsValue::from_str(&msg));
                label_1.set_inner_text(&msg);
            })?
            .add_loaded_listener(move |asset_dict| {
                // Inform results
                //console::log_1(&JsValue::from_str(&"In loading callback."));
                label_2.set_inner_text("Loaded.");
                let assets = assets::AssetsStruct::new(asset_dict)
                    .map_err(|jsval| jsval
                        .as_string()
                        .ok_or("Loaded assets do not match required types.")
                    )
                    .unwrap_throw();
                if let Err(js_val) = setup_rendering(assets) {
                    console::error_1(&js_val)
                }
            })?;
    }
    loader.load_all()
}


#[wasm_bindgen(start)]
pub extern "C" fn start() {
    console::info_1(&JsValue::from_str("WASM+Rust Test v0.2.0\n©2021-22 Multislot Inc."));
    if let Err(jval) = load() {
        console::error_1(&jval);
    }
}
