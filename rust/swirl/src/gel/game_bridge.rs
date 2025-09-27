//! Module  :   Game bridge
//! Purpose :   Lower-level interface bridging JavaScript
//!             calls (validating, converting to Rust types)
//!             and the `GELGame` trait.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Fri 02/18/2022 09:13 AM
//! Modified:   Tue 04/26/2022 09:09 PM


use lazy_static::lazy_static;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{intern, throw_str};
use js_sys::Promise;
use web_sys::console;

use super::game_event_handler::*;
use super::conversion_utils::*;
use super::swirl_game::*;


//  Mutable globals suck, but we're not left a lot of choices by
//  `wasm_bindgen`'s limitation. This keeps the global game
//  implemenation.
static mut GLOBAL_CALLBACKS: Option<Box<dyn SWIRLGame>> = None;


/// Attempts to obtain the global set of `SWIRLGame`.
pub fn get_callbacks() -> Option<&'static mut dyn SWIRLGame> {
    match unsafe { GLOBAL_CALLBACKS.as_mut() } {
        Some(game)  => Some(&mut **game),
        _           => None,
    }
}

/// Attempts to obtain the global set of `SWIRLGame`, but
/// returning a `Result` that depends on the function name.
pub fn get_callbacks_res(fn_name: &str) -> Result<&'static mut dyn SWIRLGame, JsValue> {
    get_callbacks()
        .ok_or_else(|| JsValue::from_str(&format!(
            "{}: Error: Game has not been initialized",
            fn_name
        )))
}

/// Attempts to set the global `SWIRLGame`. This function can
/// be called only once and will throw on repeated attempts.
pub fn set_dyn_callbacks(callbacks: Box<dyn SWIRLGame>) {
    let already_set = unsafe { GLOBAL_CALLBACKS.is_some() };
    if already_set {
        throw_str("swirl::gel::game_bridge::set_game(): Internal error: game already set.");
    }
    else {
        unsafe { GLOBAL_CALLBACKS = Some(callbacks); }
    }
}

/// Same as `set_dyn_callbacks`, except that, being a generic
/// function, it doesn't require a `Box`ed parameter.
#[inline(always)]
pub fn set_callbacks<S: SWIRLGame + 'static>(callbacks: S) {
    set_dyn_callbacks(Box::new(callbacks) as Box<dyn SWIRLGame>);
}


/// The public SWIRL interface to the JavaScript side.
#[wasm_bindgen(js_name = initLoad)]
pub fn init_load(
    handler         : GameEventHandler,
    language        : String,   // string
    all_symbols     : String,   // string
    initial_reels   : JsValue,  // string[]
    full_reels      : JsValue,  // string[]
    all_coins       : JsValue,  // null|string[]
    line_paths      : JsValue,  // string[]
    translations    : JsValue   // Record<string, string> aka {[key: string]: string;}
)
    -> Promise
{
    let fn_name             = intern("swirl::gel::game_bridge::load()");
    let call_and_convert    = || -> Result<Promise, JsValue> {
        let game            = get_callbacks_res(fn_name)?;
        let initial_reels   = js_as_string_slice_box(initial_reels, fn_name, "initialReels")?;
        let full_reels      = js_as_string_slice_box(full_reels, fn_name, "fullReels")?;
        let all_coins       =
            if all_coins.is_null() || all_coins.is_undefined() {
                None
            }
            else {
                Some(js_as_string_slice_box(all_coins, fn_name, "allcoins")?)
            };
        let line_paths      = js_as_string_slice_box(line_paths, fn_name, "linepaths")?;
        let translations    = js_as_string_hashmap(translations, fn_name, "translations")?;
        Ok(game.init_load(
            GameHandler::new(handler),
            language,
            all_symbols,
            initial_reels,
            full_reels,
            all_coins,
            line_paths,
            translations
        ))
    };
    match call_and_convert() {
        Ok(prom)    =>
            prom,
        Err(js_err) => {
            console::error_1(&js_err);
            Promise::reject(&js_err)
        },
    }
}


#[wasm_bindgen(js_name = updateBalance)]
pub fn update_balance(balance: String) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::updateBalance()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => game.update_balance(balance),
        Err(js_err) => console::error_1(&js_err),
    }
}

#[wasm_bindgen(js_name = reelSpin)]
pub fn reel_spin(reel: u8) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::reelSpin()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => game.reel_spin(reel),
        Err(js_err) => console::error_1(&js_err),
    }
}

#[wasm_bindgen(js_name = reelStop)]
pub fn reel_stop(reel: u8, stop_symbols: String, instant: bool, js_flags: JsValue) -> Promise {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::reelStop()");
    }
    match get_callbacks_res(&FN_NAME)
        .and_then(|game|
            js_as_opt_string_slice_box(js_flags, &FN_NAME, "flags")
                .map(move |flags| (game, flags))
        )
    {
        Ok((game, flags))   => {
            game.reel_stop(reel, stop_symbols, instant, flags)
        },
        Err(js_err)         => {
            console::error_1(&js_err);
            Promise::reject(&js_err)
        },
    }
}

#[wasm_bindgen(js_name = setSpinEnabled)]
pub fn set_spin_enabled(enabled: bool) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::setSpinEnabled()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game) => game.set_spin_enabled(enabled),
        Err(js_err) => console::error_1(&js_err),
    }
}


#[wasm_bindgen(js_name = setStopEnabled)]
pub fn set_stop_enabled(enabled: bool) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::setStopEnabled()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => game.set_stop_enabled(enabled),
        Err(js_err) => console::error_1(&js_err),
    }
}

#[wasm_bindgen(js_name = clearWinnings)]
pub fn clear_winnings() {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::clearWinnings()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => game.clear_winnings(),
        Err(js_err) => console::error_1(&js_err),
    }
}

#[wasm_bindgen(js_name = setChangeEnabled)]
pub fn set_change_enabled(
    change_coin_value   : bool,
    change_line_coins   : bool,
    change_lines_bet    : bool,
    change_total_bet    : bool
)
{
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::setChangeEnabled()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    =>
            game.set_change_enabled(change_coin_value, change_line_coins, change_lines_bet, change_total_bet),
        Err(js_err) =>
            console::error_1(&js_err),
    }
}

#[wasm_bindgen(js_name = updateTotalBet)]
pub fn update_total_bet(total_bet: String) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::updateTotalBet()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => {
            game.update_total_bet(total_bet)
        },
        Err(js_err) => {
            console::error_1(&js_err);
        },
    }
}

#[wasm_bindgen(js_name = updateCoinValue)]
pub fn update_coin_value(coin_value: String) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::updateCoinValue()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => {
            game.update_coin_value(coin_value);
        },
        Err(js_err) => {
            console::error_1(&js_err);
        },
    }
}

#[wasm_bindgen(js_name = updateLineCoins)]
pub fn update_line_coins(line_coins: u32) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::updateLineCoins()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => {
            game.update_line_coins(line_coins);
        },
        Err(js_err) => {
            console::error_1(&js_err);
        },
    }
}

#[wasm_bindgen(js_name = updateLinesBet)]
pub fn update_lines_bet(lines_bet: u32) {
    lazy_static!{
        static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::updateLinesBet()");
    }
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => {
            game.update_lines_bet(lines_bet);
        },
        Err(js_err) => {
            console::error_1(&js_err);
        },
    }
}


lazy_static!{
    static ref FN_NAME: &'static str = intern("swirl::gel::game_bridge::aspectRatiochanged()");
}

#[wasm_bindgen(js_name = aspectRatioChanged)]
pub fn aspect_ratio_changed(aspect_ratio_index: usize) {
    #[cfg(debug_assertions)]
    console::info_1(&JsValue::from_str(&format!(
        "Inside aspect_ratio_change() bridge function.\nCalled with parameter: {}",
        aspect_ratio_index
    )));
    match get_callbacks_res(&FN_NAME) {
        Ok(game)    => {
            game.aspect_ratio_changed(aspect_ratio_index);
        },
        Err(js_err) => {
            console::error_1(&js_err);
        },
    }
    #[cfg(debug_assertions)]
    console::info_1(&JsValue::from_str("Exiting aspect_ratio_change() bridge function."));
}
