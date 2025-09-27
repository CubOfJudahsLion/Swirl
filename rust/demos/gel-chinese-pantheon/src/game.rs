//! # Module        :   _Game_
//! - **Purpose**   :   Game implementation proper, after
//!                     the `swirl::SWIRLGame` trait.
//! - **Version**   :   0.4.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Tue 03/22/2022 08:34 AM
//! - **Modified**  :   Mon 07/04/2022 05:51 PM


use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, throw_val};
use js_sys::{Array, Function, Promise};
use web_sys::console;

use swirl::*;

use super::assets::*;
use super::game_builder::*;


#[allow(dead_code)]
enum GameInner {
    Unloaded,
    Loaded {
        data    : GameData,
        html    : GameHtmlElements,
        assets  : AssetsStruct,
        vis     : GameVisuals,
    },
}

#[derive(Clone)]
pub struct Game(Rc<RefCell<GameInner>>);

impl Game {
    pub fn new() -> Self { Game(Rc::new(RefCell::new(GameInner::Unloaded))) }

    fn is_unloaded(&self) -> bool {
        match &*self.0.borrow() {
            &GameInner::Unloaded    => true,
            _                       => false,
        }
    }

    fn run_loader(
        assets_mem  : Rc<RefCell<Option<AssetsStruct>>>,
        html_elems  : &GameHtmlElements
    )
        -> Promise
    {
        let prom_mem    : Rc<RefCell<Option<Box<dyn FnMut(Function, Function)>>>>
                        = Rc::new(RefCell::new(None));
        let mut prom_fn = {
            let prom_mem    = prom_mem.clone();
            let assets_mem  = assets_mem.clone();
            let html_elems  = html_elems.clone();
            let mut loader  = BasicAssetLoader::<Assets>::new();
            move |resolve: Function, reject: Function| {
                for variant in Assets::ordered_iter() {
                    match *variant {
                        Assets::Font(FontAssets::TonicLiver)        => {
                            if loader.add_font_request(*variant, variant.to_string()).is_err() {
                                let _ = reject.call1(
                                    &JsValue::NULL,
                                    &JsValue::from_str("Could not add load request for font \"LHFTonicLIVER.woff\"")
                                );
                                prom_mem.replace(None);
                                return;
                            }
                        },
                        img                                         => {
                            let img_as_str  = img.to_string();
                            let img_resize  = {
                                match img {
                                    Assets::Img(ImgAssets::Interface(intf)) => {
                                        match intf {
                                            _   =>
                                                Resize::ScaleEqually(0.6)
                                        }
                                    },
                                    _                                       =>
                                        Resize::NoResize,
                                }
                            };
                            if  loader.add_image_request(
                                    *variant,
                                    img_as_str,
                                    img_resize
                                )
                                .is_err()
                            {
                                let _ = reject.call1(
                                    &JsValue::NULL,
                                    &JsValue::from_str(&format!(
                                        "Could not add image load request for image \"{}\"",
                                        img.to_string()
                                    ))
                                );
                                prom_mem.replace(None);
                                return;
                            }
                        },
                    }
                }
                let add_listeners_result = loader
                    .add_progress_listener({
                        let label   = html_elems.load_label.clone();
                        move |percentage| {
                            label.set_inner_text(&format!("Loaded: {}%", percentage.round() as u16));
                        }
                    })
                    .and_then(|loader| loader
                        .add_loaded_listener({
                            let label       = html_elems.load_label.clone();
                            let load_div    = html_elems.load_div.clone();
                            let assets_mem  = assets_mem.clone();
                            let prom_mem    = prom_mem.clone();
                            let reject      = reject.clone();
                            move |asset_dict| {
                                // Inform results
                                //#[cfg(debug_assertions)]
                                //console::log_1(&JsValue::from_str(&"In loading callback."));
                                label.set_inner_text("Loaded.");
                                let _ = load_div.style().set_property("visibility", "hidden");
                                match AssetsStruct::new(asset_dict) {
                                    Ok(assets)  => {
                                        assets_mem.replace(Some(assets));
                                        let _ = resolve.call0(&JsValue::NULL);
                                        prom_mem.replace(None);
                                    },
                                    Err(js_err) => {
                                        let _ = reject.call1(
                                            &JsValue::NULL,
                                            &js_err
                                        );
                                    },
                                }
                            }
                        })
                    );
                if add_listeners_result.is_ok() {
                    if let Err(js_err) = loader.load_all() {
                        let _ = reject.call1(
                            &JsValue::NULL,
                            &js_err
                        );
                        prom_mem.replace(None);
                    }
                }
                else {
                    let _ = reject.call1(
                        &JsValue::NULL,
                        &JsValue::from_str("Failed adding progress/load listeners.")
                    );
                    prom_mem.replace(None);
                }
            }
        };
        let prom        = Promise::new(&mut prom_fn);
        prom_mem.replace(Some(Box::new(prom_fn) as Box<dyn FnMut(Function, Function)>));
        prom
    }
}


impl SWIRLGame for Game {
    fn init_load(
        &mut self,
        handler         : GameHandler,
        language        : String,
        all_symbols     : String,
        initial_reels   : Box<[String]>,
        full_reels      : Box<[String]>,
        all_coins       : Option<Box<[String]>>,
        line_paths      : Box<[String]>,
        translations    : HashMap<String, String>
    )
        -> Promise
    {
        if self.is_unloaded() {
            match GameHtmlElements::new() {
                Ok(html)    => {
                    let data    = Rc::new(RefCell::new(Some(GameData {
                        handler: Rc::new(handler),
                        language,
                        all_symbols,
                        initial_reels,
                        full_reels,
                        all_coins,
                        line_paths,
                        translations,
                    })));
                    let assets      : Rc<RefCell<Option<AssetsStruct>>>
                                    = Rc::new(RefCell::new(None));
                    let clos_mem    : Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>
                                    = Rc::new(RefCell::new(None));
                    let clos_fn     = {
                        let assets      = assets.clone();
                        let clos_mem    = clos_mem.clone();
                        let data        = data.clone();
                        let html        = html.clone();
                        let this        = self.clone();
                        Closure::wrap(Box::new(move |_| {
                            let data = data.replace(None).unwrap();
                            match assets.replace(None)
                                .ok_or_else(|| JsValue::from_str("Failed to unpack assets"))
                                .and_then(|assets| GameVisuals::new(&data, &assets, &html)
                                    .map(move |vis| (assets, vis))
                                )
                            {
                                Ok((assets, vis))   => {
                                    let ratios_array    = Array::new();
                                    vis.layouts.0
                                        .iter()
                                        .map(|ldr| Array::of3(
                                            &JsValue::from(ldr.dims.width),
                                            &JsValue::from(ldr.dims.height),
                                            &JsValue::from(ldr.ratio)
                                        ))
                                        .for_each(|arr| { ratios_array.push(&arr); });
                                    this.0.replace(GameInner::Loaded {
                                        data,
                                        html: html.clone(),
                                        assets,
                                        vis,
                                    });
                                    set_aspect_ratios(ratios_array.unchecked_into());
                                },
                                Err(js_err)             => {
                                    console::error_1(&js_err);
                                },
                            }
                            clos_mem.replace(None);
                        }) as Box<dyn FnMut(JsValue)>)
                    };
                    let prom = Game::run_loader(assets, &html)
                        .then(&clos_fn);
                    clos_mem.replace(Some(clos_fn));
                    prom
                },
                Err(js_err) =>
                    Promise::reject(&js_err),
            }
        }
        else {
            let js_err = JsValue::from_str("Game has been load()'ed already.");
            console::error_1(&js_err);
            Promise::reject(&js_err)
        }
    }

    fn update_balance(&mut self, balance: String) {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                if vis.texts.credits_val.set_text(balance).is_err() {
                    console::error_1(&JsValue::from_str("Failed setting game balance."));
                }
            },
            _                               => {
                console::error_1(&JsValue::from_str("Game has not been load()'ed."));
            },
        }
    }

    fn reel_spin(&mut self, reel: u8) {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                if vis.reels.reel_set.len() < (reel as usize) {
                    console::error_1(&JsValue::from_str(&format!("Unknown reel: #{}", reel)));
                }
                else {
                    vis.reels.reel_set[reel as usize - 1].start_spin();
                }
            },
            _                               => {
                console::error_1(&JsValue::from_str("Game has not been load()'ed."));
            },
        }
    }

    fn reel_stop(
        &mut self,
        reel        : u8,
        stop_symbols: String,
        instant     : bool,
        flags       : Option<Box<[String]>>
    )
        -> Promise
    {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                if vis.reels.reel_set.len() < (reel as usize) {
                    let js_err = JsValue::from_str(&format!("Unknown reel: #{}", reel));
                    console::error_1(&js_err);
                    Promise::reject(&js_err)
                }
                else {
                    vis.reels.reel_set[reel as usize - 1].stop_spin(stop_symbols, instant, flags)
                }
            },
            _                               => {
                let js_err = JsValue::from_str("Game has not been load()'ed.");
                console::error_1(&js_err);
                Promise::reject(&js_err)
            },
        }
    }

    fn set_spin_enabled(&mut self, enabled: bool) {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                vis.buttons.spin.set_button_state(
                    if enabled {
                        ButtonState::Default
                    }
                    else {
                        ButtonState::Disabled
                    }
                );
                vis.buttons.spin_events.set_enabled(enabled);
            },
            _                               => {
                let js_err = JsValue::from_str("Game has not been load()'ed.");
                console::error_1(&js_err);
            },
        }
    }

    fn set_stop_enabled(&mut self, enabled: bool) {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                vis.buttons.stop.set_button_state(
                    if enabled {
                        ButtonState::Default
                    }
                    else {
                        ButtonState::Disabled
                    }
                );
                vis.buttons.stop_events.set_enabled(enabled);
            },
            _                               => {
                let js_err = JsValue::from_str("Game has not been load()'ed.");
                console::error_1(&js_err);
            },
        }
    }

    fn set_change_enabled(
        &mut self,
        change_coin_value   : bool,
        _change_line_coins  : bool,
        _change_lines_bet   : bool,
        _change_total_bet   : bool
    )
    {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                let button_state = 
                    if change_coin_value {
                        ButtonState::Default
                    }
                    else {
                        ButtonState::Disabled
                    };
                vis.buttons.coin_minus.set_button_state(button_state);
                vis.buttons.coin_plus.set_button_state(button_state);
                vis.buttons.coin_minus_events.set_enabled(change_coin_value);
                vis.buttons.coin_plus_events.set_enabled(change_coin_value);
            },
            _                               => {
                let js_err = JsValue::from_str("Game has not been load()'ed.");
                console::error_1(&js_err);
            },
        }
    }

    fn update_total_bet(&mut self, total_bet: String) {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                if let Err(js_err) = vis.texts.total_bet_val.set_text(total_bet) {
                    console::error_1(&js_err);
                }
            },
            _                               => {
                let js_err = JsValue::from_str("Game has not been load()'ed.");
                console::error_1(&js_err);
            },
        }
    }

    //  These three do nothing in an AllWays machine.
    fn update_coin_value(&mut self, _cv: String) {}
    fn update_line_coins(&mut self, _lc: u32) {}
    fn update_lines_bet(&mut self, _lb: u32) {}

    fn clear_winnings(&mut self) {
        match &mut *self.0.borrow_mut() {
            GameInner::Loaded { vis, .. }   => {
                if let Err(js_err) = vis.texts.winnings_val.set_text("".to_string()) {
                    console::error_1(&js_err);
                }
            },
            _                               => {
                let js_err = JsValue::from_str("Game has not been load()'ed.");
                console::error_1(&js_err);
            },
        }
    }

    fn aspect_ratio_changed(&mut self, aspect_ratio_idx: usize) {
        #[cfg(debug_assertions)]
        console::info_1(&JsValue::from_str("Inside aspect_ratio_changed() method."));
        match unsafe { &mut *self.0.as_ptr() } {
            GameInner::Loaded { vis, .. }   => {
                #[cfg(debug_assertions)]
                console::info_1(&JsValue::from_str("Game is loaded, actually going for aspect ratio change."));
                if aspect_ratio_idx < vis.layouts.0.len() {
                    #[cfg(debug_assertions)]
                    console::info_1(&JsValue::from_str("Aspect ratio valid."));
                    #[cfg(debug_assertions)]
                    console::info_1(&JsValue::from_str("Stopping auto rendering..."));
                    vis.renderer
                        .clear_auto_rendering()
                        .expect_throw("Could not stop auto-rendering for aspect-ratio change.");
                    #[cfg(debug_assertions)]
                    console::info_1(&JsValue::from_str("Auto-rendering stopped."));
                    match vis.layouts.0[aspect_ratio_idx].layout
                        .fast_forward()
                        .ok_or_else(|| JsValue::null())
                        .or_else(|_| {
                            let ldr         = &mut vis.layouts.0[aspect_ratio_idx];
                            let scene_left  = GameVisuals::H_RES - ldr.dims.width  >> 1;
                            let scene_top   = GameVisuals::V_RES - ldr.dims.height >> 1;
                            ldr.layout.calc_graph(
                                &Rect {
                                    top     : scene_top,
                                    left    : scene_left,
                                    bottom  : scene_top  + ldr.dims.height - 1,
                                    right   : scene_left + ldr.dims.width  - 1,
                                },
                                None
                            )
                            .map(|calc_out| calc_out.graph)
                            .map_err(|err| JsValue::from_str(&err.to_string()))
                        })
                    {
                        Ok(graph)    => {
                            #[cfg(debug_assertions)]
                            console::info_1(&JsValue::from_str("Obtained SceneGraphNode, restarting auto-rendering."));
                            vis.renderer.set_auto_rendering(
                                Some(GameVisuals::FPS as i32),
                                &graph,
                                BeforeRendering::Clear
                            )
                            .expect_throw("Could not restart auto-rendering after aspect-ratio change.");
                            #[cfg(debug_assertions)]
                            console::info_1(&JsValue::from_str("Auto-rendering restarted."));
                        },
                        Err(err)        => {
                            //console::error_1(&err);
                            throw_val(err);
                        },
                    }
                }
                else {
                    console::error_1(&JsValue::from_str("aspectRatioChanged(): index out of range."));
                }
            },
            _                               => {
                let js_err = JsValue::from_str("Game has not been load()'ed.");
                console::error_1(&js_err);
            },
        }
        #[cfg(debug_assertions)]
        console::info_1(&JsValue::from_str("Exiting aspect_ratio_changed() method."));
    }
}
