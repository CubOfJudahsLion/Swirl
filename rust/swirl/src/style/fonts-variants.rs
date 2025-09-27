make_enum_set! {
    FontEastAsianGlyphStandard ::=
        JIS78       => "jis78",
        JIS83       => "jis83",
        JIS90       => "jis90",
        JIS04       => "jis04",
        Simplified  => "simplified",
        Traditional => "traditional",
}

make_enum_set! {
    FontEastAsianWidth ::=
        FullWidth           => "full-width",
        ProportionalWidth   => "proportional-width",
}

#[derive(Clone, Copy)]
pub struct FontEastAsianVariant {
    pub ruby    : bool,
    pub glyph   : Option<FontEastAsianGlyphStandard>,
    pub width   : Option<FontEastAsianWidth>,
}

impl FontEastAsianVariant {
    pub fn normal() -> Self {
        FontEastAsianVariant {
            ruby    : false,
            glyph   : None,
            width   : None,
        }
    }
}

impl ToString for FontEastAsianVariant {
    fn to_string(&self) -> String {
        match (self.ruby, self.glyph, self.width) {
            (false, None, None)             =>
                "normal".to_string(),
            (ruby, opt_glyph, opt_width)    => {
                let mut accum   = String::new();
                ([
                    if ruby {
                        "ruby".to_string()
                    }
                    else {
                        String::new()
                    }
                ,   opt_glyph
                        .map(|glyph| glyph.to_string())
                        .unwrap_or(String::new())
                ,   opt_width
                        .map(|width| width.to_string())
                        .unwrap_or(String::new())
                ])
                    .iter()
                    .for_each(|item| {
                        if accum.len() > 0 {
                            accum.push(' ');
                        }
                        accum.push_str(item);
                    });
                accum
            },
        }
    }
}


/// Cumulative *historical ligature* type. It can additionally
/// specify *contextual ligatures*, or just itself.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontHistoricalLigature {
    WithContextual,
    HistoricalOnly,
}

impl ToString for FontHistoricalLigature {
    fn to_string(&self) -> String {
        let s = "historical-ligatures".to_string();
        if let &FontHistoricalLigature::WithContextual = self {
            s.push_str(" contextual");
        }
        s
    }
}

/// Cumulative *discretionary ligature* type. It can additionally
/// specify a `FontHistoricalligature`, a *contextual ligature*,
/// or just itself.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontDiscretionaryLigature {
    WithHistorical(FontHistoricalLigature),
    WithContextual,
    DiscretionaryOnly,
}

impl ToString for FontDiscretionaryLigature {
    fn to_string(&self) -> String {
        use FontDiscretionaryLigature::*;
        let s = "discretionary-ligatures".to_string();
        match self {
            &WithHistorical(hist)   => {
                s.push(' ');
                s.push_str(&hist.to_string());
            },
            &WithContextual         => {
                s.push_str(" contextual");
            },
            &DiscretionaryOnly      => (),
        }
        s
    }
}

/// Cumulative *common ligature* type. It can additionally
/// specify just itself, a `FontDiscretionaryligature`,
/// a `FontHistoricalligature`, a *contextual ligature*,
/// or just itself.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontCommonLigature {
    WithDiscretionary(FontDiscretionaryLigature),
    WithHistorical(FontHistoricalLigature),
    WithContextual,
    CommonOnly,
}

impl ToString for FontCommonLigature {
    fn to_string(&self) -> String {
        use FontCommonLigature::*;
        let s = "common-ligatures".to_string();
        match self {
            &WithDiscretionary(dis) => {
                s.push(' ');
                s.push_str(&dis.to_string());
            },
            &WithHistorical(hist)   => {
                s.push(' ');
                s.push_str(&hist.to_string());
            },
            &WithContextual         => {
                s.push_str(" contextual");
            },
            _                       => (),
        }
        s
    }
}


/// Font ligature spec type. While the specification
/// allows ligature specs other than "none" and "normal"
/// to be given in any order, it makes it clear that least
/// one of them should be present. Hence, the types are
/// designed to hold at least one variant if `None` or
/// `Normal` are not chosen.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontLigatures {
    None,
    Normal,
    Common(FontCommonLigature),
    Discretionary(FontDiscretionaryLigature),
    Historical(FontHistoricalLigature),
    Contextual,
}

impl ToString for FontLigatures {
    fn to_string(&self) -> String {
        use FontLigatures::*;
        match self {
            &None               => "none".to_string(),
            &Normal             => "normal".to_string(),
            &Common(comm)       => comm.to_string(),
            &Discretionary(dis) => dis.to_string(),
            &Historical(hist)   => hist.to_string(),
            &Contextual         => "contextual".to_string(),
        }
    }
}


make_enum_set! {
    FontNumericFigures ::=
        LiningNums      => "lining-nums",
        OldStyleNums    => "oldstyle-nums",
}


make_enum_set! {
    FontNumericSpacing ::=
        ProportionalNums    => "proportional-nums",
        TabularNums         => "tabular-nums",
}


make_enum_set! {
    FontNumericFractions ::=
        DiagonalFractions   => "diagonal-fractions",
        StackedFractions    => "stacked-fractions",
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FontNumeric {
    pub ordinal     : bool,
    pub slashed_zero: bool,
    pub figures     : Option<FontNumericFigures>,
    pub spacing     : Option<FontNumericSpacing>,
    pub fractions   : Option<FontNumericFractions>,
}

impl FontNumeric {
    #[inline]
    pub fn normal() -> Self {
        FontNumeric {
            ordinal     : false,
            slashed_zero: false,
            figures     : None,
            spacing     : None,
            fractions   : None,
        }
    }
}

impl ToString for FontNumeric {
    fn to_string(&self) -> String {
        match   ( self.ordinal
                , self.slashed_zero
                , self.figures
                , self.spacing
                , self.fractions
                )
        {
            (false, false, None, None, None)        =>
                "normal".to_string(),
            (ord, slz, opt_fig, opt_spc, opt_frac)  => {
                let s = String::new();
                [
                    if ord {
                        "ordinal".to_string()
                    }
                    else {
                        String::new()
                    }
                ,   if slz {
                        "slashed-zero".to_string()
                    }
                    else {
                        String::new()
                    }
                ,   opt_fig
                        .map(|fig| fig.to_string())
                        .unwrap_or(String::new())
                ,   opt_spc
                        .map(|spc| spc.to_string())
                        .unwrap_or(String::new())
                ,   opt_frac
                        .map(|frac| frac.to_string())
                        .unwrap_or(String::new())
                ]
                    .iter()
                    .for_each(|&item| {
                        if s.len() > 0 {
                            s.push(' ');
                        }
                        s.push_str(&item);
                    });
                s
            },
        }
    }
}


/// A Font Variant reunites the 


