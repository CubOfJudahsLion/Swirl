//! # Module        :   _Alignment_
//! - **Purpose**   :   Allows one to specify alignment
//!                     (vertical and/or horizontal) for
//!                     elements of a layout.
//! - **Version**   :   0.1.1
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Sun 24 Apr 2022 09:45:09 AM CST
//! - **Modified**  :   Mon 09 May 2022 09:54:39 PM CST


/// Describes all classes that are alignment types.
pub trait AlignmentType: Default { }


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl Default for VerticalAlign {
    fn default() -> Self { VerticalAlign::Middle }
}

impl AlignmentType for VerticalAlign {}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlign {
    fn default() -> Self { HorizontalAlign::Center }
}

impl AlignmentType for HorizontalAlign {}
