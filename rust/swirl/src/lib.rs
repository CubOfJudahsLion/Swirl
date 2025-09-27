//! # Module        :   _SWIRL_ (Scene-graph Web-targeting Independent Rendering Library), _main module_
//! - **Purpose**   :   A fast, minimal graphics library for Canvas-based apps.
//!                     The emphasis of SWIRL is simplicity and speeds. It implements
//!                     a strictly sufficient set of primitives and operations to
//!                     deliver content directly through the browser's Canvas API
//!                     for Casino games and bonus rounds.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   2021-02-22 14:51
//! - **Modified**  :   Thu 02/17/2022 02:06 PM


//  The base library just acknowledges and brings into scope the
//  several submodules. Users can take advantage of the flattened
//  structure if they so wish.

pub mod base;
pub mod math;
pub mod utils;
pub mod events;
pub mod loader;
pub mod style;
pub mod scene_graph;
pub mod drawables;
pub mod rendering;
pub mod sound;
pub mod gel;
//pub mod app;

pub use base::*;
pub use math::*;
pub use utils::*;
pub use events::*;
pub use loader::*;
pub use style::{*, color::*, fonts::*};
pub use scene_graph::*;
pub use drawables::*;
pub use rendering::*;
pub use sound::*;
pub use gel::*;
//pub use app::*;


//  Several operations in this library might use a (common)
//  canvas for intermediate operations.  It's better than
//  having allocate one each -- and it's feasible: if repeated
//  calls to `get_context` are made, as long as it's the same
//  context type, they'll obtain the same context.
pub(crate) const COMMON_CANVAS_NAME: &str = "____common_SWIRL_canvas____";


#[cfg(test)]
pub mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
