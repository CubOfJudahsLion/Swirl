//! Module  :   SWIRL Game trait
//! Purpose :   Specify our outward-facing interface
//!             for MultiSlot's GEL game-driver protocol.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu 02/17/2022 02:44 PM
//! Modified:   Mon 07/04/2022 06:27 PM


use std::collections::HashMap;
use js_sys::Promise;
use super::game_event_handler::*;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    /// Accepts the lists of ratios. Each element
    /// in the `Array` is itself an array with three
    /// elements in this order: width, height and ratio.
    #[wasm_bindgen(js_name = setAspectRatios)]
    pub fn set_aspect_ratios(ratios: JsValue);
}


pub trait SWIRLGame {
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
    ) -> Promise;

    fn update_balance(&mut self, balance: String);

    fn reel_spin(&mut self, reel: u8);

    fn reel_stop(&mut self, reel: u8, stop_symbols: String, instant: bool, flags: Option<Box<[String]>>) -> Promise;

    fn set_spin_enabled(&mut self, enabled: bool);

    fn set_stop_enabled(&mut self, enabled: bool);

    fn set_change_enabled(&mut self, change_coin_value: bool, change_line_coins: bool, change_lines_bet: bool, change_total_bet: bool);

    fn update_total_bet(&mut self, total_bet: String);

    fn update_coin_value(&mut self, coin_value: String);

    fn update_line_coins(&mut self, line_coins: u32);

    fn update_lines_bet(&mut self, lines_bet: u32);

    fn clear_winnings(&mut self);

    fn aspect_ratio_changed(&mut self, aspect_ratio_index: usize);
}
