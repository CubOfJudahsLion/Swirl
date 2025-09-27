//! Module  :   Img
//! Purpose :   Identification of image resources
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar  9 02:29:55 PM CST 2021
//! Modified:   Thu 07/01/2021 01:51 PM


use std::fmt::{self, Display, Formatter};
use crate::make_resource_set;
use crate::utils::EnumSet;
use super::ResourceSet;


const DIR: &str = "img";

make_resource_set!{
    Single ::=
        Background  => "bkgd",
        Interface   => "interface-mockup";
    << "{}/{}.png", DIR, () >>
}

make_resource_set!{
    PlayButton ::=
        Normal  => "play-button-normal",
        Over    => "play-button-over",
        Down    => "play-button-click";
    << "{}/{}.png", DIR, () >>
}

//make_resource_set!{
//    StopButton ::=
//        Normal  => "stop-button-normal",
//        Over    => "stop-button-over",
//        Down    => "stop-button-click";
//    << "{}/{}.png", DIR, () >>
//}

make_resource_set!{
    Icon ::=
        A   => "A",
        B   => "B",
        C   => "C",
        D   => "D",
        E   => "E",
        F   => "F",
        G   => "G",
        H   => "H",
        I   => "I",
        J   => "J",
        //K   => "K",
        //L   => "L",
        W   => "W",
        X   => "X";
    << "{}/{}.png", DIR, () >>
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(usize)]
pub enum Set {
    S(Single),
    PB(PlayButton),
    //SB(StopButton),
    I(Icon),
}

impl From<Set> for &'static str {
    fn from(handle: Set) -> Self {
        match handle {
            Set::S(s)   => s.into(),
            Set::PB(p)  => p.into(),
            //Set::SB(s)  => s.into(),
            Set::I(i)   => i.into(),
        }
    }
}

impl From<Set> for usize {
    fn from(handle: Set) -> Self {
        match handle {
            Set::S(s)   => s as usize,
            Set::PB(p)  => p as usize + Single::VARIANT_COUNT,
            //Set::SB(s)  => s as usize + Single::VARIANT_COUNT + PlayButton::VARIANT_COUNT,
            Set::I(i)   => i as usize + Single::VARIANT_COUNT + PlayButton::VARIANT_COUNT, // + StopButton::VARIANT_COUNT,
        }
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(
            match *self {
                Set::S(s)   => s.into(),
                Set::PB(p)  => p.into(),
                //Set::SB(s)  => s.into(),
                Set::I(i)   => i.into(),
            }
        )
    }
}

use Single::*;
use Icon::*;

impl EnumSet for Set {
    const ORDERED_VARIANTS: &'static [Set] = &[
        Set::S(Background),
        Set::S(Interface),
        Set::PB(PlayButton::Normal), Set::PB(PlayButton::Over), Set::PB(PlayButton::Down),
        //Set::SB(StopButton::Normal), Set::SB(StopButton::Over), Set::SB(StopButton::Down),
        Set::I(A), Set::I(B), Set::I(C),
        Set::I(D), Set::I(E), Set::I(F),
        Set::I(G), Set::I(H), Set::I(I),
        //Set::I(J), Set::I(K), Set::I(L),
        Set::I(W), Set::I(X),
    ];
}

impl ResourceSet for Set {
    fn file_for(self) -> String {
        match self {
            Set::S(s)   => s.file_for(),
            Set::PB(p)  => p.file_for(),
            //Set::SB(s)  => s.file_for(),
            Set::I(i)   => i.file_for(),
        }
    }
}
