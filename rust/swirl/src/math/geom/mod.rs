//! Module  :   Basic Geometric entities.
//! Purpose :   Implements points, rectangles, matrices and any other mathematical
//!             objects required for the library.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu Jul  8 11:40:58 AM CST 2021
//! Modified:   Mon 04/11/2022 04:19 PM


pub mod point;
pub use point::*;

pub mod line;
pub use line::*;

pub mod dimensions;
pub use dimensions::*;

pub mod rect;
pub use rect::*;

pub mod transform;
pub use transform::*;

pub mod polygon;
pub use polygon::*;
