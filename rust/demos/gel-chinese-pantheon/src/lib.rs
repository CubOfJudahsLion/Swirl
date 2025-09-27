//! # Module        :   _Lib (Main)_
//! - **Purpose**   :   Module structure and location of main function.
//!                     This version implements the GEL protocol through
//!                     SWIRL::gel.
//! - **Version**   :   0.4.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   2021-02-09 14:53
//! - **Modified**  :   Fri 20 May 2022 11:40:44 AM CST


use wasm_bindgen::prelude::*;
use web_sys::console;

mod assets;
mod reels;
mod game_builder;
mod game;
use game::Game;


#[wasm_bindgen]
pub fn init() {
    console::info_1(&JsValue::from_str(
         "Monkey Khan Slots v0.4.0\n\
          [SWIRL+GEL|wasm32-unknown-unknown]\n\
          \"Monkey Khan\", \"SWIRL\" and \"GEL\" ©2021-22 Multislot Inc.\n\
          http://multislot.com"
    ));
    let game = Game::new();
    swirl::set_callbacks(game);
}
