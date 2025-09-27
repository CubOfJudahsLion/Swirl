//! Module  :   Interactive Events
//! Purpose :   Implements mouse events (for now) and modifiers.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 04/13/2021 03:57 PM
//! Modified:   Thu 09/30/2021 06:59 PM


pub mod modifiers;
pub use modifiers::*;

pub mod pointer;
pub use pointer::*;

use super::EventLike;


/// An user-interface (i.e., interactive) event, mostly comprised of user actions.
pub trait UIEventLike: EventLike {
    /// Obtain the keyboard modifiers.
    fn modifiers(&self) -> &Modifiers;
}
