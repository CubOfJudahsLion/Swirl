//! Module  :   Fonts
//! Purpose :   Style information pertaining exclusively to text rendition.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Wed 04/28/2021 04:33 PM
//! Modified:   Thu 10/14/2021 01:37 PM


use std::cell::{RefCell, Cell};
use std::fmt::Debug;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Array;
use web_sys::CanvasRenderingContext2d;

use crate::make_enum_set;
use super::*;

use paste::paste;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FontItalics {
    NonItalic,
    Italic,
}

impl FontItalics {
    pub const DEFAULT: FontItalics = FontItalics::NonItalic;
}

impl From<FontItalics> for &'static str {
    fn from(italics: FontItalics) -> &'static str {
        use FontItalics::*;
        match italics {
            NonItalic   => "normal",
            Italic      => "italic",
        }
    }
}

impl ToString for FontItalics {
    fn to_string(&self) -> String {
        <&str as From<FontItalics>>::from(*self).to_string()
    }
}


//  Small caps variant. Either present or not.
make_enum_set! {
    FontVariant ::=
        NormalLowercase => "normal",
        SmallCaps       => "small-caps",
}

impl FontVariant {
    pub const DEFAULT: FontVariant = FontVariant::NormalLowercase;
}


//  Font weights are either predefined or a
//  numeric set (equivalents) from `100` to `900` (e.g.,
//  `400` is the same as `"normal"`, `700` the same as `"bold"`.)
//  While newer specifications accept a number from
//  1 to 1000 inclusive, these are transformed back
//  into any of the whole-hundreds.
make_enum_set! {
    FontWeight ::=
        NormalWeight    => "normal",
        Bold            => "bold",
}

impl FontWeight {
    pub const DEFAULT: FontWeight = FontWeight::NormalWeight;
}


/// Font or line size specifiers using relative or absolute units.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FontSizeUnit {
    /// Widths of "0" glyph
    Ch,
    /// EMs, or relative to the specified font size (e.g.,
    /// in a 16 point font, 1 em = 16 pt. Name comes from
    /// the width of the campital "M" as it used to be the
    /// same as the point size.
    Em,
    /// "X-height", o heights of the "x" (lowercase) glyph
    Ex,
    /// EMs of the font in the root element
    Rem,
    /// One VH is equivalent to 1% of viewport height
    Vh,
    /// One VW is equivalent to 1% of viewport width
    Vw,
    /// Smallest of Vh or Vw
    Vmin,
    /// Greatest of Vh or Vw
    Vmax,
    /// Pixels
    Px,
    /// Centimeters
    Cm,
    /// Millimiters
    Mm,
    /// Inches
    In,
    /// Picas
    Pc,
    /// Points
    Pt,
}

impl From<FontSizeUnit> for &'static str {
    fn from(unit: FontSizeUnit) -> &'static str {
        use FontSizeUnit::*;
        match unit {
            Ch      => "ch",
            Em      => "em",
            Ex      => "ex",
            Rem     => "rem",
            Vh      => "vh",
            Vw      => "vw",
            Vmin    => "vmin",
            Vmax    => "vmax",
            Px      => "px",
            Cm      => "cm",
            Mm      => "mm",
            In      => "in",
            Pc      => "pc",
            Pt      => "pt",
        }
    }
}

impl ToString for FontSizeUnit {
    #[inline]
    fn to_string(&self) -> String {
        <&str as From<FontSizeUnit>>::from(*self).to_string()
    }
}

/// An enum containing pre-established, relative,
/// unit relative and unit absolute font sizes.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FontSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
    Larger,
    Smaller,
    FontSizeAsUnits(f32, FontSizeUnit),
    FontSizeAsPercent(f32),
}

impl FontSize {
    pub const DEFAULT: FontSize = FontSize::FontSizeAsUnits(10., FontSizeUnit::Pt);
}

impl ToString for FontSize {
    fn to_string(&self) -> String {
        use FontSize::*;
        match self {
            &XXSmall                    => "xx-small".to_string(),
            &XSmall                     => "x-small".to_string(),
            &Small                      => "small".to_string(),
            &Medium                     => "medium".to_string(),
            &Large                      => "large".to_string(),
            &XLarge                     => "x-large".to_string(),
            &XXLarge                    => "xx-large".to_string(),
            &XXXLarge                   => "xxx-large".to_string(),
            &Larger                     => "larger".to_string(),
            &Smaller                    => "smaller".to_string(),
            &FontSizeAsUnits(amt, unit) => format!("{}{}", amt, <&str as From<FontSizeUnit>>::from(unit)),
            &FontSizeAsPercent(pct)     => format!("{}%", pct),
        }
    }
}


/// Groups both generic and user-specified fonts.
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FontFamily {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
    NamedFamily(String),
}

impl FontFamily {
    pub const DEFAULT: FontFamily = FontFamily::SansSerif;
}

impl ToString for FontFamily {
    fn to_string(&self) -> String {
        use FontFamily::*;
        match self {
            &Serif                  => "serif".to_string(),
            &SansSerif              => "sans-serif".to_string(),
            &Monospace              => "monospace".to_string(),
            &Cursive                => "cursive".to_string(),
            &Fantasy                => "fantasy".to_string(),
            &NamedFamily(ref fam)   => format!("\"{}\"", fam),
        }
    }
}


//  Text alignment respective to the drawing coordinate.
make_enum_set! {
    TextAlign ::= 
        AlignLeft   => "left",
        AlignRight  => "right",
        AlignCenter => "center",
}

impl TextAlign {
    pub const DEFAULT: TextAlign = TextAlign::AlignLeft;
}


make_enum_set! {
    TextBaseline ::=
        TopBaseline         => "top",
        HangingBaseline     => "hanging",
        MiddleBaseline      => "middle",
        AlphabeticBaseline  => "alphabetic",
        IdeographicBaseline => "ideographic",
        BottomBaseline      => "bottom",
}

impl TextBaseline {
    pub const DEFAULT: TextBaseline = TextBaseline::AlphabeticBaseline;
}


//  These cache types may be hard to read, so here's a quick explanation.
//
//  As the cache is meant update behind the scenes even when the text and
//  its style are read-only in a renderer's queue, all wrappers are cell-
//  types -- and for maximum shareability, they are independent cells as
//  as to minimize borrowing conflicts.
//
//  Inside the cells, the caching value is an option -- it either caches
//  something or not. Sometimes the value inside is an option itself,
//  when the corresponding `FontStyle` field is also optional.
//
//  Finally, in the case of `RefCell<_>` cache fields (so chosen since
//  their inner elements are not `Copy`), the innermost type is
//  wrapped inside a `Rc`. These values are are costly to clone, so
//  the functions that produce them as results can return them as as
//  `'static` smart pointers instead.
#[derive(Clone, Debug)]
struct FontStyleCache {
    style       : RefCell<Option<Rc<String>>>,
    align       : RefCell<Option<Rc<String>>>,
    baseline    : RefCell<Option<Rc<String>>>,
    fill        : RefCell<Option<Option<Rc<JsValue>>>>,
    stroke_color: RefCell<Option<Option<Rc<JsValue>>>>,
    stroke_width: Cell<Option<Option<f64>>>,
    empty_array : JsValue,
}

macro_rules! cache_clearers {
    ($field:ident : RefCell) => {
        paste!{
            fn [< clear_ $field >] (&self) -> &Self {
                self.$field.replace(None);
                self
            }
        }
    };
    ($field:ident : Cell) => {
        paste!{
            fn [<clear_ $field>] (&self) -> &Self {
                self.$field.set(None);
                self
            }
        }
    };
}


//  A font format info cache prevents us from having to parse all of the font
//  info every time it's drawn. Cache info is invalidated automatically
//  whenever the corresponding fields of the `FontStyle` object (see below)
//  are updated. Updates to the cache are to be manual.
impl FontStyleCache {
    fn new() -> Self {
        FontStyleCache {
            style       : RefCell::new(None),
            align       : RefCell::new(None),
            baseline    : RefCell::new(None),
            fill        : RefCell::new(None),
            stroke_color: RefCell::new(None),
            stroke_width: Cell::new(None),
            empty_array : Array::new().unchecked_into::<JsValue>(),
        }
    }

    cache_clearers!{style       : RefCell}
    cache_clearers!{align       : RefCell}
    cache_clearers!{baseline    : RefCell}
    cache_clearers!{fill        : RefCell}
    cache_clearers!{stroke_color: RefCell}
    cache_clearers!{stroke_width: Cell   }
}


/// A `FontStyle` reunities all of the above `Font*` properties into
/// a single `struct`.
#[derive(Clone, Debug)]
pub struct FontStyle {
    italics     : FontItalics,
    variant     : FontVariant,
    weight      : FontWeight,
    size        : FontSize,
    family      : FontFamily,
    align       : TextAlign,
    baseline    : TextBaseline,
    fill        : Option<Color>,
    stroke      : Option<(u8, Color)>,
    cached      : FontStyleCache,
}


macro_rules! builder_with {
    ($field:ident : $type:ty) => {
        paste!{
            pub fn [<with_ $field>] (mut self, field_val: $type) -> Self {
                self.0.$field = field_val;
                self
            }
        }
    };
}

#[derive(Clone, Debug)]
pub struct FontStyleBuilder(FontStyle);

impl FontStyleBuilder {
    pub fn new() -> Self {
        FontStyleBuilder(FontStyle::new())
    }

    builder_with!{italics       : FontItalics}
    builder_with!{variant       : FontVariant}
    builder_with!{weight        : FontWeight}
    builder_with!{size          : FontSize}
    builder_with!{family        : FontFamily}
    builder_with!{align         : TextAlign}
    builder_with!{baseline      : TextBaseline}
    builder_with!{fill          : Option<Color>}
    builder_with!{stroke        : Option<(u8, Color)>}

    #[inline(always)]
    pub fn build(self) -> FontStyle {
        //  Update caches.
        self.0.update_caches();
        self.0
    }
}


macro_rules! font_style_getters_setters {
    (@clear_cache $base:ident.italics) => {
        $base.cached.clear_style()
    };
    (@clear_cache $base:ident.variant) => {
        $base.cached.clear_style()
    };
    (@clear_cache $base:ident.weight) => {
        $base.cached.clear_style()
    };
    (@clear_cache $base:ident.stretch) => {
        $base.cached.clear_style()
    };
    (@clear_cache $base:ident.size) => {
        $base.cached.clear_style()
    };
    (@clear_cache $base:ident.line_height) => {
        $base.cached.clear_style()
    };
    (@clear_cache $base:ident.family) => {
        $base.cached.clear_style()
    };
    (@clear_cache $base:ident.stroke) => {
        $base.cached
            .clear_stroke_color()
            .clear_stroke_width()
    };
    (@clear_cache $base:ident.$field:ident) => {
        paste!{
            $base.cached.[<clear_ $field>]()
        }
    };
    ($($field:ident : $type:ty),+ $(,)?) => {
        $(
            #[inline(always)]
            pub fn $field(&self) -> & $type {
                &self.$field
            }
            paste!{
                pub fn [<set_ $field>] (&mut self, new_value: $type) -> &mut Self {
                    font_style_getters_setters!(@clear_cache self.$field);
                    self.$field = new_value;
                    self
                }
            }
        )+
    };
}


impl FontStyle {
    /// Creates a font spec from the minimum required descriptors. To
    /// fill in more of the font specification, use the functions below.
    pub fn new() -> Self {
        FontStyle {
            italics     : FontItalics::DEFAULT,
            variant     : FontVariant::DEFAULT,
            weight      : FontWeight::DEFAULT,
            size        : FontSize::DEFAULT,
            family      : FontFamily::DEFAULT,
            align       : TextAlign::DEFAULT,
            baseline    : TextBaseline::DEFAULT,
            fill        : Some(Color::Named(NamedColor::Black)),
            stroke      : None,
            cached      : FontStyleCache::new(),
        }
    }

    font_style_getters_setters!{
        italics     : FontItalics,
        variant     : FontVariant,
        weight      : FontWeight,
        size        : FontSize,
        family      : FontFamily,
        align       : TextAlign,
        baseline    : TextBaseline,
        fill        : Option<Color>,
        stroke      : Option<(u8, Color)>,
    }

    pub(crate) fn generate_style_string(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.italics.to_string(),
            <FontVariant as Into<&str>>::into(self.variant),
            self.weight.to_string(),
            self.size.to_string(),
            self.family.to_string()
        )
    }

    /// Produces an updated style string.
    pub(crate) fn style_string(&self) -> Rc<String> {
        let opt_clone = self
            .cached
            .style
            .borrow()
            .as_ref()
            .map(|rc| rc.clone());
        opt_clone
            .unwrap_or_else(|| {
                let rc = Rc::new(self.generate_style_string());
                self.cached.style.replace(Some(rc.clone()));
                rc
            })
    }

    /// Produces an updated text alignment string.
    pub(crate) fn align_string(&self) -> Rc<String> {
        let opt_clone = self
            .cached
            .align
            .borrow()
            .as_ref()
            .map(|rc| rc.clone());
        opt_clone
            .unwrap_or_else(|| {
                let rc = Rc::new(self.align.to_string());
                self.cached.align.replace(Some(rc.clone()));
                rc
            })
    }

    /// Produces an updated text baseline string.
    pub(crate) fn baseline_string(&self) -> Rc<String> {
        let opt_clone = self
            .cached
            .baseline
            .borrow()
            .as_ref()
            .map(|rc| rc.clone());
        opt_clone
            .unwrap_or_else(|| {
                let rc = Rc::new(self.baseline.to_string());
                self.cached.baseline.replace(Some(rc.clone()));
                rc
            })
    }

    /// Produces an updated fill value.
    pub(crate) fn fill_jsvalue(&self) -> Rc<JsValue> {
        let opt_clone = self
            .cached
            .fill
            .borrow()
            .as_ref()
            .map(|rc| rc.clone());
        opt_clone
            .unwrap_or_else(|| {
                let opt_fill_color_rc = self.fill
                    .map(|fill| Rc::new(JsValue::from_str(&fill.to_string())));
                self.cached.fill.replace(Some(opt_fill_color_rc.clone()));
                opt_fill_color_rc
            })
            .unwrap_or_else(||
                Rc::new(JsValue::from_str(&Color::TRANSPARENT.to_string()))
            )
    }

    /// Produces an updated stroke value.
    pub(crate) fn stroke_width_number(&self) -> f64 {
        let opt_width = self
            .cached
            .stroke_width
            .get();
        opt_width
            .unwrap_or_else(|| {
                let opt_width = self
                    .stroke
                    .map(|s| s.0 as f64);
                self.cached.stroke_width.set(Some(opt_width));
                opt_width
            })
            .unwrap_or(0.)
    }

    /// Produces an updated stroke value.
    pub(crate) fn stroke_color_jsvalue(&self) -> Rc<JsValue> {
        let opt_clone = self
            .cached
            .stroke_color
            .borrow()
            .as_ref()
            .map(|rc| rc.clone());
        opt_clone
            .unwrap_or_else(|| {
                let opt_stroke_color_rc = self
                    .stroke
                    .map(|s| {
                        let color_str = s.1.to_string();
                        Rc::new(JsValue::from_str(&color_str))
                    });
                self.cached.stroke_color.replace(Some(opt_stroke_color_rc.clone()));
                opt_stroke_color_rc
            })
            .unwrap_or_else(||
                Rc::new(JsValue::from_str(&Color::TRANSPARENT.to_string()))
            )
    }

    /// Apply the font style to a canvas.
    #[allow(dead_code)]
    pub(crate) fn apply(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        ctx.set_font(&*self.style_string());
        ctx.set_text_align(&*self.align_string());
        ctx.set_text_baseline(&*self.baseline_string());
        if self.fill.is_some() {
            ctx.set_fill_style(&*self.fill_jsvalue());
        }
        if self.stroke.is_some() {
            ctx.set_line_dash_offset(0.);
            ctx.set_line_dash(&self.cached.empty_array)?;
            ctx.set_line_width(self.stroke_width_number());
            ctx.set_stroke_style(&*self.stroke_color_jsvalue());
        }
        Ok(())
    }

    /// Determine whether the style has had changes since last time the
    /// cache was updated.
    #[allow(dead_code)]
    pub(crate) fn cache_incomplete(&self) -> bool {
            self.cached.style.borrow().is_none()
        ||  self.cached.align.borrow().is_none()
        ||  self.cached.baseline.borrow().is_none()
        ||  self.cached.fill.borrow().is_none()
        ||  self.cached.stroke_color.borrow().is_none()
        ||  self.cached.stroke_width.get().is_none()
    }

    /// Updates the caches.
    #[allow(dead_code)]
    pub(crate) fn update_caches(&self) {
        self.style_string();
        self.align_string();
        self.baseline_string();
        self.fill_jsvalue();
        self.stroke_width_number();
        self.stroke_color_jsvalue();
    }
}


#[cfg(test)]
pub mod tests {
    use std::ops::Deref;
    use wasm_bindgen_test::*;
    use wasm_bindgen::JsCast;
    use crate::utils::html::get_common_canvas;
    use super::*;

    #[wasm_bindgen_test]
    fn text_style_constructor_and_builder_defaults() {
        let mut f = FontStyle::new();
        assert_eq!(f.italics()    , &FontItalics::DEFAULT);
        assert_eq!(f.variant()    , &FontVariant::DEFAULT);
        assert_eq!(f.weight()     , &FontWeight::DEFAULT);
        assert_eq!(f.size()       , &FontSize::DEFAULT);
        assert_eq!(f.family()     , &FontFamily::DEFAULT);
        assert_eq!(f.align()      , &TextAlign::DEFAULT);
        assert_eq!(f.baseline()   , &TextBaseline::DEFAULT);
        assert_eq!(f.fill()       , &Some(Color::Named(NamedColor::Black)));
        assert_eq!(f.stroke()     , &None);
        f = FontStyleBuilder::new().build();
        assert_eq!(f.italics()    , &FontItalics::DEFAULT);
        assert_eq!(f.variant()    , &FontVariant::DEFAULT);
        assert_eq!(f.weight()     , &FontWeight::DEFAULT);
        assert_eq!(f.size()       , &FontSize::DEFAULT);
        assert_eq!(f.family()     , &FontFamily::DEFAULT);
        assert_eq!(f.align()      , &TextAlign::DEFAULT);
        assert_eq!(f.baseline()   , &TextBaseline::DEFAULT);
        assert_eq!(f.fill()       , &Some(Color::Named(NamedColor::Black)));
        assert_eq!(f.stroke()     , &None);
    }

    #[wasm_bindgen_test]
    fn text_style_getters_setters() {
        use super::super::shorten::*;
        //  These tests that setters and getters refer to the same data.
        let mut f = FontStyle::new();
        assert!(f.cache_incomplete());
        {
            let setting = Italic;
            f.set_italics(setting);
            assert_eq!(f.italics(), &setting);
        }
        {
            let setting = SmallCaps;
            f.set_variant(setting);
            assert_eq!(f.variant(), &setting);
        }
        {
            let setting = Bold;
            f.set_weight(setting);
            assert_eq!(f.weight(), &setting);
        }
        {
            let setting = FontSizeAsPercent(25.);
            f.set_size(setting);
            assert_eq!(f.size(), &setting);
        }
        {
            let setting = NamedFamily("Times New Roman".to_string());
            f.set_family(setting.clone());
            assert_eq!(f.family(), &setting);
        }
        {
            let setting = AlignCenter;
            f.set_align(setting);
            assert_eq!(f.align(), &setting);
        }
        {
            let setting = MiddleBaseline;
            f.set_baseline(setting);
            assert_eq!(f.baseline(), &setting);
        }
        {
            let setting = Some(HSL(150., 25., 88.));
            f.set_fill(setting);
            assert_eq!(f.fill(), &setting);
        }
        {
            let setting = Some((3, HSLA(75., 100., 21., 0.5)));
            f.set_stroke(setting);
            assert_eq!(f.stroke(), &setting);
        }
    }

    #[wasm_bindgen_test]
    fn text_style_cache() {
        use super::super::shorten::*;
        let mut f = FontStyle::new();
        assert!(f.cache_incomplete());
        f
            .set_italics(Italic)
            .set_variant(SmallCaps)
            .set_weight(Bold)
            .set_size(FontSizeAsPercent(25.))
            .set_family(NamedFamily("Times New Roman".to_string()).clone())
            .set_align(AlignCenter)
            .set_baseline(MiddleBaseline)
            .set_fill(Some(HSL(150., 25., 88.)))
            .set_stroke(Some((3, HSLA(75., 100., 21., 0.5))));
        //  Proofs of correct cache updating and queries.
        assert!(f.cache_incomplete());
        {
            assert!(f.cached.style.borrow().is_none());
            let cached = f.style_string();
            assert!(f.cached.style.borrow().is_some());
            assert_eq!(f.cached.style.borrow().as_ref().unwrap().deref(), cached.deref());
            assert_eq!(&*cached, &f.generate_style_string());
        }
        assert!(f.cache_incomplete());
        {
            assert!(f.cached.align.borrow().is_none());
            let cached = f.align_string();
            assert!(f.cached.align.borrow().is_some());
            assert_eq!(f.cached.align.borrow().as_ref().unwrap().deref(), cached.deref());
            assert_eq!(&*cached, &f.align().to_string());
        }
        assert!(f.cache_incomplete());
        {
            assert!(f.cached.baseline.borrow().is_none());
            let cached = f.baseline_string();
            assert!(f.cached.baseline.borrow().is_some());
            assert_eq!(f.cached.baseline.borrow().as_ref().unwrap().deref(), cached.deref());
            assert_eq!(&*cached, &f.baseline().to_string());
        }
        assert!(f.cache_incomplete());
        {
            assert!(f.cached.fill.borrow().is_none());
            let cached = f.fill_jsvalue();
            assert!(f.cached.fill.borrow().is_some());
            assert_eq!(f.cached.fill.borrow().as_ref().unwrap().as_ref().unwrap().deref(), cached.deref());
            assert_eq!(&*cached, &JsValue::from_str(&f.fill().unwrap().to_string()));
        }
        assert!(f.cache_incomplete());
        {
            assert!(f.cached.stroke_color.borrow().is_none());
            let cached = f.stroke_color_jsvalue();
            assert!(f.cached.stroke_color.borrow().is_some());
            assert_eq!(f.cached.stroke_color.borrow().as_ref().unwrap().as_ref().unwrap().deref(), cached.deref());
            assert_eq!(&*cached, &JsValue::from_str(&f.stroke().unwrap().1.to_string()));
        }
        assert!(f.cache_incomplete());
        {
            assert!(f.cached.stroke_width.get().is_none());
            let cached = f.stroke_width_number();
            assert!(f.cached.stroke_width.get().as_ref().and_then(|x| x.as_ref()).is_some());
            assert_eq!(f.cached.stroke_width.get().unwrap().unwrap(), cached);
            assert_eq!(cached, f.stroke().unwrap().0 as f64);
        }
        //  Testing that, for items with nested Options, it updates as it should.
        assert!(!f.cache_incomplete());
        f.set_fill(None);
        f.set_stroke(None);
        assert!(f.cache_incomplete());
        assert!(f.cached.fill.borrow().as_ref().is_none());
        f.fill_jsvalue();
        assert!(f.cached.fill.borrow().as_ref().is_some());
        assert!(f.cached.fill.borrow().as_ref().and_then(|x| x.as_ref()).is_none());
        assert!(f.cache_incomplete());
        f.stroke_color_jsvalue();
        assert!(f.cached.stroke_color.borrow().is_some());
        assert!(f.cached.stroke_color.borrow().as_ref().unwrap().is_none());
        assert!(f.cached.stroke_width.get().is_none());
        f.stroke_width_number();
        assert!(f.cached.stroke_width.get().is_some());
        assert!(f.cached.stroke_width.get().as_ref().unwrap().is_none());
        assert!(!f.cache_incomplete());
        f.set_stroke(Some((1, RGBA(90, 5, 200, 0.75))));
        f.stroke_color_jsvalue();
        assert!(f.cached.stroke_color.borrow().as_ref().unwrap().is_some());
        f.stroke_width_number();
        assert!(f.cached.stroke_width.get().as_ref().unwrap().is_some());
        assert!(!f.cache_incomplete());
        //  Check that the diverse updates to style only touch the single style cache.
        {
            f.set_italics(Italic);
            assert!(f.cache_incomplete());
            f.style_string();
            assert!(!f.cache_incomplete());
        }
        {
            f.set_variant(NormalLowercase);
            assert!(f.cache_incomplete());
            f.style_string();
            assert!(!f.cache_incomplete());
        }
        {
            f.set_weight(Bold);
            assert!(f.cache_incomplete());
            f.style_string();
            assert!(!f.cache_incomplete());
        }
        {
            f.set_size(XLarge);
            assert!(f.cache_incomplete());
            f.style_string();
            assert!(!f.cache_incomplete());
        }
        {
            f.set_family(Cursive);
            assert!(f.cache_incomplete());
            f.style_string();
            assert!(!f.cache_incomplete());
        }
    }

    #[wasm_bindgen_test]
    fn context_application() {
        let swirlcanvas = get_common_canvas().unwrap();
        let swirlctx    = swirlcanvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let mut style   = FontStyleBuilder::new()
            .with_family(FontFamily::NamedFamily("Times New Roman".to_string()))
            .with_size(FontSize::XXLarge)
            .build();
        //console_log!("before application: family {}, size {}.", style.family().to_string(), style.size().to_string());
        assert!(style.apply(&swirlctx).is_ok());
        //console_log!("font: {}", swirlctx.font());
        let first_style = swirlctx.font();
        style
            .set_family(FontFamily::Fantasy)
            .set_size(FontSize::FontSizeAsUnits(25., FontSizeUnit::Mm));
        assert!(style.apply(&swirlctx).is_ok());
        //console_log!("after application: family {}, size {}.", style.family().to_string(), style.size().to_string());
        //console_log!("font: {}", swirlctx.font());
        assert_ne!(swirlctx.font(), first_style);
    }

    #[wasm_bindgen_test]
    fn all_styles() {
        use super::super::shorten::*;
        let all_italics     = &[Italic, NonItalic,];
        let all_variants    = &[SmallCaps, NormalLowercase,];
        let all_weights     = &[Bold, NormalWeight];
        let all_sizes =
            &[  XXSmall
            ,   XSmall
            ,   Small
            ,   Medium
            ,   Large
            ,   XLarge
            ,   XXLarge
            ,   XXXLarge
            ,   Larger
            ,   Smaller
            ,   FontSizeAsUnits(100., Ch)
            ,   FontSizeAsUnits(101., Em)
            ,   FontSizeAsUnits(102., Ex)
            ,   FontSizeAsUnits(103., Rem)
            ,   FontSizeAsUnits(104., Vh)
            ,   FontSizeAsUnits(105., Vw)
            ,   FontSizeAsUnits(106., Vmin)
            ,   FontSizeAsUnits(107., Vmax)
            ,   FontSizeAsUnits(108., Px)
            ,   FontSizeAsUnits(109., Cm)
            ,   FontSizeAsUnits(110., Mm)
            ,   FontSizeAsUnits(111., In)
            ,   FontSizeAsUnits(112., Pc)
            ,   FontSizeAsUnits(113., Pt)
            ,   FontSizeAsPercent(88.)
            ];
        let all_families =
            &[  Cursive
            ,   SansSerif
            ,   Monospace
            ,   Serif
            ,   Fantasy
            ,   NamedFamily("Cretino".to_string())
            ,   NamedFamily("Times New Roman".to_string())
            ];
        let swirlcanvas     = get_common_canvas().unwrap();
        let swirlctx        = swirlcanvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let mut style       = FontStyle::new();
        assert!(style.apply(&swirlctx).is_ok());
        let mut font_str    = swirlctx.font();
        for italic in all_italics {
            style.set_italics(italic.clone());
            assert!(style.apply(&swirlctx).is_ok());
            let new_font_str    = swirlctx.font();
            assert_ne!(font_str, new_font_str);
            font_str            = new_font_str;
        }
        for variant in all_variants {
            style.set_variant(variant.clone());
            assert!(style.apply(&swirlctx).is_ok());
            let new_font_str    = swirlctx.font();
            assert_ne!(font_str, new_font_str);
            font_str            = new_font_str;
        }
        //console_log!("Initial Weight: {}", style.weight().to_string());
        //console_log!("Initial context font: {}", swirlctx.font());
        for weight in all_weights {
            //console_log!("New Weight: {}", weight.to_string());
            style.set_weight(weight.clone());
            assert!(style.apply(&swirlctx).is_ok());
            //console_log!("Resulting context font: {}", swirlctx.font());
            let new_font_str    = swirlctx.font();
            assert_ne!(font_str, new_font_str);
            font_str            = new_font_str;
        }
        for size in all_sizes {
            style.set_size(size.clone());
            assert!(style.apply(&swirlctx).is_ok());
            let new_font_str    = swirlctx.font();
            assert_ne!(font_str, new_font_str);
            font_str            = new_font_str;
        }
        for family in all_families {
            style.set_family(family.clone());
            assert!(style.apply(&swirlctx).is_ok());
            let new_font_str    = swirlctx.font();
            assert_ne!(font_str, new_font_str);
            font_str            = new_font_str;
        }
    }

    #[wasm_bindgen_test]
    fn all_alignments() {
        use super::super::shorten::*;
        let swirlcanvas     = get_common_canvas().unwrap();
        let swirlctx        = swirlcanvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let mut style       = FontStyle::new();
        assert!(style.apply(&swirlctx).is_ok());
        let mut style_str   = swirlctx.text_align();
        let all_alignments  =
            &[  AlignRight
            ,   AlignLeft
            ,   AlignCenter
            ];
        for alignment in all_alignments {
            style.set_align(alignment.clone());
            assert!(style.apply(&swirlctx).is_ok());
            let new_style_str   = swirlctx.text_align();
            assert_ne!(style_str, new_style_str);
            style_str           = new_style_str;
        }
    }
}
