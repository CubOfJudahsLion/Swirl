//! Module  :   Modifiers
//! Purpose :   Key and mouse event modifiers.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu Feb 25 03:21:53 PM CST 2021
//! Modified:   Wed 04/14/2021 02:29 PM


use std::fmt::{self, Display, Formatter};
use web_sys::{self, TouchEvent};


/// Keyboard modifiers. While not expected to receive much use
/// for games, in-house tools might take advantage of them.
#[derive(Clone, Copy, Debug)]
pub struct Modifiers {
    pub alt     : bool,
    pub ctrl    : bool,
    pub shift   : bool,
    pub meta    : bool,
}

impl Modifiers {
    /// Created directly by the user.
    pub fn new(alt: bool, ctrl: bool, shift: bool, meta: bool) -> Self {
        Modifiers {alt, ctrl, shift, meta,}
    }

    /// Adapt directly from a browser's MouseEvent.
    pub(crate) fn from_mouse_event_modifiers(browser_ev: &web_sys::MouseEvent) -> Self {
        Modifiers {
            alt     : browser_ev.alt_key(),
            ctrl    : browser_ev.ctrl_key(),
            shift   : browser_ev.shift_key(),
            meta    : browser_ev.meta_key(),
        }
    }

    /// Adapt directly from TouchEvent.
    pub(crate) fn from_touch_event_modifiers(browser_ev: &TouchEvent) -> Self {
        Modifiers {
            alt     : browser_ev.alt_key(),
            ctrl    : browser_ev.ctrl_key(),
            shift   : browser_ev.shift_key(),
            meta    : browser_ev.meta_key(),
        }
    }
}

impl Display for Modifiers {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Modifiers{{alt: {}, ctrl: {}, shift: {}}}", self.alt, self.ctrl, self.shift)
    }
}
