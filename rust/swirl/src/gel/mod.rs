//! Module  :   GEL interface.
//! Purpose :   Implements the interface for MultiSlot's
//!             GEL game-driver protocol.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu 02/17/2022 12:48 PM
//! Modified:   Fri 02/18/2022 09:27 AM


pub mod game_event_handler;
pub mod conversion_utils;
pub mod game_bridge;
pub mod swirl_game;

pub use game_event_handler::*;
pub use conversion_utils::*;
pub use game_bridge::*;
pub use swirl_game::*;
