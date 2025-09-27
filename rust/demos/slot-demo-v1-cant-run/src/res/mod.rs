//! Module  :   Res (Resource enumeration)
//! Purpose :   Enumeration and symbolic representation of resources
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Wed Mar 10 11:54:32 PM CST 2021
//! Modified:   Tue Jul  6 02:21:33 PM CST 2021


use std::convert::From;
use std::fmt::{self, Formatter, Display};
use std::hash::Hash;

pub mod img;
pub mod font;
use crate::utils::EnumSet;


pub trait ResourceSet: EnumSet {
    fn file_for(self) -> String;
}


#[macro_export]
macro_rules! make_resource_set {
    ($enum_name:ident ::=
        $($variant:ident => $str:literal),+ ;
        << $fmt_str:literal, $($fmt_part:tt),+ >>
    ) =>
    {
        $crate::make_enum_set!{$enum_name ::= $($variant => $str,)+}

        impl $crate::res::ResourceSet for $enum_name {
            fn file_for(self) -> String {
                format!($fmt_str $(, make_resource_set!{@replace_empty $fmt_part, self})+)
            }
        }
    };

    (@replace_empty (), $other:tt) => {
        $other
    };

    (@replace_empty $orig:tt, $_:tt) => {
        $orig
    };
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum All {
    Img(img::Set),
    Font(font::Set),
}

impl From<All> for usize {
    fn from(subset: All) -> usize {
        match subset {
            All::Img(i)   => <usize as From<img::Set>>::from(i),
            All::Font(f)  => <usize as From<font::Set>>::from(f) + img::Set::VARIANT_COUNT,
        }
    }
}

impl From<All> for &'static str {
    fn from(subset: All) -> &'static str {
        match subset {
            All::Img(i)   => <&'static str as From<img::Set>>::from(i),
            All::Font(f)  => <&'static str as From<font::Set>>::from(f),
        }
    }
}

impl Display for All {
    fn fmt(&self, form: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &All::Img(i)  => i.fmt(form),
            &All::Font(f) => f.fmt(form),
        }
    }
}

use img::Set::{S, PB};
use img::Single::*;
use img::PlayButton::*;
use img::Icon::*;
use font::Set::*;

impl EnumSet for All {
    const ORDERED_VARIANTS: &'static[All] = &[
        All::Img(S(Background)),
        All::Img(S(Interface)),
        All::Img(PB(Normal)),
        All::Img(PB(Over)),
        All::Img(PB(Down)),
        All::Img(img::Set::I(A)), All::Img(img::Set::I(B)), All::Img(img::Set::I(C)),
        All::Img(img::Set::I(D)), All::Img(img::Set::I(E)), All::Img(img::Set::I(F)),
        All::Img(img::Set::I(G)), All::Img(img::Set::I(H)), All::Img(img::Set::I(img::Icon::I)),
        All::Img(img::Set::I(J)), //All::Img(img::Set::I(K)), All::Img(img::Set::I(L)),
        All::Img(img::Set::I(W)), All::Img(img::Set::I(X)),
        All::Font(TonicLiver),
        All::Font(TekoSemibold),
    ];
}

impl ResourceSet for All {
    fn file_for(self) -> String {
        match self {
            All::Img(img)   => img.file_for(),
            All::Font(font) => font.file_for(),
        }
    }
}
