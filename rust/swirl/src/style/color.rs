//! Module  :   Color
//! Purpose :   Color-related representations and operations
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Wed 04/28/2021 05:48 PM
//! Modified:   Wed 10/13/2021 04:32 PM


use crate::make_enum_set;
//use super::StyleAngle;


//  The 148 named **CSS3** colors as
//  [documented by Mozilla](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value).
make_enum_set! {
    NamedColor ::=
        Black                   =>  "black",
        Silver                  =>  "silver",
        Gray                    =>  "gray",
        White                   =>  "white",
        Maroon                  =>  "maroon",
        Red                     =>  "red",
        Purple                  =>  "purple",
        Fuchsia                 =>  "fuchsia",
        Green                   =>  "green",
        Lime                    =>  "lime",
        Olive                   =>  "olive",
        Yellow                  =>  "yellow",
        Navy                    =>  "navy",
        Blue                    =>  "blue",
        Teal                    =>  "teal",
        Aqua                    =>  "aqua",
        Orange                  =>  "orange",
        AliceBlue               =>  "aliceblue",
        AntiqueWhite            =>  "antiquewhite",
        Aquamarine              =>  "aquamarine",
        Azure                   =>  "azure",
        Beige                   =>  "beige",
        Bisque                  =>  "bisque",
        BlanchedAlmond          =>  "blanchedalmond",
        BlueViolet              =>  "blueviolet",
        Brown                   =>  "brown",
        Burlywood               =>  "burlywood",
        Cadetblue               =>  "cadetblue",
        Chartreuse              =>  "chartreuse",
        Chocolate               =>  "chocolate",
        Coral                   =>  "coral",
        CornFlowerBlue          =>  "cornflowerblue",
        CornSilk                =>  "cornsilk",
        Crimson                 =>  "crimson",
        Cyan                    =>  "cyan",
        DarkBlue                =>  "darkblue",
        DarkCyan                =>  "darkcyan",
        DarkGoldenrod           =>  "darkgoldenrod",
        DarkGray                =>  "darkgray",
        DarkGreen               =>  "darkgreen",
        DarkGrey                =>  "darkgrey",
        DarkKhaki               =>  "darkkhaki",
        DarkMagenta             =>  "darkmagenta",
        DarkOliveGreen          =>  "darkolivegreen",
        DarkOrange              =>  "darkorange",
        DarkOrchid              =>  "darkorchid",
        DarkRed                 =>  "darkred",
        DarkSalmon              =>  "darksalmon",
        DarkSeaGreen            =>  "darkseagreen",
        DarkSlateBlue           =>  "darkslateblue",
        DarkSlateGray           =>  "darkslategray",
        DarkSlateGrey           =>  "darkslategrey",
        DarkTurquoise           =>  "darkturquoise",
        DarkViolet              =>  "darkviolet",
        DeepPink                =>  "deeppink",
        DeepSkyBlue             =>  "deepskyblue",
        Dimgray                 =>  "dimgray",
        Dimgrey                 =>  "dimgrey",
        DodgerBlue              =>  "dodgerblue",
        FireBrick               =>  "firebrick",
        FloralWhite             =>  "floralwhite",
        ForestGreen             =>  "forestgreen",
        Gainsboro               =>  "gainsboro",
        GhostWhite              =>  "ghostwhite",
        Gold                    =>  "gold",
        GoldenRod               =>  "goldenrod",
        GreenYellow             =>  "greenyellow",
        Grey                    =>  "grey",
        HoneyDew                =>  "honeydew",
        HotPink                 =>  "hotpink",
        IndianRed               =>  "indianred",
        Indigo                  =>  "indigo",
        Ivory                   =>  "ivory",
        Khaki                   =>  "khaki",
        Lavender                =>  "lavender",
        LavenderBlush           =>  "lavenderblush",
        LawnGreen               =>  "lawngreen",
        LemonChiffon            =>  "lemonchiffon",
        LightBlue               =>  "lightblue",
        LightCoral              =>  "lightcoral",
        LightCyan               =>  "lightcyan",
        LightGoldenRodYellow    =>  "lightgoldenrodyellow",
        LightGray               =>  "lightgray",
        LightGreen              =>  "lightgreen",
        LightGrey               =>  "lightgrey",
        LightPink               =>  "lightpink",
        LightSalmon             =>  "lightsalmon",
        LightSeaGreen           =>  "lightseagreen",
        LightSkyBlue            =>  "lightskyblue",
        LightSlateGray          =>  "lightslategray",
        LightSlateGrey          =>  "lightslategrey",
        LightSteelBlue          =>  "lightsteelblue",
        LightYellow             =>  "lightyellow",
        Limegreen               =>  "limegreen",
        Linen                   =>  "linen",
        Magenta                 =>  "magenta",
        MediumAquamarine        =>  "mediumaquamarine",
        MediumBlue              =>  "mediumblue",
        MediumOrchid            =>  "mediumorchid",
        MediumPurple            =>  "mediumpurple",
        MediumSeaGreen          =>  "mediumseagreen",
        MediumSlateBlue         =>  "mediumslateblue",
        MediumSpringGreen       =>  "mediumspringgreen",
        MediumTurquoise         =>  "mediumturquoise",
        MediumVioletRed         =>  "mediumvioletred",
        MidnightBlue            =>  "midnightblue",
        MintCream               =>  "mintcream",
        MistyRose               =>  "mistyrose",
        Moccasin                =>  "moccasin",
        NavajoWhite             =>  "navajowhite",
        OldLace                 =>  "oldlace",
        OliveDrab               =>  "olivedrab",
        OrangeRed               =>  "orangered",
        Orchid                  =>  "orchid",
        PaleGoldenRod           =>  "palegoldenrod",
        PaleGreen               =>  "palegreen",
        PaleTurquoise           =>  "paleturquoise",
        PaleVioletRed           =>  "palevioletred",
        PapayaWhip              =>  "papayawhip",
        PeachPuff               =>  "peachpuff",
        Peru                    =>  "peru",
        Pink                    =>  "pink",
        Plum                    =>  "plum",
        PowderBlue              =>  "powderblue",
        RosyBrown               =>  "rosybrown",
        RoyalBlue               =>  "royalblue",
        SaddleBrown             =>  "saddlebrown",
        Salmon                  =>  "salmon",
        SandyBrown              =>  "sandybrown",
        SeaGreen                =>  "seagreen",
        Seashell                =>  "seashell",
        Sienna                  =>  "sienna",
        SkyBlue                 =>  "skyblue",
        SlateBlue               =>  "slateblue",
        SlateGray               =>  "slategray",
        SlateGrey               =>  "slategrey",
        Snow                    =>  "snow",
        SpringGreen             =>  "springgreen",
        SteelBlue               =>  "steelblue",
        Tan                     =>  "tan",
        Thistle                 =>  "thistle",
        Tomato                  =>  "tomato",
        Turquoise               =>  "turquoise",
        Violet                  =>  "violet",
        Wheat                   =>  "wheat",
        WhiteSmoke              =>  "whitesmoke",
        YellowGreen             =>  "yellowgreen",
        RebeccaPurple           =>  "rebeccapurple",
}


/// Type alias for an Hue angle
pub type HueDegrees             = f64;

/// Type alias for color channels ([0..255] integer)
pub type ColorChannel           = u8;

/// Type alias for color alpha quantities ([0..1] float)
pub type ColorAlpha             = f64;

/// Type alias for saturation as a percentage
pub type SaturationPercentage   = f64;

/// Type alias for luminance as a percentage
pub type LuminancePercentage    = f64;


/// Color can be specified in a number of ways.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    //  A 0-255 triplet
    RGB(ColorChannel, ColorChannel, ColorChannel),
    //  The triplet plus a 0-1 alpha
    RGBA(ColorChannel, ColorChannel, ColorChannel, ColorAlpha),
    //  Degrees (0-360), percentage, percentage
    HSL(HueDegrees, SaturationPercentage, LuminancePercentage),
    //  Degrees (0-360), percentage, percentage and 0-1 alpha
    HSLA(HueDegrees, SaturationPercentage, LuminancePercentage, ColorAlpha),
    //  A named system color
    Named(NamedColor),
}

impl ToString for Color {
    fn to_string(&self) -> String {
        use Color::*;
        match self {
            &RGB(r, g, b)       => format!("rgb({}, {}, {})", r, g, b),
            &RGBA(r, g, b, a)   => format!("rgba({}, {}, {}, {})", r, g, b, a),
            &HSL(h, s, l)       => format!("hsl({}, {}%, {}%)", h.to_string(), s, l),
            &HSLA(h, s, l, a)   => format!("hsl({}, {}%, {}%, {})", h.to_string(), s, l, a),
            &Named(name)        => name.to_string(),
        }
    }
}

impl Color {
    /// The keyword `transparent` is not a named color.
    /// It's best to treat it as an RGBA color in the
    /// existing structures.
    pub const TRANSPARENT: Color = Color::RGBA(0, 0, 0, 0.0);
}


//  This table lists equivalent RGB triplets in the same order
//  as `NamedColor`, which is an `EnumSet` and thus guaranteed
//  to be `Ord` in order of appearance.
pub static NAMED_COLOR_RGB: [(ColorChannel, ColorChannel, ColorChannel); 148] = [
    (0x00, 0x00, 0x00),
    (0xc0, 0xc0, 0xc0),
    (0x80, 0x80, 0x80),
    (0xff, 0xff, 0xff),
    (0x80, 0x00, 0x00),
    (0xff, 0x00, 0x00),
    (0x80, 0x00, 0x80),
    (0xff, 0x00, 0xff),
    (0x00, 0x80, 0x00),
    (0x00, 0xff, 0x00),
    (0x80, 0x80, 0x00),
    (0xff, 0xff, 0x00),
    (0x00, 0x00, 0x80),
    (0x00, 0x00, 0xff),
    (0x00, 0x80, 0x80),
    (0x00, 0xff, 0xff),
    (0xff, 0xa5, 0x00),
    (0xf0, 0xf8, 0xff),
    (0xfa, 0xeb, 0xd7),
    (0x7f, 0xff, 0xd4),
    (0xf0, 0xff, 0xff),
    (0xf5, 0xf5, 0xdc),
    (0xff, 0xe4, 0xc4),
    (0xff, 0xeb, 0xcd),
    (0x8a, 0x2b, 0xe2),
    (0xa5, 0x2a, 0x2a),
    (0xde, 0xb8, 0x87),
    (0x5f, 0x9e, 0xa0),
    (0x7f, 0xff, 0x00),
    (0xd2, 0x69, 0x1e),
    (0xff, 0x7f, 0x50),
    (0x64, 0x95, 0xed),
    (0xff, 0xf8, 0xdc),
    (0xdc, 0x14, 0x3c),
    (0x00, 0xff, 0xff),
    (0x00, 0x00, 0x8b),
    (0x00, 0x8b, 0x8b),
    (0xb8, 0x86, 0x0b),
    (0xa9, 0xa9, 0xa9),
    (0x00, 0x64, 0x00),
    (0xa9, 0xa9, 0xa9),
    (0xbd, 0xb7, 0x6b),
    (0x8b, 0x00, 0x8b),
    (0x55, 0x6b, 0x2f),
    (0xff, 0x8c, 0x00),
    (0x99, 0x32, 0xcc),
    (0x8b, 0x00, 0x00),
    (0xe9, 0x96, 0x7a),
    (0x8f, 0xbc, 0x8f),
    (0x48, 0x3d, 0x8b),
    (0x2f, 0x4f, 0x4f),
    (0x2f, 0x4f, 0x4f),
    (0x00, 0xce, 0xd1),
    (0x94, 0x00, 0xd3),
    (0xff, 0x14, 0x93),
    (0x00, 0xbf, 0xff),
    (0x69, 0x69, 0x69),
    (0x69, 0x69, 0x69),
    (0x1e, 0x90, 0xff),
    (0xb2, 0x22, 0x22),
    (0xff, 0xfa, 0xf0),
    (0x22, 0x8b, 0x22),
    (0xdc, 0xdc, 0xdc),
    (0xf8, 0xf8, 0xff),
    (0xff, 0xd7, 0x00),
    (0xda, 0xa5, 0x20),
    (0xad, 0xff, 0x2f),
    (0x80, 0x80, 0x80),
    (0xf0, 0xff, 0xf0),
    (0xff, 0x69, 0xb4),
    (0xcd, 0x5c, 0x5c),
    (0x4b, 0x00, 0x82),
    (0xff, 0xff, 0xf0),
    (0xf0, 0xe6, 0x8c),
    (0xe6, 0xe6, 0xfa),
    (0xff, 0xf0, 0xf5),
    (0x7c, 0xfc, 0x00),
    (0xff, 0xfa, 0xcd),
    (0xad, 0xd8, 0xe6),
    (0xf0, 0x80, 0x80),
    (0xe0, 0xff, 0xff),
    (0xfa, 0xfa, 0xd2),
    (0xd3, 0xd3, 0xd3),
    (0x90, 0xee, 0x90),
    (0xd3, 0xd3, 0xd3),
    (0xff, 0xb6, 0xc1),
    (0xff, 0xa0, 0x7a),
    (0x20, 0xb2, 0xaa),
    (0x87, 0xce, 0xfa),
    (0x77, 0x88, 0x99),
    (0x77, 0x88, 0x99),
    (0xb0, 0xc4, 0xde),
    (0xff, 0xff, 0xe0),
    (0x32, 0xcd, 0x32),
    (0xfa, 0xf0, 0xe6),
    (0xff, 0x00, 0xff),
    (0x66, 0xcd, 0xaa),
    (0x00, 0x00, 0xcd),
    (0xba, 0x55, 0xd3),
    (0x93, 0x70, 0xdb),
    (0x3c, 0xb3, 0x71),
    (0x7b, 0x68, 0xee),
    (0x00, 0xfa, 0x9a),
    (0x48, 0xd1, 0xcc),
    (0xc7, 0x15, 0x85),
    (0x19, 0x19, 0x70),
    (0xf5, 0xff, 0xfa),
    (0xff, 0xe4, 0xe1),
    (0xff, 0xe4, 0xb5),
    (0xff, 0xde, 0xad),
    (0xfd, 0xf5, 0xe6),
    (0x6b, 0x8e, 0x23),
    (0xff, 0x45, 0x00),
    (0xda, 0x70, 0xd6),
    (0xee, 0xe8, 0xaa),
    (0x98, 0xfb, 0x98),
    (0xaf, 0xee, 0xee),
    (0xdb, 0x70, 0x93),
    (0xff, 0xef, 0xd5),
    (0xff, 0xda, 0xb9),
    (0xcd, 0x85, 0x3f),
    (0xff, 0xc0, 0xcb),
    (0xdd, 0xa0, 0xdd),
    (0xb0, 0xe0, 0xe6),
    (0xbc, 0x8f, 0x8f),
    (0x41, 0x69, 0xe1),
    (0x8b, 0x45, 0x13),
    (0xfa, 0x80, 0x72),
    (0xf4, 0xa4, 0x60),
    (0x2e, 0x8b, 0x57),
    (0xff, 0xf5, 0xee),
    (0xa0, 0x52, 0x2d),
    (0x87, 0xce, 0xeb),
    (0x6a, 0x5a, 0xcd),
    (0x70, 0x80, 0x90),
    (0x70, 0x80, 0x90),
    (0xff, 0xfa, 0xfa),
    (0x00, 0xff, 0x7f),
    (0x46, 0x82, 0xb4),
    (0xd2, 0xb4, 0x8c),
    (0xd8, 0xbf, 0xd8),
    (0xff, 0x63, 0x47),
    (0x40, 0xe0, 0xd0),
    (0xee, 0x82, 0xee),
    (0xf5, 0xde, 0xb3),
    (0xf5, 0xf5, 0xf5),
    (0x9a, 0xcd, 0x32),
    (0x66, 0x33, 0x99),
];


/// Obtains the color value for a named color.
#[inline(always)]
pub fn named_color_to_rgb(named: NamedColor) -> &'static (ColorChannel, ColorChannel, ColorChannel) {
    &NAMED_COLOR_RGB[named as usize]
}


/// Converts RGB to HSL. This conversion yields an angle in degrees
/// and saturation and luminance in the [0, 1] real range.
#[allow(dead_code)]
pub fn rgb_to_hsl(
    r: ColorChannel,
    g: ColorChannel,
    b: ColorChannel
) -> (HueDegrees, SaturationPercentage, LuminancePercentage)
{
    //  Reduce to [0, 1]
    let r               =   r as f64 / 255.0;
    let g               =   g as f64 / 255.0;
    let b               =   b as f64 / 255.0;
    //  Get highest/lowest components
    let (x_min, x_max)  =   [r, g, b]
                                .iter()
                                .fold(
                                    (f64::MAX, f64::MIN),
                                    |(min, max), curr|
                                        (
                                            f64::min(min, *curr)
                                        ,   f64::max(max, *curr)
                                        )
                                );
    //  Chromicity is a difference of those two
    let chroma          =   x_max - x_min;
    //  And lightness their average
    let lightness       =   (x_max + x_min) / 2.0;
    //  The 'value' (as it would correspond in other color systems)
    //  is the same as the maximum component.
    let value           =   x_max;
    //  Calculate hue from dominant and closest color
    let hue             =   60.0
                        *   if chroma == 0.0 {
                                0.0
                            }
                            else if value == r {
                                (g - b) / chroma
                            }
                            else if value == g {
                                2.0 + (b - r) / chroma
                            }
                            else {
                                4.0 + (r - g) / chroma
                            };
    //  Saturation is relative to the difference of value (brightness of
    //  highest component) and lightness.
    let saturation      =   if lightness == 0.0 || lightness == 1.0 {
                                0.0
                            }
                            else {
                                    (value - lightness)
                                /   f64::min(lightness, 1.0 - lightness)
                            };
    (hue, saturation, lightness)
}


/// Converts an HSL color into its equivalent RGB triplet.
/// `h` has to be provided as 0-360⁰, and `s` and `l` as
/// floating values within [0,1]. The response is a tuple with
/// (r, g, b) integer values in [0, 255].
pub fn hsl_to_rgb(
    h: HueDegrees,
    s: SaturationPercentage,
    l: LuminancePercentage
) -> (ColorChannel, ColorChannel, ColorChannel)
{
    //  Calculate chroma
    let c           =   (1.0 - f64::abs(2.0 * l - 1.0)) * s;
    //  Reduce hue to a "sector" in the circle
    let h           =   h % 360.0 / 60.0;
    //  Calculate second most prominent color component
    let x           =   c * (1.0 - f64::abs(h % 2.0 - 1.0));
    //  Partial assignment of major and second component
    let (r, g, b)   =   if 0.0 <= h && h < 1.0 {
                            (c, x, 0.0)
                        }
                        else if 1.0 <= h && h < 2.0 {
                            (x, c, 0.0)
                        }
                        else if 2.0 <= h && h < 3.0 {
                            (0.0, c, x)
                        }
                        else if 3.0 <= h && h < 4.0 {
                            (0.0, x, c)
                        }
                        else if 4.0 <= h && h < 5.0 {
                            (x, 0.0, c)
                        }
                        else {
                            (c, 0.0, x)
                        };
    //  Minimal common lightness for all components
    let m           =   l - c / 2.0;
    //  Add it to each and convert to bytes
    (
        ((r + m) * 255.0) as ColorChannel
    ,   ((g + m) * 255.0) as ColorChannel
    ,   ((b + m) * 255.0) as ColorChannel
    )
}
