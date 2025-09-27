//! Module  :   GEL Game Event Handler
//! Purpose :   Implements GEL's outward-facing interface
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu 02/17/2022 12:53 PM
//! Modified:   Mon 07/04/2022 05:50 PM


use std::rc::Rc;
use std::fmt::{self, Debug, Formatter};
use wasm_bindgen::prelude::*;
use crate::math::numeric::Numeric;


/// The change action is implemented in the **TypeScript** side
/// as a union of `+`, `-` and a `number`. We cannot restrict
/// that type directly, so we'll take it as-is.
pub type JsChangeAction = JsValue;


#[wasm_bindgen]
extern "C" {
    /// This type is notified of interactive events, such as
    /// user clicks or when a spin ends. The **GEL** driver then
    /// issues commands through the `Game` interface, which
    /// of course every game should implement.
    ///
    /// Note that there is no constructor: GEL passes a
    /// unique instance to the game, to use throughout.
    pub type GameEventHandler;


    #[wasm_bindgen(method, js_name = onAutoPlayClicked)]
    pub fn on_autoplay_clicked(this: &GameEventHandler);

    #[wasm_bindgen(method, js_name = onAutoPlayConfigSelected)]
    pub fn on_autoplay_config_selected(this: &GameEventHandler);

    #[wasm_bindgen(method, js_name = onAutoPlayOptionSelected)]
    pub fn on_autoplay_option_selected(this: &GameEventHandler, value: f64);

    /*
     * Event raised when the player clicks the BETMAX button.
     */
    #[wasm_bindgen(method, js_name = onBetMaxClicked)]
    pub fn on_bet_max_clicked(this: &GameEventHandler);

    /*
     * Event raised when the player clicks the coin-value button
     * @param to either '+', '-' or a specific number indicating if it is the #1 coin, the #2 coin, etc. (NOT the coin value)
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    #[wasm_bindgen(method, js_name = onCoinValueClicked)]
    pub fn on_coin_value_clicked(this: &GameEventHandler, to: JsChangeAction, circular: Option<bool>);

    /*
     * Event raised when the player clicks the coins-per-line button
     * @param to either '+', '-' or a specific number indicating the number of coins per line
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    #[wasm_bindgen(method, js_name = onLineCoinsClicked)]
    pub fn on_line_coins_clicked(this: &GameEventHandler, to: JsChangeAction, circular: Option<bool>);

    /*
     * Event raised when the player clicks the lines-bet button or a selector
     * @param to either '+', '-' or a specific number indicating the number of lines
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    #[wasm_bindgen(method, js_name = onLinesBetClicked)]
    pub fn on_lines_bet_clicked(this: &GameEventHandler, to: JsChangeAction, circular: Option<bool>);

    #[wasm_bindgen(method, js_name = onPayoutTableClicked)]
    pub fn on_payout_table_clicked(this: &GameEventHandler);

    /*
     * Event raised when the player clicks the SPIN button.
     */
    #[wasm_bindgen(method, js_name = onSpinClicked)]
    pub fn on_spin_clicked(this: &GameEventHandler);

    /*
     * Event raised when the player clicks the STOP button
     */
    #[wasm_bindgen(method, js_name = onStopClicked)]
    pub fn on_stop_clicked(this: &GameEventHandler);

    /*
     * Event raised when the player clicks the TotalBet button
     * @param to either '+', '-' or a specific number indicating the number of coins per line
     * @param circular optional parameter, default is false. Only pass true if you have one single button (as opposed to two separate [-] and [+] buttons)
     */
    #[wasm_bindgen(method, js_name = onTotalBetClicked)]
    pub fn on_total_bet_clicked(this: &GameEventHandler, to: JsChangeAction, circular: Option<bool>);
}


/// A better type for `ChangeAction` in the Rust side.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChangeAction<N>  {
    Increase,
    Decrease,
    SetTo(N),
}

impl<N> From<ChangeAction<N>> for JsValue
    where   N       : Numeric,
            JsValue : From<N>
{
    fn from(chga: ChangeAction<N>) -> Self {
        match chga {
            ChangeAction::Increase  => JsValue::from_str("+"),
            ChangeAction::Decrease  => JsValue::from_str("-"),
            ChangeAction::SetTo(n)  => JsValue::from(n),
        }
    }
}


/// A higher-level wrapper for the `GameEventHandler`
#[derive(Clone)]
pub struct GameHandler(Rc<GameEventHandler>);

impl Debug for GameHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[GameHandler{GameEventHandler}]")
    }
}

impl GameHandler {
    #[inline(always)]
    pub fn new(geh: GameEventHandler) -> Self {
        GameHandler(Rc::new(geh))
    }

    #[inline(always)]
    pub fn on_autoplay_clicked(&self) {
        self.0.on_autoplay_clicked();
    }

    #[inline(always)]
    pub fn on_autoplay_config_selected(&self) {
        self.0.on_autoplay_config_selected();
    }

    #[inline(always)]
    pub fn on_autoplay_option_selected(&self, value: f64) {
        self.0.on_autoplay_option_selected(value);
    }

    #[inline(always)]
    pub fn on_bet_max_clicked(&self) {
        self.0.on_bet_max_clicked();
    }

    #[inline]
    pub fn on_coin_value_clicked(&self, to: ChangeAction<f64>, circular: Option<bool>) {
        self.0.on_coin_value_clicked(to.into(), circular)
    }

    #[inline]
    pub fn on_line_coins_clicked(&self, to: ChangeAction<u32>, circular: Option<bool>) {
        self.0.on_line_coins_clicked(to.into(), circular)
    }

    #[inline]
    pub fn on_lines_bet_clicked(&self, to: ChangeAction<u32>, circular: Option<bool>) {
        self.0.on_lines_bet_clicked(to.into(), circular)
    }

    #[inline(always)]
    pub fn on_payout_table_clicked(&self) {
        self.0.on_payout_table_clicked();
    }

    #[inline(always)]
    pub fn on_spin_clicked(&self) {
        self.0.on_spin_clicked()
    }

    #[inline(always)]
    pub fn on_stop_clicked(&self) {
        self.0.on_stop_clicked()
    }

    #[inline]
    pub fn on_total_bet_clicked(&self, to: ChangeAction<f64>, circular: Option<bool>) {
        self.0.on_total_bet_clicked(to.into(), circular)
    }
}
