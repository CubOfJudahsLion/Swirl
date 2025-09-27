//! Module  :   Style
//! Purpose :   Style information and data structures for specifying drawing
//!             and text styles.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 04/27/2021 08:15 AM
//! Modified:   Wed 10/13/2021 04:34 PM


//use std::cmp::{Ordering, PartialOrd, Ord};
//use std::num::FpCategory;
//use std::string::ToString;
//
//use wasm_bindgen::{JsValue, JsCast};
//use js_sys::Array;
//use web_sys::{CanvasGradient, CanvasPattern, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
//
//use crate::make_enum_set;


pub mod color;
pub mod fonts;
pub mod shorten;

pub use color::*;
pub use fonts::*;


// /// In the HSL/HSLA models, hue is specified as an angle in
// /// one of several possible units.
// #[derive(Clone, Copy, PartialEq)]
// pub enum StyleAngle {
//     Deg(f32),
//     Rad(f32),
//     Grad(f32),
//     Turns(f32),
// }
// 
// impl ToString for StyleAngle {
//     fn to_string(&self) -> String {
//         use StyleAngle::*;
//         match self {
//             &Deg(d)     => format!("{}deg", d),
//             &Rad(r)     => format!("{}rad", r),
//             &Grad(g)    => format!("{}grad", g),
//             &Turns(t)   => format!("{}turn", t),
//         }
//     }
// }
// 
// /// Sources for a repeated pattern.
// #[derive(Clone)]
// pub enum PatternSource {
//     Image(HtmlImageElement),
//     Canvas(HtmlCanvasElement),
// }
// 
// //  Pattern repetition alternatives.
// make_enum_set! {
//     PatternRepetition ::=
//         Repeat      => "repeat",
//         RepeatX     => "repeat-x",
//         RepeatY     => "repeat-y",
//         NoRepeat    => "no-repeat",
// }
// 
// /// A pattern requires a source and a repetition mode.
// #[derive(Clone)]
// pub struct Pattern {
//     src     : PatternSource,
//     rep     : PatternRepetition,
//     cache   : Option<CanvasPattern>,
// }
// 
// impl Pattern {
//     #[inline(always)]
//     pub fn new(src: PatternSource, rep: PatternRepetition) -> Self {
//         Pattern {
//             src,
//             rep,
//             cache: None,
//         }
//     }
// 
//     #[inline(always)]
//     pub fn new_from_image(img: &HtmlImageElement, rep: PatternRepetition) -> Self {
//         Pattern {
//             src     : PatternSource::Image(img.clone()),
//             rep,
//             cache   : None,
//         }
//     }
// 
//     #[inline(always)]
//     pub fn new_from_canvas(cnv: &HtmlCanvasElement, rep: PatternRepetition) -> Self {
//         Pattern {
//             src     : PatternSource::Canvas(cnv.clone()),
//             rep,
//             cache   : None,
//         }
//     }
// 
//     #[inline(always)]
//     pub fn source(&self) -> &PatternSource {
//         &self.src
//     }
// 
//     pub fn set_source(&mut self, src: PatternSource) {
//         self.src    = src;
//         self.cache  = None;
//     }
// 
//     #[inline(always)]
//     pub fn repetitions(&self) -> &PatternRepetition {
//         &self.rep
//     }
// 
//     pub fn set_repetitions(&mut self, rep: PatternRepetition) {
//         if rep != self.rep {
//             self.rep    = rep;
//             self.cache  = None;
//         }
//     }
// 
//     fn to_canvas_pattern_no_cache(&self, ctx: &CanvasRenderingContext2d) -> Result<CanvasPattern, JsValue> {
//         use PatternSource::*;
//         match &self.src {
//             &Image(ref img)     =>
//                 ctx.create_pattern_with_html_image_element(
//                     img,
//                     self.rep.into()
//                 ),
//             &Canvas(ref cnv)    =>
//                 ctx.create_pattern_with_html_canvas_element(
//                     cnv,
//                     self.rep.into()
//                 ),
//         }
//             .map(|opt_pat| opt_pat.ok_or(JsValue::from_str("Pattern not created.")))
//             .and_then(|res_pat| res_pat)
//     }
// 
//     pub fn to_canvas_pattern(&self, ctx: &CanvasRenderingContext2d) -> Result<CanvasPattern, JsValue> {
//         match &self.cache {
//             &Some(ref pat)  => Ok(pat.clone()),
//             _               => self.to_canvas_pattern_no_cache(ctx),
//         }
//     }
// 
//     pub fn update_cache(&mut self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
//         if self.cache.is_none() {
//             self.cache = Some(self.to_canvas_pattern_no_cache(ctx)?);
//         }
//         Ok(())
//     }
// }
// 
// 
// make_enum_set! {
//     //  Gradient errors.
//     GradientError ::=
//         //  For now, the only possible fault is to trying to insert a
//         //  stop with a non-numeric position (i.e., infinite or NaN.)
//         NonNumericStop => "non-numeric stop (NaN or Infinite) attempted.",
// }
// 
// /// A color stop is an index of the relative position in the gradient's
// /// span in [0, 1] and the color to be set in that position.
// #[derive(Clone, Copy)]
// pub struct ColorStop(pub f32, pub Color);
// 
// //  The next four trait implementation give `ColorStop` a total ordering
// //  (converting NaN into the lesser possible alternative.) This is only
// //  to satisfy the trait -- we won't allow *NaN*, *INF* or *-INF* to be in
// //  the stop list.
// 
// impl PartialEq for ColorStop {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.0 == rhs.0
//     }
// }
// 
// impl Eq for ColorStop {}
// 
// impl PartialOrd for ColorStop {
//     #[inline(always)]
//     fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
//         self.0.partial_cmp(&rhs.0)
//     }
// }
// 
// impl Ord for ColorStop {
//     fn cmp(&self, rhs: &Self) -> Ordering {
//         match (self.0.classify(), rhs.0.classify()) {
//             // We make NaN lesser than everything else.
//             (FpCategory::Nan, FpCategory::Nan)  => Ordering::Equal,
//             (FpCategory::Nan, _)                => Ordering::Less,
//             (_,               FpCategory::Nan)  => Ordering::Greater,
//             _                                   => self.0.partial_cmp(&rhs.0).unwrap()
//         }
//     }
// }
// 
// /// Parameters for the creation of a gradient
// #[derive(Clone)]
// pub enum GradientParams {
//     Linear {
//         x0: f64,
//         y0: f64,
//         x1: f64,
//         y1: f64,
//     },
//     Radial {
//         x0: f64,
//         y0: f64,
//         r0: f64,
//         x1: f64,
//         y1: f64,
//         r1: f64,
//     },
// }
// 
// /// A gradient is merely a list of color stops.
// #[derive(Clone)]
// pub struct Gradient {
//     stops   : Vec<ColorStop>,
//     params  : GradientParams,
//     cache   : Option<CanvasGradient>,
// }
// 
// /// Two methods to locate a stop: by its position in the gradient (`f32`)
// /// or its index position (`usize`).
// #[derive(PartialEq, Clone, Copy)]
// pub enum StopLocation {
//     Position(f32),
//     Index(usize),
// }
// 
// impl Gradient {
//     /// Create a new color gradient. The parameters ensure there's at least
//     /// one color stop, i.e., that the gradient is at least a flat color.
//     /// Do note that creation can stil fail: if any of the stops is a NaN
//     /// or infinite, the function will return a `GradientError`.
//     pub fn new(first_stop: ColorStop, other_stops: Vec<ColorStop>, params: GradientParams) -> Result<Self, GradientError> {
//         let mut stops = other_stops;
//         stops.insert(0, first_stop);
//         if stops.iter().any(
//             |&ColorStop(n, _)| n.is_nan() || n.is_infinite()
//         )
//         {
//             Err(GradientError::NonNumericStop)
//         }
//         else {
//             stops.sort();
//             Ok(Gradient {stops, params, cache: None})
//         }
//     }
// 
//     /// Read the parameters.
//     #[inline(always)]
//     pub fn parameters(&self) -> &GradientParams {
//         &self.params
//     }
// 
//     /// Writeable parameters.
//     #[inline(always)]
//     pub fn parameters_mut(&mut self) -> &mut GradientParams {
//         &mut self.params
//     }
// 
//     /// Set the parameters.
//     pub fn set_parameters(&mut self, params: GradientParams) {
//         self.params = params;
//         self.cache  = None;
//     }
// 
//     /// Number of stops.
//     #[inline(always)]
//     pub fn stop_len(&self) -> usize { self.stops.len() }
// 
//     /// Read-only slice containing all stops.
//     #[inline(always)]
//     pub fn all_stops(&self) -> &[ColorStop] {
//         &self.stops
//     }
// 
//     /// Insert a new stop, provided another was not
//     /// occupying the same gradient position. Successful
//     /// insertion yields `true`. Finding a stop in that
//     /// position prevents insertion and yields `false`.
//     /// To modify an existing stop, see [`Self::get_stop_mut`]
//     /// below.
//     pub fn insert_stop(&mut self, stop: ColorStop) -> bool {
//         if stop.0.is_nan() || stop.0.is_infinite() {
//             false
//         }
//         else {
//             match self.stops.binary_search(&stop) {
//                 Ok(_)       =>
//                     false,
//                 Err(idx)    => {
//                     self.cache = None;
//                     self.stops.insert(idx, stop);
//                     true
//                 }
//             }
//         }
//     }
// 
//     //  Helper function to resolve `StopLocation` variants into a
//     //  usized-result. Just as [`Vec::binary_search_by`], `Ok` means
//     //  the location is found, and `Err` signals a possible insertion
//     //  position. However, an incorrect float in `StopLocation::Position`
//     //  will cause `Err(usize::MAX)` to be returned.
//     fn get_index(&self, pos: &StopLocation) -> Result<usize, usize> {
//         match pos {
//             &StopLocation::Position(stop_pos)   =>
//                 if stop_pos.is_infinite() || stop_pos.is_nan() {
//                     Err(usize::MAX) }
//                 else {
//                     self.stops.binary_search_by(
//                         |&ColorStop(n, _)| n.partial_cmp(&stop_pos).unwrap()
//                     )
//                 },
//             &StopLocation::Index(idx)           =>
//                 if idx < self.stops.len() {
//                     Ok(idx)
//                 }
//                 else {
//                     Err(idx)
//                 }
//         }
//     }
// 
//     /// Remove a `ColorStop`.
//     pub fn remove_stop(&mut self, pos: &StopLocation) -> bool {
//         match self.get_index(&pos) {
//             Ok(idx) => {
//                 self.cache = None;
//                 self.stops.remove(idx);
//                 true
//             },
//             _       =>
//                 false,
//         }
//     }
// 
//     /// Get a `ColorStop` (read-only.)
//     pub fn get_stop(&self, pos: &StopLocation) -> Option<&ColorStop> {
//         match self.get_index(&pos) {
//             Ok(idx) => Some(&self.stops[idx]),
//             _       => None,
//         }
//     }
// 
//     /// Get a `ColorStop` (for modification.)
//     pub fn get_stop_mut(&mut self, pos: &StopLocation) -> Option<&mut ColorStop> {
//         match self.get_index(&pos) {
//             Ok(idx) => {
//                 self.cache = None;
//                 Some(&mut self.stops[idx])
//             },
//             _       =>
//                 None,
//         }
//     }
// 
//     fn to_canvas_gradient_no_cache(&self, ctx: &CanvasRenderingContext2d) -> Result<CanvasGradient, JsValue> {
//         let grad = (match &self.params {
//             &GradientParams::Linear {x0, y0, x1, y1,}           =>
//                 Ok(ctx.create_linear_gradient(x0, y0, x1, y1)),
//             &GradientParams::Radial {x0, y0, r0, x1, y1, r1,}   =>
//                 ctx.create_radial_gradient(x0, y0, r0, x1, y1, r1),
//         })?;
//         for &ColorStop(pos, color) in self.stops.iter() {
//             grad.add_color_stop(pos, &color.to_string())
//                 .map_err(|_|
//                     JsValue::from_str(
//                         &format!(
//                             "{}::Gradient::to_canvas_gradient: malformed color stop.",
//                             module_path!()
//                         )
//                     )
//                 )?;
//         }
//         Ok(grad)
//     }
// 
//     /// Transform into a native representation
//     pub fn to_canvas_gradient(&self, ctx: &CanvasRenderingContext2d) -> Result<CanvasGradient, JsValue> {
//         match &self.cache {
//             &Some(ref grad) =>
//                 Ok(grad.clone()),
//             _               =>
//                 self.to_canvas_gradient_no_cache(ctx),
//         }
//     }
// 
//     /// Update the cache, if needed.
//     pub fn update_cache(&mut self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
//         if self.cache.is_none() {
//             self.cache = Some(self.to_canvas_gradient_no_cache(ctx)?);
//         }
//         Ok(())
//     }
// }
// 
// 
// /// Fill styles for text and shapes. One can choose not to fill/draw
// /// them (`Empty`) or to specify a color, a gradient, or a patternderive(Clone)]
// #[derive(Clone)]
// pub enum FillStyle {
//     Empty,
//     Flat(Color),
//     Grad(Gradient),
//     Pat(Pattern),
// }
// 
// impl FillStyle {
//     #[inline(always)]
//     pub fn new_color(col: Color) -> Self { FillStyle::Flat(col) }
// 
//     #[inline(always)]
//     pub fn new_grad(grad: Gradient) -> Self { FillStyle::Grad(grad) }
// 
//     #[inline(always)]
//     pub fn new_pat(pat: Pattern) -> Self { FillStyle::Pat(pat) }
// 
//     pub fn new_make_grad(first_stop: ColorStop, other_stops: Vec<ColorStop>, params: GradientParams) -> Result<Self, JsValue> {
//         Ok(FillStyle::Grad(
//             Gradient::new(first_stop, other_stops, params)
//                 .map_err(|err| JsValue::from_str(&("Gradient error: ".to_string() + &err.to_string())))?
//         ))
//     }
// 
//     #[inline(always)]
//     pub fn new_make_pat(src: PatternSource, rep: PatternRepetition) -> Self {
//         FillStyle::Pat(Pattern::new(src, rep))
//     }
// 
//     pub(crate) fn apply(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
//         use FillStyle::*;
//         ctx.set_fill_style(&match self {
//             &Empty          => JsValue::from_str(&Color::TRANSPARENT.to_string()),
//             &Flat(ref col)  => JsValue::from_str(&col.to_string()),
//             &Grad(ref grad) => grad.to_canvas_gradient(ctx)?.unchecked_into::<JsValue>(),
//             &Pat(ref pat)   => pat.to_canvas_pattern(ctx)?.unchecked_into::<JsValue>(),
//         });
//         Ok(())
//     }
// 
//     pub fn update_cache(&mut self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
//         match self {
//             &mut FillStyle::Grad(ref mut grad)  => grad.update_cache(ctx),
//             &mut FillStyle::Pat(ref mut pat)    => pat.update_cache(ctx),
//             _                                   => Ok(()),
//         }
//     }
// }
// 
// 
// /// A line width is just a float.
// pub type LineWidth = f64;
// 
// //  Line caps: the shape used when the line ends.
// make_enum_set! {
//     LineCap ::=
//         Butt    => "butt",
//         Round   => "round",
//         Square  => "square",
// }
// 
// /// Describes what to do when two lines meet.
// #[derive(Clone, Copy, PartialEq)]
// pub enum LineJoin {
//     /// Draws a bevel
//     Bevel,
//     /// Rounds the corner
//     Round,
//     /// Attempts to extend line edges until they meet,
//     /// down to a specified limit distance.
//     Miter(f64),
// }
// 
// impl From<LineJoin> for &'static str {
//     fn from(lj: LineJoin) -> Self {
//         use LineJoin::*;
//         match lj {
//             Bevel       => "bevel",
//             Round       => "round",
//             Miter(_)    => "miter",
//         }
//     }
// }
// 
// #[derive(Clone)]
// pub struct LineDash(f64, JsValue);
// 
// impl LineDash {
//     pub fn new_none() -> Self {
//         LineDash(0.0, Array::new().unchecked_into::<JsValue>())
//     }
// 
//     pub fn new(first: u16, rest: &[u16], ofs: Option<f64>) -> Self {
//         let ofs   = ofs.unwrap_or(0.0);
//         let arr   = Array::new_with_length((rest.len() + 1) as u32);
//         arr.set(0, JsValue::from_f64(first as f64));
//         for i in 0..rest.len() {
//             arr.set((i + 1) as u32, JsValue::from_f64(rest[i] as f64));
//         }
//         LineDash(ofs, arr.unchecked_into::<JsValue>())
//     }
// }
// 
// 
// /// Line style, including the line width, style, caps, join and dash style.
// #[derive(Clone)]
// pub struct StrokeStyle {
//     pub width   : f64,
//     pub style   : FillStyle,
//     pub cap     : LineCap,
//     pub join    : LineJoin,
//     pub dash    : LineDash,
// }
// 
// impl StrokeStyle {
//     #[allow(dead_code)]
//     pub fn new_default() -> Self {
//         StrokeStyle {
//             width   : 1.0,
//             style   : FillStyle::Flat(Color::Named(NamedColor::Black)),
//             cap     : LineCap::Butt,
//             join    : LineJoin::Miter(10.0),
//             dash    : LineDash::new_none(),
//         }
//     }
// 
//     #[allow(dead_code)]
//     pub fn new_none() -> Self {
//         StrokeStyle {
//             width   : 0.0,
//             style   : FillStyle::Empty,
//             cap     : LineCap::Butt,
//             join    : LineJoin::Miter(10.0),
//             dash    : LineDash::new_none(),
//         }
//     }
// 
//     #[allow(dead_code)]
//     pub fn new(
//         width   : Option<f64>,
//         style   : Option<FillStyle>,
//         cap     : Option<LineCap>,
//         join    : Option<LineJoin>,
//         dash    : Option<LineDash>
//     ) -> Self {
//         StrokeStyle {
//             width   : width.unwrap_or(1.0),
//             style   : style.unwrap_or(FillStyle::Flat(Color::Named(NamedColor::Black))),
//             cap     : cap.unwrap_or(LineCap::Butt),
//             join    : join.unwrap_or(LineJoin::Miter(10.0)),
//             dash    : dash.unwrap_or(LineDash::new_none()),
//         }
//     }
// 
//     pub fn update_cache(&mut self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
//         self.style.update_cache(ctx)
//     }
// 
//     pub fn apply(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
//         ctx.set_line_width(self.width);
//         self.style.apply(ctx)?;
//         ctx.set_line_cap(self.cap.into());
//         ctx.set_line_join(self.join.into());
//         if let &LineJoin::Miter(miter_limit) = &self.join {
//             ctx.set_miter_limit(miter_limit);
//         }
//         ctx.set_line_dash_offset(self.dash.0);
//         ctx.set_line_dash(&self.dash.1)?;
//         Ok(())
//     }
// }
