//! # Module        :   _Assets_
//! - **Purpose**   :   Creates a single record owning all the audiovisual
//!                     assets required for the game.
//! - **Version**   :   0.4.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Thu Mar 11 09:00:52 AM CST 2021
//! - **Modified**  :   Sat 06/04/2022 07:36 PM


use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, self};
use std::ops::Index;
use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;
use swirl::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum FontAssets {
    TonicLiver,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum NormalButtonAssets {
    Normal,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum NormalDisabledButtonAssets {
    Normal,
    Disabled,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum AllStateButtonAssets {
    Normal,
    Over,
    Down,
    Disabled,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum ButtonAssets {
    CoinValuePlus(NormalDisabledButtonAssets),
    CoinValueMinus(NormalDisabledButtonAssets),
    Autoplay(NormalDisabledButtonAssets),
    Info(NormalDisabledButtonAssets),
    Spin(AllStateButtonAssets),
    Stop(NormalButtonAssets),
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum TextboxAssets {
    Balance,
    //CoinValue,
    TotalBet,
    TotalWin,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum InterfaceAssets {
    Button(ButtonAssets),
    Textbox(TextboxAssets),
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum SymbolAssets {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    W,
    X,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum ImgAssets {
    Background,
    Interface(InterfaceAssets),
    Symbol(SymbolAssets),
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Assets {
    Font(FontAssets),
    Img(ImgAssets),
}


impl From<Assets> for usize {
    fn from(a: Assets) -> Self {
        match a {
            Assets::Font(_)                                     => 100000,
            Assets::Img(ImgAssets::Background)                  => 200000,
            Assets::Img(ImgAssets::Interface(intf))             => 210000 +
                match intf {
                    InterfaceAssets::Button(but)                =>   1000 +
                        match but {
                            ButtonAssets::Autoplay(st)          =>      0 +  st as usize,
                            ButtonAssets::CoinValuePlus(cv)     =>    100 +  cv as usize,
                            ButtonAssets::CoinValueMinus(cv)    =>    200 +  cv as usize,
                            ButtonAssets::Info(inf)             =>    300 + inf as usize,
                            ButtonAssets::Spin(spn)             =>    400 + spn as usize,
                            ButtonAssets::Stop(stp)             =>    500 + stp as usize,
                        },
                    InterfaceAssets::Textbox(txb)               =>   2000 + txb as usize,
                },
            Assets::Img(ImgAssets::Symbol(sym))                 => 220000 + sym as usize,
        }
    }
}

impl From<Assets> for &'static str {
    fn from(a: Assets) -> Self {
        match a {
            Assets::Font(FontAssets::TonicLiver)                                                                                            => "fonts/LHFTonicLIVER.woff",

            Assets::Img(ImgAssets::Background)                                                                                              => "img/bkgd.png",

            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValuePlus(NormalDisabledButtonAssets::Normal))))     => "img/interface/btnPlus.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValuePlus(NormalDisabledButtonAssets::Disabled))))   => "img/interface/btnPlus_disabled.png",

            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValueMinus(NormalDisabledButtonAssets::Normal))))    => "img/interface/btnMinus.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValueMinus(NormalDisabledButtonAssets::Disabled))))  => "img/interface/btnMinus_disabled.png",

            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Autoplay(NormalDisabledButtonAssets::Normal))))          => "img/interface/btnAutoplay.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Autoplay(NormalDisabledButtonAssets::Disabled))))        => "img/interface/btnAutoplay_disabled.png",

            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Info(NormalDisabledButtonAssets::Normal))))              => "img/interface/btnPaytable.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Info(NormalDisabledButtonAssets::Disabled))))            => "img/interface/btnPaytable_disabled.png",

            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Normal))))                    => "img/interface/btnSpin_normal.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Over))))                      => "img/interface/btnSpin_hover.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Down))))                      => "img/interface/btnSpin_clicked.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Disabled))))                  => "img/interface/btnSpin_disabled.png",

            Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Stop(NormalButtonAssets::Normal))))                      => "img/interface/btnStop_normal.png",

            Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::Balance)))                                             => "img/interface/boxBkgd_balance.png",
            //Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::CoinValue)))                                           => "img/interface/boxBkgd_plusMinus.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::TotalBet)))                                            => "img/interface/boxBkgd_plusMinus.png",
            //"img/interface/boxBkgd_totalbet.png",
            Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::TotalWin)))                                            => "img/interface/boxBkgd_totalwin.png",

            Assets::Img(ImgAssets::Symbol(SymbolAssets::A))                                                                                 => "img/symbols/A.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::B))                                                                                 => "img/symbols/B.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::C))                                                                                 => "img/symbols/C.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::D))                                                                                 => "img/symbols/D.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::E))                                                                                 => "img/symbols/E.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::F))                                                                                 => "img/symbols/F.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::G))                                                                                 => "img/symbols/G.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::H))                                                                                 => "img/symbols/H.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::I))                                                                                 => "img/symbols/I.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::J))                                                                                 => "img/symbols/J.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::K))                                                                                 => "img/symbols/K.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::L))                                                                                 => "img/symbols/L.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::W))                                                                                 => "img/symbols/W.png",
            Assets::Img(ImgAssets::Symbol(SymbolAssets::X))                                                                                 => "img/symbols/X.png",
        }
    }
}

impl std::fmt::Display for Assets {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(<&'static str as From<Assets>>::from(*self))
    }
}

impl EnumSet for Assets {
    const ORDERED_VARIANTS: &'static [Assets] = &[
        Assets::Font(FontAssets::TonicLiver),
        Assets::Img(ImgAssets::Background),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValuePlus(NormalDisabledButtonAssets::Normal)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValuePlus(NormalDisabledButtonAssets::Disabled)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValueMinus(NormalDisabledButtonAssets::Normal)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValueMinus(NormalDisabledButtonAssets::Disabled)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Autoplay(NormalDisabledButtonAssets::Normal)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Autoplay(NormalDisabledButtonAssets::Disabled)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Info(NormalDisabledButtonAssets::Normal)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Info(NormalDisabledButtonAssets::Disabled)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Normal)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Over)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Down)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Disabled)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Stop(NormalButtonAssets::Normal)))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::Balance))),
        //Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::CoinValue))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::TotalBet))),
        Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::TotalWin))),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::A)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::B)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::C)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::D)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::E)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::F)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::G)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::H)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::I)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::J)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::K)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::L)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::W)),
        Assets::Img(ImgAssets::Symbol(SymbolAssets::X)),
    ];
}

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum Icon {
    A = SymbolAssets::A as usize,
    B = SymbolAssets::B as usize,
    C = SymbolAssets::C as usize,
    D = SymbolAssets::D as usize,
    E = SymbolAssets::E as usize,
    F = SymbolAssets::F as usize,
    G = SymbolAssets::G as usize,
    H = SymbolAssets::H as usize,
    I = SymbolAssets::I as usize,
    J = SymbolAssets::J as usize,
    K = SymbolAssets::K as usize,
    L = SymbolAssets::L as usize,
    W = SymbolAssets::W as usize,
    X = SymbolAssets::X as usize,
}

impl From<Icon> for SymbolAssets {
    fn from(icon: Icon) -> Self {
        use Icon::*;
        match icon {
            A => SymbolAssets::A,
            B => SymbolAssets::B,
            C => SymbolAssets::C,
            D => SymbolAssets::D,
            E => SymbolAssets::E,
            F => SymbolAssets::F,
            G => SymbolAssets::G,
            H => SymbolAssets::H,
            I => SymbolAssets::I,
            J => SymbolAssets::J,
            K => SymbolAssets::K,
            L => SymbolAssets::L,
            W => SymbolAssets::W,
            X => SymbolAssets::X,
        }
    }
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        use Icon::*;
        match icon {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
            H => 'H',
            I => 'I',
            J => 'J',
            K => 'K',
            L => 'L',
            W => 'W',
            X => 'X',
        }
    }
}

impl TryFrom<char> for Icon {
    type Error = JsValue;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        use Icon::*;
        match letter {
            'A' | 'a'   =>  Ok(A),
            'B' | 'b'   =>  Ok(B),
            'C' | 'c'   =>  Ok(C),
            'D' | 'd'   =>  Ok(D),
            'E' | 'e'   =>  Ok(E),
            'F' | 'f'   =>  Ok(F),
            'G' | 'g'   =>  Ok(G),
            'H' | 'h'   =>  Ok(H),
            'I' | 'i'   =>  Ok(I),
            'J' | 'j'   =>  Ok(J),
            'K' | 'k'   =>  Ok(K),
            'L' | 'l'   =>  Ok(L),
            'W' | 'w'   =>  Ok(W),
            'X' | 'x'   =>  Ok(X),
            _           =>  Err(JsValue::from_str(&format!(
                                "Error: '{}' corresponds to no reel symbol.",
                                letter
                            ))),
        }
    }
}

impl Display for Icon {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("Icon({})", <char as From<Icon>>::from(*self)))
    }
}

impl TryFrom<Assets> for Icon {
    type Error = ();

    fn try_from(asset: Assets) -> Result<Self, Self::Error> {
        use Icon::*;
        match asset {
            Assets::Img(ImgAssets::Symbol(SymbolAssets::A)) => Ok(A),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::B)) => Ok(B),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::C)) => Ok(C),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::D)) => Ok(D),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::E)) => Ok(E),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::F)) => Ok(F),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::G)) => Ok(G),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::H)) => Ok(H),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::I)) => Ok(I),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::J)) => Ok(J),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::K)) => Ok(K),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::L)) => Ok(L),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::W)) => Ok(W),
            Assets::Img(ImgAssets::Symbol(SymbolAssets::X)) => Ok(X),
            _                                               => Err(()),
        }
    }
}


//  From here on out, it's data structures to hold the assets.
pub struct Stop {
    pub normal: HtmlImageElement,
}

pub struct Spin {
    pub normal  : HtmlImageElement,
    pub over    : HtmlImageElement,
    pub down    : HtmlImageElement,
    pub disabled: HtmlImageElement,
}

pub struct Info {
    pub normal  : HtmlImageElement,
    pub disabled: HtmlImageElement,
}

pub struct CoinValue {
    pub normal  : HtmlImageElement,
    pub disabled: HtmlImageElement,
}

pub struct Autoplay {
    pub normal  : HtmlImageElement,
    pub disabled: HtmlImageElement,
}

pub struct Button {
    pub coin_plus   : CoinValue,
    pub coin_minus  : CoinValue,
    pub info        : Info,
    pub spin        : Spin,
    pub stop        : Stop,
    pub autoplay    : Autoplay,
}

pub struct TextBox {
    pub balance     : HtmlImageElement,
    //pub coin_value  : HtmlImageElement,
    pub total_bet   : HtmlImageElement,
    pub total_win   : HtmlImageElement,
}

pub struct Interface {
    pub button  : Button,
    pub text_box: TextBox,
}

pub struct Symbols {
    pub a: HtmlImageElement,
    pub b: HtmlImageElement,
    pub c: HtmlImageElement,
    pub d: HtmlImageElement,
    pub e: HtmlImageElement,
    pub f: HtmlImageElement,
    pub g: HtmlImageElement,
    pub h: HtmlImageElement,
    pub i: HtmlImageElement,
    pub j: HtmlImageElement,
    pub k: HtmlImageElement,
    pub l: HtmlImageElement,
    pub w: HtmlImageElement,
    pub x: HtmlImageElement,
}

pub struct Imgs {
    pub background  : HtmlImageElement,
    pub interface   : Interface,
    pub symbols     : Symbols,
}

pub struct Fonts {
    pub tonic_liver: String,
}

//pub struct Sounds {
//    pub theme: HtmlAudioElement,
//}

pub struct AssetsStruct {
    pub imgs    : Imgs,
    pub fonts   : Fonts,
}

impl AssetsStruct {
    fn extract_font(asset_dict: &mut HashMap<Assets, Loaded>, key: Assets) -> Result<String, JsValue> {
        match asset_dict.remove(&key) {
            Some(Loaded::Font(font_face))   => Ok(font_face),
            Some(_)                         => Err(JsValue::from_str(&format!("Asset {} is not a font", key.to_string()))),
            _                               => Err(JsValue::from_str(&format!("Asset {} not found", key.to_string()))),
        }
    }

    fn extract_image(asset_dict: &mut HashMap<Assets, Loaded>, key: Assets) -> Result<HtmlImageElement, JsValue> {
        match asset_dict.remove(&key) {
            Some(Loaded::Image(img))    => Ok(img),
            Some(_)                     => Err(JsValue::from_str(&format!("Asset {} is not an image", key.to_string()))),
            _                           => Err(JsValue::from_str(&format!("Asset {} not found", key.to_string()))),
        }
    }

    //fn extract_sound(asset_dict: &mut HashMap<Assets, Loaded>, key: Assets) -> Result<HtmlAudioElement, JsValue> {
    //    match asset_dict.remove(&key) {
    //        Some(Loaded::Sound(snd))    => Ok(snd),
    //        Some(_)                     => Err(JsValue::from_str(&format!("Asset {} is not a sound", key.to_string()))),
    //        _                           => Err(JsValue::from_str(&format!("Asset {} not found", key.to_string()))),
    //    }
    //}

    pub fn new(asset_dict: &mut HashMap<Assets, Loaded>) -> Result<Self, JsValue> {
        //
        macro_rules! extract {
            (font $key:expr) => {
                Self::extract_font(asset_dict, $key)?
            };
            (img $key:expr) => {
                Self::extract_image(asset_dict, $key)?
            };
            //(snd $key:ident) => {
            //    Self::extract_sound(asset_dict, Assets::$key)?
            //};
        }
        //
        Ok(AssetsStruct {
            imgs: Imgs {
                background: extract!(img Assets::Img(ImgAssets::Background)),
                interface: Interface {
                    button: Button {
                        autoplay: Autoplay {
                            normal  : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Autoplay(NormalDisabledButtonAssets::Normal))))),
                            disabled: extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Autoplay(NormalDisabledButtonAssets::Disabled))))),
                        },
                        coin_minus: CoinValue {
                            normal  : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValueMinus(NormalDisabledButtonAssets::Normal))))),
                            disabled: extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValueMinus(NormalDisabledButtonAssets::Disabled))))),
                        },
                        coin_plus: CoinValue {
                            normal  : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValuePlus(NormalDisabledButtonAssets::Normal))))),
                            disabled: extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::CoinValuePlus(NormalDisabledButtonAssets::Disabled))))),
                        },
                        info: Info {
                            normal  : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Info(NormalDisabledButtonAssets::Normal))))),
                            disabled: extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Info(NormalDisabledButtonAssets::Disabled))))),
                        },
                        spin: Spin {
                            normal  : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Normal))))),
                            over    : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Over))))),
                            down    : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Down))))),
                            disabled: extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Spin(AllStateButtonAssets::Disabled))))),
                        },
                        stop: Stop {
                            normal  : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Button(ButtonAssets::Stop(NormalButtonAssets::Normal))))),
                        },
                    },
                    text_box: TextBox {
                        balance     : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::Balance)))),
                        //coin_value  : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::CoinValue)))),
                        total_bet   : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::TotalBet)))),
                        total_win   : extract!(img Assets::Img(ImgAssets::Interface(InterfaceAssets::Textbox(TextboxAssets::TotalWin)))),
                    }
                },
                symbols: Symbols {
                    a: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::A))),
                    b: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::B))),
                    c: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::C))),
                    d: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::D))),
                    e: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::E))),
                    f: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::F))),
                    g: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::G))),
                    h: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::H))),
                    i: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::I))),
                    j: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::J))),
                    k: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::K))),
                    l: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::L))),
                    w: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::W))),
                    x: extract!(img Assets::Img(ImgAssets::Symbol(SymbolAssets::X))),
                }
            },
            fonts: Fonts {
                tonic_liver: extract!(font Assets::Font(FontAssets::TonicLiver)),
            },
        })
    }
}


impl Index<&Icon> for AssetsStruct {
    type Output = HtmlImageElement;

    fn index(&self, icon: &Icon) -> &Self::Output {
        use Icon::*;
        match icon {
            &A  => &self.imgs.symbols.a,
            &B  => &self.imgs.symbols.b,
            &C  => &self.imgs.symbols.c,
            &D  => &self.imgs.symbols.d,
            &E  => &self.imgs.symbols.e,
            &F  => &self.imgs.symbols.f,
            &G  => &self.imgs.symbols.g,
            &H  => &self.imgs.symbols.h,
            &I  => &self.imgs.symbols.i,
            &J  => &self.imgs.symbols.j,
            &K  => &self.imgs.symbols.k,
            &L  => &self.imgs.symbols.l,
            &W  => &self.imgs.symbols.w,
            &X  => &self.imgs.symbols.x,
        }
    }
}
