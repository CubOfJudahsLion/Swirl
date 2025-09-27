//! Module  :   Base Traits
//! Purpose :   A set of base characteristics/feature sets that
//!             all other objects in SWIRL build upon.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   2021-02-22 14:51
//! Modified:   Tue 04/26/2022 11:13 AM


use std::default::Default;
use std::fmt::Debug;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::bounded::Bounded;
    

/// These are the supported Porter-Duff blending modes
/// supported by SWIRL. The range is currently limited
/// as future compatibility options are weighed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Erase,
    Lighten,
    Screen,
    Add,
    Overlay,
    Darken,
    Multiply,
    Burn,
}

impl From<BlendMode> for &'static str {
    fn from(mode: BlendMode) -> &'static str {
        use BlendMode::*;
        match mode {
            Normal      => "source-over",
            Erase       => "destination-out",
            Lighten     => "lighten",
            Screen      => "screen",
            Add         => "lighter",
            Overlay     => "overlay",
            Darken      => "darken",
            Multiply    => "multiply",
            Burn        => "color-burn",
        }
    }
}

impl Default for BlendMode {
    #[inline(always)]
    fn default() -> Self { BlendMode::Normal }
}


/// Drawable elements only have to know one thing:
/// how to draw themselves into a `CanvasRenderingContext2d`
/// given a world transform. Note that this world transform
/// *has been applied* before the call; it's only provided
/// for the drawable to combine with its own local transform.
pub trait Drawable: Bounded<i16> + Debug {
    //fn blending_mode(&self) -> BlendMode;
    //fn set_blending_mode(&mut self, mode: BlendMode);

    #[inline(always)]
    fn drawable(&self) -> bool { true }

    #[inline(always)]
    fn set_drawable(&mut self, _drawable: bool) {}

    #[inline(always)]
    fn alpha(&self) -> f64 { 1. }

    #[inline(always)]
    fn set_alpha(&mut self, _alpha: f64) {}

    #[inline(always)]
    fn blend_mode(&self) -> BlendMode { BlendMode::default() }

    #[inline(always)]
    fn set_blend_mode(&mut self, _blend_mode: BlendMode) {}

    fn draw(
        &self,
        context : &CanvasRenderingContext2d
    ) -> Result<(), JsValue>;
}
