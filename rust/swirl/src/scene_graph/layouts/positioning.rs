
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
//
//
// #[derive(Clone, Debug)]
// pub struct LayoutPositioning {
//     padding : Padding,
//     v_align : VerticalAlign,
//     h_align : HorizontalAlign,
//     spacing : Spacing,
// }
// 
// impl Default for LayoutPositioning {
//     fn default() -> Self {
//         LayoutPositioning {
//             padding     : Padding::default(),
//             v_align     : VerticalAlign::default(),
//             h_align     : HorizontalAlign::default(),
//             spacing     : Spacing::default(),
//         }
//     }
// }
// 
// 
// pub struct LayoutPositioningBuilder(LayoutPositioning);
// 
// macro_rules! builder_method {
//     ($member:ident : $type:ty) => {
//         paste! {
//             pub fn [<with_ $member>](self, $member: $type) -> Self {
//                 self.0.$member = $member;
//                 self
//             }
//         }
//     }
// }
// 
// impl LayoutPositioningBuilder {
//     pub fn new() -> Self { LayoutPositioningBuilder(LayoutPositioning::default()) }
// 
//     builder_method!{padding     : Padding}
//     builder_method!{v_align     : VerticalAlign}
//     builder_method!{h_align     : HorizontalAlign}
//     builder_method!{spacing     : Spacing}
// 
//     pub fn build(self) -> LayoutPositioning { self.0 }
// }
