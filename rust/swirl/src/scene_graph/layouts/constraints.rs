//! # Module        :   _Constraints_
//! - **Purpose**   :   Declares constraints (horizontal
//!                     and vertical) to delimit space
//!                     consumption of grouped items.
//! - **Version**   :   0.1.1
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Mon 04/11/2022 08:31 PM
//! - **Modified**  :   Tue 10 May 2022 10:49:33 AM CST


use paste::paste;


/// A trait embodying constraints for any dimensions.
pub trait ConstraintType: Default {}


/// A height constraint can be null, told to use the minimum
/// possible height, maximum available height, or a height
/// relative to that of other height-relative objects.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeightConstraints {
    MinHeight,
    MaxHeight,
    MinPixelHeight(i16),
    VerticalParts(i16),
}

impl Default for HeightConstraints {
    fn default() -> Self { HeightConstraints::MinHeight }
}

impl ConstraintType for HeightConstraints {}


/// A width constraint can be null, told to use the minimum
/// possible width, maximum available width, or a width
/// relative to that of other height-relative objects.
#[derive(Clone, Copy, PartialEq, Eq, Debug,)]
pub enum WidthConstraints {
    MinWidth,
    MaxWidth,
    MinPixelWidth(i16),
    HorizontalParts(i16),
}

impl Default for WidthConstraints {
    fn default() -> Self { WidthConstraints::MinWidth }
}

impl ConstraintType for WidthConstraints {}


/// Groups height and width constraints.
#[derive(Clone, PartialEq, Debug)]
pub struct LayoutConstraints {
    pub height_constr : HeightConstraints,
    pub width_constr  : WidthConstraints,
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        LayoutConstraints {
            height_constr   : HeightConstraints::default(),
            width_constr    : WidthConstraints::default(),
        }
    }
}


#[derive(Clone, Debug)]
pub struct LayoutConstraintsBuilder(LayoutConstraints);

macro_rules! builder_method {
    ($member:ident : $type:ty) => {
        paste! {
            pub fn [<with_ $member>](mut self, $member: $type) -> Self {
                self.0.$member = $member;
                self
            }
        }
    }
}

impl LayoutConstraintsBuilder {
    pub fn new() -> Self { LayoutConstraintsBuilder(LayoutConstraints::default()) }

    builder_method!{height_constr   : HeightConstraints}
    builder_method!{width_constr    : WidthConstraints}

    pub fn build(self) -> LayoutConstraints { self.0 }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn defaults() {
        let mut layout = LayoutConstraints::default();
        assert_eq!(layout.height_constr, HeightConstraints::default());
        assert_eq!(layout.width_constr, WidthConstraints::default());
        layout = LayoutConstraintsBuilder::new().build();
        assert_eq!(layout.height_constr, HeightConstraints::default());
        assert_eq!(layout.width_constr, WidthConstraints::default());
    }

    #[wasm_bindgen_test]
    fn constraints_builder() {
        let layout: LayoutConstraints = LayoutConstraintsBuilder::new()
            .with_height_constr(HeightConstraints::VerticalParts(3))
            .with_width_constr(WidthConstraints::MinWidth)
            .build();
        assert_eq!(
            layout,
            LayoutConstraints {
                height_constr: HeightConstraints::VerticalParts(3),
                width_constr: WidthConstraints::MinWidth,
            }
        );
    }
}
