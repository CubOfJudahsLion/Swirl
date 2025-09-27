//! Module  :   Spacing
//! Purpose :   Allows denotation of different
//!             spacing options between elements
//!             in a multiple-object layout.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Sun 24 Apr 2022 09:47:06 AM CST
//! Modified:   ~

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Spacing {
    NoSpacing,
    SpacingBetween(i16),
    AvailableEqualSpacingBetween,
    CenterSpacing(i16),
    AvailableEqualCenterSpacing,
}

impl Default for Spacing {
    fn default() -> Self { Spacing::AvailableEqualCenterSpacing }
}
