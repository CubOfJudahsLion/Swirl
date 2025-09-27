//! Module  :   Assets
//! Purpose :   Creates a single record owning all the audiovisual
//!             assets required for the game.
//! Version :   0.2.0
//! Author  :   Alex Feterman
//! Created :   Thu Mar 11 09:00:52 AM CST 2021
//! Modified:   Thu 11/18/2021 01:26 AM


use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, self};
use std::ops::Index;
use wasm_bindgen::JsValue;
use web_sys::{HtmlAudioElement, HtmlImageElement};
use swirl::*;


make_enum_set!{
    Assets ::=
        FontTonicLiver                      => "fonts/LHFTonicLIVER.woff",

        ImgBackground                       => "img/bkgd.png",

        ImgInterfaceButtonCoinValueNormal   => "img/interface/coin-value-normal.png",
        ImgInterfaceButtonCoinValueOver     => "img/interface/coin-value-over.png",
        ImgInterfaceButtonCoinValueDown     => "img/interface/coin-value-down.png",

        ImgInterfaceButtonInfoNormal        => "img/interface/info-button-normal.png",
        ImgInterfaceButtonInfoOver          => "img/interface/info-button-over.png",
        ImgInterfaceButtonInfoDown          => "img/interface/info-button-down.png",

        ImgInterfaceButtonPlayNormal        => "img/interface/play-button-normal.png",
        ImgInterfaceButtonPlayOver          => "img/interface/play-button-over.png",
        ImgInterfaceButtonPlayDown          => "img/interface/play-button-down.png",

        ImgInterfaceButtonStopNormal        => "img/interface/stop-button-normal.png",
        ImgInterfaceButtonStopOver          => "img/interface/stop-button-over.png",
        ImgInterfaceButtonStopDown          => "img/interface/stop-button-down.png",

        ImgInterfaceWinningsOrnament        => "img/interface/winnings-ornament.png",

        ImgLightningAnimAllAround0001       => "img/lightning-anim/lightning-all-around-0001.png",
        ImgLightningAnimAllAround0002       => "img/lightning-anim/lightning-all-around-0002.png",
        ImgLightningAnimAllAround0003       => "img/lightning-anim/lightning-all-around-0003.png",
        ImgLightningAnimAllAround0004       => "img/lightning-anim/lightning-all-around-0004.png",
        ImgLightningAnimAllAround0005       => "img/lightning-anim/lightning-all-around-0005.png",
        ImgLightningAnimAllAround0006       => "img/lightning-anim/lightning-all-around-0006.png",
        ImgLightningAnimAllAround0007       => "img/lightning-anim/lightning-all-around-0007.png",
        ImgLightningAnimAllAround0008       => "img/lightning-anim/lightning-all-around-0008.png",
        ImgLightningAnimAllAround0009       => "img/lightning-anim/lightning-all-around-0009.png",
        ImgLightningAnimAllAround0010       => "img/lightning-anim/lightning-all-around-0010.png",
        ImgLightningAnimAllAround0011       => "img/lightning-anim/lightning-all-around-0011.png",
        ImgLightningAnimAllAround0012       => "img/lightning-anim/lightning-all-around-0012.png",
        ImgLightningAnimAllAround0013       => "img/lightning-anim/lightning-all-around-0013.png",
        ImgLightningAnimAllAround0014       => "img/lightning-anim/lightning-all-around-0014.png",
        ImgLightningAnimAllAround0015       => "img/lightning-anim/lightning-all-around-0015.png",
        ImgLightningAnimAllAround0016       => "img/lightning-anim/lightning-all-around-0016.png",
        ImgLightningAnimAllAround0017       => "img/lightning-anim/lightning-all-around-0017.png",
        ImgLightningAnimAllAround0018       => "img/lightning-anim/lightning-all-around-0018.png",
        ImgLightningAnimAllAround0019       => "img/lightning-anim/lightning-all-around-0019.png",
        ImgLightningAnimAllAround0020       => "img/lightning-anim/lightning-all-around-0020.png",
        ImgLightningAnimAllAround0021       => "img/lightning-anim/lightning-all-around-0021.png",
        ImgLightningAnimAllAround0022       => "img/lightning-anim/lightning-all-around-0022.png",
        ImgLightningAnimAllAround0023       => "img/lightning-anim/lightning-all-around-0023.png",
        ImgLightningAnimAllAround0024       => "img/lightning-anim/lightning-all-around-0024.png",
        ImgLightningAnimAllAround0025       => "img/lightning-anim/lightning-all-around-0025.png",
        ImgLightningAnimAllAround0026       => "img/lightning-anim/lightning-all-around-0026.png",
        ImgLightningAnimAllAround0027       => "img/lightning-anim/lightning-all-around-0027.png",
        ImgLightningAnimAllAround0028       => "img/lightning-anim/lightning-all-around-0028.png",
        ImgLightningAnimAllAround0029       => "img/lightning-anim/lightning-all-around-0029.png",
        ImgLightningAnimAllAround0030       => "img/lightning-anim/lightning-all-around-0030.png",
        ImgLightningAnimAllAround0031       => "img/lightning-anim/lightning-all-around-0031.png",
        ImgLightningAnimAllAround0032       => "img/lightning-anim/lightning-all-around-0032.png",
        ImgLightningAnimAllAround0033       => "img/lightning-anim/lightning-all-around-0033.png",
        ImgLightningAnimAllAround0034       => "img/lightning-anim/lightning-all-around-0034.png",

        ImgLightningAnimLeftCap0001         => "img/lightning-anim/lightning-left-cap-0001.png",
        ImgLightningAnimLeftCap0002         => "img/lightning-anim/lightning-left-cap-0002.png",
        ImgLightningAnimLeftCap0003         => "img/lightning-anim/lightning-left-cap-0003.png",
        ImgLightningAnimLeftCap0004         => "img/lightning-anim/lightning-left-cap-0004.png",
        ImgLightningAnimLeftCap0005         => "img/lightning-anim/lightning-left-cap-0005.png",
        ImgLightningAnimLeftCap0006         => "img/lightning-anim/lightning-left-cap-0006.png",
        ImgLightningAnimLeftCap0007         => "img/lightning-anim/lightning-left-cap-0007.png",
        ImgLightningAnimLeftCap0008         => "img/lightning-anim/lightning-left-cap-0008.png",
        ImgLightningAnimLeftCap0009         => "img/lightning-anim/lightning-left-cap-0009.png",
        ImgLightningAnimLeftCap0010         => "img/lightning-anim/lightning-left-cap-0010.png",
        ImgLightningAnimLeftCap0011         => "img/lightning-anim/lightning-left-cap-0011.png",
        ImgLightningAnimLeftCap0012         => "img/lightning-anim/lightning-left-cap-0012.png",
        ImgLightningAnimLeftCap0013         => "img/lightning-anim/lightning-left-cap-0013.png",
        ImgLightningAnimLeftCap0014         => "img/lightning-anim/lightning-left-cap-0014.png",
        ImgLightningAnimLeftCap0015         => "img/lightning-anim/lightning-left-cap-0015.png",
        ImgLightningAnimLeftCap0016         => "img/lightning-anim/lightning-left-cap-0016.png",
        ImgLightningAnimLeftCap0017         => "img/lightning-anim/lightning-left-cap-0017.png",
        ImgLightningAnimLeftCap0018         => "img/lightning-anim/lightning-left-cap-0018.png",
        ImgLightningAnimLeftCap0019         => "img/lightning-anim/lightning-left-cap-0019.png",
        ImgLightningAnimLeftCap0020         => "img/lightning-anim/lightning-left-cap-0020.png",
        ImgLightningAnimLeftCap0021         => "img/lightning-anim/lightning-left-cap-0021.png",
        ImgLightningAnimLeftCap0022         => "img/lightning-anim/lightning-left-cap-0022.png",
        ImgLightningAnimLeftCap0023         => "img/lightning-anim/lightning-left-cap-0023.png",
        ImgLightningAnimLeftCap0024         => "img/lightning-anim/lightning-left-cap-0024.png",
        ImgLightningAnimLeftCap0025         => "img/lightning-anim/lightning-left-cap-0025.png",
        ImgLightningAnimLeftCap0026         => "img/lightning-anim/lightning-left-cap-0026.png",
        ImgLightningAnimLeftCap0027         => "img/lightning-anim/lightning-left-cap-0027.png",
        ImgLightningAnimLeftCap0028         => "img/lightning-anim/lightning-left-cap-0028.png",
        ImgLightningAnimLeftCap0029         => "img/lightning-anim/lightning-left-cap-0029.png",
        ImgLightningAnimLeftCap0030         => "img/lightning-anim/lightning-left-cap-0030.png",
        ImgLightningAnimLeftCap0031         => "img/lightning-anim/lightning-left-cap-0031.png",
        ImgLightningAnimLeftCap0032         => "img/lightning-anim/lightning-left-cap-0032.png",
        ImgLightningAnimLeftCap0033         => "img/lightning-anim/lightning-left-cap-0033.png",
        ImgLightningAnimLeftCap0034         => "img/lightning-anim/lightning-left-cap-0034.png",

        ImgLightningAnimMiddle10001          => "img/lightning-anim/lightning-middle-1-0001.png",
        ImgLightningAnimMiddle10002          => "img/lightning-anim/lightning-middle-1-0002.png",
        ImgLightningAnimMiddle10003          => "img/lightning-anim/lightning-middle-1-0003.png",
        ImgLightningAnimMiddle10004          => "img/lightning-anim/lightning-middle-1-0004.png",
        ImgLightningAnimMiddle10005          => "img/lightning-anim/lightning-middle-1-0005.png",
        ImgLightningAnimMiddle10006          => "img/lightning-anim/lightning-middle-1-0006.png",
        ImgLightningAnimMiddle10007          => "img/lightning-anim/lightning-middle-1-0007.png",
        ImgLightningAnimMiddle10008          => "img/lightning-anim/lightning-middle-1-0008.png",
        ImgLightningAnimMiddle10009          => "img/lightning-anim/lightning-middle-1-0009.png",
        ImgLightningAnimMiddle10010          => "img/lightning-anim/lightning-middle-1-0010.png",
        ImgLightningAnimMiddle10011          => "img/lightning-anim/lightning-middle-1-0011.png",
        ImgLightningAnimMiddle10012          => "img/lightning-anim/lightning-middle-1-0012.png",
        ImgLightningAnimMiddle10013          => "img/lightning-anim/lightning-middle-1-0013.png",
        ImgLightningAnimMiddle10014          => "img/lightning-anim/lightning-middle-1-0014.png",
        ImgLightningAnimMiddle10015          => "img/lightning-anim/lightning-middle-1-0015.png",
        ImgLightningAnimMiddle10016          => "img/lightning-anim/lightning-middle-1-0016.png",
        ImgLightningAnimMiddle10017          => "img/lightning-anim/lightning-middle-1-0017.png",
        ImgLightningAnimMiddle10018          => "img/lightning-anim/lightning-middle-1-0018.png",
        ImgLightningAnimMiddle10019          => "img/lightning-anim/lightning-middle-1-0019.png",
        ImgLightningAnimMiddle10020          => "img/lightning-anim/lightning-middle-1-0020.png",
        ImgLightningAnimMiddle10021          => "img/lightning-anim/lightning-middle-1-0021.png",
        ImgLightningAnimMiddle10022          => "img/lightning-anim/lightning-middle-1-0022.png",
        ImgLightningAnimMiddle10023          => "img/lightning-anim/lightning-middle-1-0023.png",
        ImgLightningAnimMiddle10024          => "img/lightning-anim/lightning-middle-1-0024.png",
        ImgLightningAnimMiddle10025          => "img/lightning-anim/lightning-middle-1-0025.png",
        ImgLightningAnimMiddle10026          => "img/lightning-anim/lightning-middle-1-0026.png",
        ImgLightningAnimMiddle10027          => "img/lightning-anim/lightning-middle-1-0027.png",
        ImgLightningAnimMiddle10028          => "img/lightning-anim/lightning-middle-1-0028.png",
        ImgLightningAnimMiddle10029          => "img/lightning-anim/lightning-middle-1-0029.png",
        ImgLightningAnimMiddle10030          => "img/lightning-anim/lightning-middle-1-0030.png",
        ImgLightningAnimMiddle10031          => "img/lightning-anim/lightning-middle-1-0031.png",
        ImgLightningAnimMiddle10032          => "img/lightning-anim/lightning-middle-1-0032.png",
        ImgLightningAnimMiddle10033          => "img/lightning-anim/lightning-middle-1-0033.png",
        ImgLightningAnimMiddle10034          => "img/lightning-anim/lightning-middle-1-0034.png",


        ImgLightningAnimMiddle20001          => "img/lightning-anim/lightning-middle-2-0001.png",
        ImgLightningAnimMiddle20002          => "img/lightning-anim/lightning-middle-2-0002.png",
        ImgLightningAnimMiddle20003          => "img/lightning-anim/lightning-middle-2-0003.png",
        ImgLightningAnimMiddle20004          => "img/lightning-anim/lightning-middle-2-0004.png",
        ImgLightningAnimMiddle20005          => "img/lightning-anim/lightning-middle-2-0005.png",
        ImgLightningAnimMiddle20006          => "img/lightning-anim/lightning-middle-2-0006.png",
        ImgLightningAnimMiddle20007          => "img/lightning-anim/lightning-middle-2-0007.png",
        ImgLightningAnimMiddle20008          => "img/lightning-anim/lightning-middle-2-0008.png",
        ImgLightningAnimMiddle20009          => "img/lightning-anim/lightning-middle-2-0009.png",
        ImgLightningAnimMiddle20010          => "img/lightning-anim/lightning-middle-2-0010.png",
        ImgLightningAnimMiddle20011          => "img/lightning-anim/lightning-middle-2-0011.png",
        ImgLightningAnimMiddle20012          => "img/lightning-anim/lightning-middle-2-0012.png",
        ImgLightningAnimMiddle20013          => "img/lightning-anim/lightning-middle-2-0013.png",
        ImgLightningAnimMiddle20014          => "img/lightning-anim/lightning-middle-2-0014.png",
        ImgLightningAnimMiddle20015          => "img/lightning-anim/lightning-middle-2-0015.png",
        ImgLightningAnimMiddle20016          => "img/lightning-anim/lightning-middle-2-0016.png",
        ImgLightningAnimMiddle20017          => "img/lightning-anim/lightning-middle-2-0017.png",
        ImgLightningAnimMiddle20018          => "img/lightning-anim/lightning-middle-2-0018.png",
        ImgLightningAnimMiddle20019          => "img/lightning-anim/lightning-middle-2-0019.png",
        ImgLightningAnimMiddle20020          => "img/lightning-anim/lightning-middle-2-0020.png",
        ImgLightningAnimMiddle20021          => "img/lightning-anim/lightning-middle-2-0021.png",
        ImgLightningAnimMiddle20022          => "img/lightning-anim/lightning-middle-2-0022.png",
        ImgLightningAnimMiddle20023          => "img/lightning-anim/lightning-middle-2-0023.png",
        ImgLightningAnimMiddle20024          => "img/lightning-anim/lightning-middle-2-0024.png",
        ImgLightningAnimMiddle20025          => "img/lightning-anim/lightning-middle-2-0025.png",
        ImgLightningAnimMiddle20026          => "img/lightning-anim/lightning-middle-2-0026.png",
        ImgLightningAnimMiddle20027          => "img/lightning-anim/lightning-middle-2-0027.png",
        ImgLightningAnimMiddle20028          => "img/lightning-anim/lightning-middle-2-0028.png",
        ImgLightningAnimMiddle20029          => "img/lightning-anim/lightning-middle-2-0029.png",
        ImgLightningAnimMiddle20030          => "img/lightning-anim/lightning-middle-2-0030.png",
        ImgLightningAnimMiddle20031          => "img/lightning-anim/lightning-middle-2-0031.png",
        ImgLightningAnimMiddle20032          => "img/lightning-anim/lightning-middle-2-0032.png",
        ImgLightningAnimMiddle20033          => "img/lightning-anim/lightning-middle-2-0033.png",
        ImgLightningAnimMiddle20034          => "img/lightning-anim/lightning-middle-2-0034.png",

        ImgLightningAnimMiddle30001          => "img/lightning-anim/lightning-middle-3-0001.png",
        ImgLightningAnimMiddle30002          => "img/lightning-anim/lightning-middle-3-0002.png",
        ImgLightningAnimMiddle30003          => "img/lightning-anim/lightning-middle-3-0003.png",
        ImgLightningAnimMiddle30004          => "img/lightning-anim/lightning-middle-3-0004.png",
        ImgLightningAnimMiddle30005          => "img/lightning-anim/lightning-middle-3-0005.png",
        ImgLightningAnimMiddle30006          => "img/lightning-anim/lightning-middle-3-0006.png",
        ImgLightningAnimMiddle30007          => "img/lightning-anim/lightning-middle-3-0007.png",
        ImgLightningAnimMiddle30008          => "img/lightning-anim/lightning-middle-3-0008.png",
        ImgLightningAnimMiddle30009          => "img/lightning-anim/lightning-middle-3-0009.png",
        ImgLightningAnimMiddle30010          => "img/lightning-anim/lightning-middle-3-0010.png",
        ImgLightningAnimMiddle30011          => "img/lightning-anim/lightning-middle-3-0011.png",
        ImgLightningAnimMiddle30012          => "img/lightning-anim/lightning-middle-3-0012.png",
        ImgLightningAnimMiddle30013          => "img/lightning-anim/lightning-middle-3-0013.png",
        ImgLightningAnimMiddle30014          => "img/lightning-anim/lightning-middle-3-0014.png",
        ImgLightningAnimMiddle30015          => "img/lightning-anim/lightning-middle-3-0015.png",
        ImgLightningAnimMiddle30016          => "img/lightning-anim/lightning-middle-3-0016.png",
        ImgLightningAnimMiddle30017          => "img/lightning-anim/lightning-middle-3-0017.png",
        ImgLightningAnimMiddle30018          => "img/lightning-anim/lightning-middle-3-0018.png",
        ImgLightningAnimMiddle30019          => "img/lightning-anim/lightning-middle-3-0019.png",
        ImgLightningAnimMiddle30020          => "img/lightning-anim/lightning-middle-3-0020.png",
        ImgLightningAnimMiddle30021          => "img/lightning-anim/lightning-middle-3-0021.png",
        ImgLightningAnimMiddle30022          => "img/lightning-anim/lightning-middle-3-0022.png",
        ImgLightningAnimMiddle30023          => "img/lightning-anim/lightning-middle-3-0023.png",
        ImgLightningAnimMiddle30024          => "img/lightning-anim/lightning-middle-3-0024.png",
        ImgLightningAnimMiddle30025          => "img/lightning-anim/lightning-middle-3-0025.png",
        ImgLightningAnimMiddle30026          => "img/lightning-anim/lightning-middle-3-0026.png",
        ImgLightningAnimMiddle30027          => "img/lightning-anim/lightning-middle-3-0027.png",
        ImgLightningAnimMiddle30028          => "img/lightning-anim/lightning-middle-3-0028.png",
        ImgLightningAnimMiddle30029          => "img/lightning-anim/lightning-middle-3-0029.png",
        ImgLightningAnimMiddle30030          => "img/lightning-anim/lightning-middle-3-0030.png",
        ImgLightningAnimMiddle30031          => "img/lightning-anim/lightning-middle-3-0031.png",
        ImgLightningAnimMiddle30032          => "img/lightning-anim/lightning-middle-3-0032.png",
        ImgLightningAnimMiddle30033          => "img/lightning-anim/lightning-middle-3-0033.png",
        ImgLightningAnimMiddle30034          => "img/lightning-anim/lightning-middle-3-0034.png",

        ImgLightningAnimRightCap0001        => "img/lightning-anim/lightning-right-cap-0001.png",
        ImgLightningAnimRightCap0002        => "img/lightning-anim/lightning-right-cap-0002.png",
        ImgLightningAnimRightCap0003        => "img/lightning-anim/lightning-right-cap-0003.png",
        ImgLightningAnimRightCap0004        => "img/lightning-anim/lightning-right-cap-0004.png",
        ImgLightningAnimRightCap0005        => "img/lightning-anim/lightning-right-cap-0005.png",
        ImgLightningAnimRightCap0006        => "img/lightning-anim/lightning-right-cap-0006.png",
        ImgLightningAnimRightCap0007        => "img/lightning-anim/lightning-right-cap-0007.png",
        ImgLightningAnimRightCap0008        => "img/lightning-anim/lightning-right-cap-0008.png",
        ImgLightningAnimRightCap0009        => "img/lightning-anim/lightning-right-cap-0009.png",
        ImgLightningAnimRightCap0010        => "img/lightning-anim/lightning-right-cap-0010.png",
        ImgLightningAnimRightCap0011        => "img/lightning-anim/lightning-right-cap-0011.png",
        ImgLightningAnimRightCap0012        => "img/lightning-anim/lightning-right-cap-0012.png",
        ImgLightningAnimRightCap0013        => "img/lightning-anim/lightning-right-cap-0013.png",
        ImgLightningAnimRightCap0014        => "img/lightning-anim/lightning-right-cap-0014.png",
        ImgLightningAnimRightCap0015        => "img/lightning-anim/lightning-right-cap-0015.png",
        ImgLightningAnimRightCap0016        => "img/lightning-anim/lightning-right-cap-0016.png",
        ImgLightningAnimRightCap0017        => "img/lightning-anim/lightning-right-cap-0017.png",
        ImgLightningAnimRightCap0018        => "img/lightning-anim/lightning-right-cap-0018.png",
        ImgLightningAnimRightCap0019        => "img/lightning-anim/lightning-right-cap-0019.png",
        ImgLightningAnimRightCap0020        => "img/lightning-anim/lightning-right-cap-0020.png",
        ImgLightningAnimRightCap0021        => "img/lightning-anim/lightning-right-cap-0021.png",
        ImgLightningAnimRightCap0022        => "img/lightning-anim/lightning-right-cap-0022.png",
        ImgLightningAnimRightCap0023        => "img/lightning-anim/lightning-right-cap-0023.png",
        ImgLightningAnimRightCap0024        => "img/lightning-anim/lightning-right-cap-0024.png",
        ImgLightningAnimRightCap0025        => "img/lightning-anim/lightning-right-cap-0025.png",
        ImgLightningAnimRightCap0026        => "img/lightning-anim/lightning-right-cap-0026.png",
        ImgLightningAnimRightCap0027        => "img/lightning-anim/lightning-right-cap-0027.png",
        ImgLightningAnimRightCap0028        => "img/lightning-anim/lightning-right-cap-0028.png",
        ImgLightningAnimRightCap0029        => "img/lightning-anim/lightning-right-cap-0029.png",
        ImgLightningAnimRightCap0030        => "img/lightning-anim/lightning-right-cap-0030.png",
        ImgLightningAnimRightCap0031        => "img/lightning-anim/lightning-right-cap-0031.png",
        ImgLightningAnimRightCap0032        => "img/lightning-anim/lightning-right-cap-0032.png",
        ImgLightningAnimRightCap0033        => "img/lightning-anim/lightning-right-cap-0033.png",
        ImgLightningAnimRightCap0034        => "img/lightning-anim/lightning-right-cap-0034.png",

        ImgSymbolsA                         => "img/symbols/demon.png",
        ImgSymbolsB                         => "img/symbols/emperor.png",
        ImgSymbolsC                         => "img/symbols/jewel.png",
        ImgSymbolsD                         => "img/symbols/monkey.png",
        ImgSymbolsW                         => "img/symbols/wild.png",

        SndTheme                            => "audio/theme.mp3",
}


#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum Icon {
    A = Assets::ImgSymbolsA as usize,
    B = Assets::ImgSymbolsB as usize,
    C = Assets::ImgSymbolsC as usize,
    D = Assets::ImgSymbolsD as usize,
    W = Assets::ImgSymbolsW as usize,
}

impl From<Icon> for Assets {
    fn from(icon: Icon) -> Self {
        use Icon::*;
        use Assets::*;
        match icon {
            A => ImgSymbolsA,
            B => ImgSymbolsB,
            C => ImgSymbolsC,
            D => ImgSymbolsD,
            W => ImgSymbolsW,
        }
    }
}

impl Display for Icon {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        use Icon::*;
        fmt.write_str(
            match self {
                &A => "A",
                &B => "B",
                &C => "C",
                &D => "D",
                &W => "W",
            }
        )
    }
}

impl TryFrom<Assets> for Icon {
    type Error = ();

    fn try_from(asset: Assets) -> Result<Self, Self::Error> {
        use Icon::*;
        use Assets::*;
        match asset {
            ImgSymbolsA => Ok(A),
            ImgSymbolsB => Ok(B),
            ImgSymbolsC => Ok(C),
            ImgSymbolsD => Ok(D),
            ImgSymbolsW => Ok(W),
            _           => Err(()),
        }
    }
}


//  From here on out, it's data structures to hold the assets.
pub struct Stop {
    pub normal: HtmlImageElement,
    pub over: HtmlImageElement,
    pub down: HtmlImageElement,
}

pub struct Play {
    pub normal: HtmlImageElement,
    pub over: HtmlImageElement,
    pub down: HtmlImageElement,
}

pub struct Info {
    pub normal: HtmlImageElement,
    pub over: HtmlImageElement,
    pub down: HtmlImageElement,
}

pub struct CoinValue {
    pub normal: HtmlImageElement,
    pub over: HtmlImageElement,
    pub down: HtmlImageElement,
}

pub struct Button {
    pub coin_value: CoinValue,
    pub info: Info,
    pub play: Play,
    pub stop: Stop,
}

pub struct Interface {
    pub button: Button,
    pub winnings_ornament: HtmlImageElement,
}

pub struct LightningAnim {
    pub all_around: Vec<HtmlImageElement>,
    pub left_cap: Vec<HtmlImageElement>,
    pub middle_1: Vec<HtmlImageElement>,
    pub middle_2: Vec<HtmlImageElement>,
    pub middle_3: Vec<HtmlImageElement>,
    pub right_cap: Vec<HtmlImageElement>,
}

pub struct Symbols {
    pub a: HtmlImageElement,
    pub b: HtmlImageElement,
    pub c: HtmlImageElement,
    pub d: HtmlImageElement,
    pub w: HtmlImageElement,
}

pub struct Imgs {
    pub background: HtmlImageElement,
    pub interface: Interface,
    pub lightning_anim: LightningAnim,
    pub symbols: Symbols,
}

pub struct Fonts {
    pub tonic_liver: String,
}

pub struct Sounds {
    pub theme: HtmlAudioElement,
}

pub struct AssetsStruct {
    pub imgs: Imgs,
    pub fonts: Fonts,
    pub sounds: Sounds,
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

    fn extract_sound(asset_dict: &mut HashMap<Assets, Loaded>, key: Assets) -> Result<HtmlAudioElement, JsValue> {
        match asset_dict.remove(&key) {
            Some(Loaded::Sound(snd))    => Ok(snd),
            Some(_)                     => Err(JsValue::from_str(&format!("Asset {} is not a sound", key.to_string()))),
            _                           => Err(JsValue::from_str(&format!("Asset {} not found", key.to_string()))),
        }
    }

    pub fn new(asset_dict: &mut HashMap<Assets, Loaded>) -> Result<Self, JsValue> {
        //
        macro_rules! extract {
            (font $key:ident) => {
                Self::extract_font(asset_dict, Assets::$key)?
            };
            (img $key:ident) => {
                Self::extract_image(asset_dict, Assets::$key)?
            };
            (snd $key:ident) => {
                Self::extract_sound(asset_dict, Assets::$key)?
            };
        }
        //
        Ok(AssetsStruct {
            imgs: Imgs {
                background: extract!(img ImgBackground),
                interface: Interface {
                    button: Button {
                        coin_value: CoinValue {
                            normal: extract!(img ImgInterfaceButtonCoinValueNormal),
                            over: extract!(img ImgInterfaceButtonCoinValueOver),
                            down: extract!(img ImgInterfaceButtonCoinValueDown),
                        },
                        info: Info {
                            normal: extract!(img ImgInterfaceButtonInfoNormal),
                            over: extract!(img ImgInterfaceButtonInfoOver),
                            down: extract!(img ImgInterfaceButtonInfoDown),
                        },
                        play: Play {
                            normal: extract!(img ImgInterfaceButtonPlayNormal),
                            over: extract!(img ImgInterfaceButtonPlayOver),
                            down: extract!(img ImgInterfaceButtonPlayDown),
                        },
                        stop: Stop {
                            normal: extract!(img ImgInterfaceButtonStopNormal),
                            over: extract!(img ImgInterfaceButtonStopOver),
                            down: extract!(img ImgInterfaceButtonStopDown),
                        },
                    },
                    winnings_ornament: extract!(img ImgInterfaceWinningsOrnament),
                },
                lightning_anim: LightningAnim {
                    all_around: vec![
                        extract!(img ImgLightningAnimAllAround0001),
                        extract!(img ImgLightningAnimAllAround0002),
                        extract!(img ImgLightningAnimAllAround0003),
                        extract!(img ImgLightningAnimAllAround0004),
                        extract!(img ImgLightningAnimAllAround0005),
                        extract!(img ImgLightningAnimAllAround0006),
                        extract!(img ImgLightningAnimAllAround0007),
                        extract!(img ImgLightningAnimAllAround0008),
                        extract!(img ImgLightningAnimAllAround0009),
                        extract!(img ImgLightningAnimAllAround0010),
                        extract!(img ImgLightningAnimAllAround0011),
                        extract!(img ImgLightningAnimAllAround0012),
                        extract!(img ImgLightningAnimAllAround0013),
                        extract!(img ImgLightningAnimAllAround0014),
                        extract!(img ImgLightningAnimAllAround0015),
                        extract!(img ImgLightningAnimAllAround0016),
                        extract!(img ImgLightningAnimAllAround0017),
                        extract!(img ImgLightningAnimAllAround0018),
                        extract!(img ImgLightningAnimAllAround0019),
                        extract!(img ImgLightningAnimAllAround0020),
                        extract!(img ImgLightningAnimAllAround0021),
                        extract!(img ImgLightningAnimAllAround0022),
                        extract!(img ImgLightningAnimAllAround0023),
                        extract!(img ImgLightningAnimAllAround0024),
                        extract!(img ImgLightningAnimAllAround0025),
                        extract!(img ImgLightningAnimAllAround0026),
                        extract!(img ImgLightningAnimAllAround0027),
                        extract!(img ImgLightningAnimAllAround0028),
                        extract!(img ImgLightningAnimAllAround0029),
                        extract!(img ImgLightningAnimAllAround0030),
                        extract!(img ImgLightningAnimAllAround0031),
                        extract!(img ImgLightningAnimAllAround0032),
                        extract!(img ImgLightningAnimAllAround0033),
                        extract!(img ImgLightningAnimAllAround0034),
                    ],
                    left_cap:  vec![
                        extract!(img ImgLightningAnimLeftCap0001),
                        extract!(img ImgLightningAnimLeftCap0002),
                        extract!(img ImgLightningAnimLeftCap0003),
                        extract!(img ImgLightningAnimLeftCap0004),
                        extract!(img ImgLightningAnimLeftCap0005),
                        extract!(img ImgLightningAnimLeftCap0006),
                        extract!(img ImgLightningAnimLeftCap0007),
                        extract!(img ImgLightningAnimLeftCap0008),
                        extract!(img ImgLightningAnimLeftCap0009),
                        extract!(img ImgLightningAnimLeftCap0010),
                        extract!(img ImgLightningAnimLeftCap0011),
                        extract!(img ImgLightningAnimLeftCap0012),
                        extract!(img ImgLightningAnimLeftCap0013),
                        extract!(img ImgLightningAnimLeftCap0014),
                        extract!(img ImgLightningAnimLeftCap0015),
                        extract!(img ImgLightningAnimLeftCap0016),
                        extract!(img ImgLightningAnimLeftCap0017),
                        extract!(img ImgLightningAnimLeftCap0018),
                        extract!(img ImgLightningAnimLeftCap0019),
                        extract!(img ImgLightningAnimLeftCap0020),
                        extract!(img ImgLightningAnimLeftCap0021),
                        extract!(img ImgLightningAnimLeftCap0022),
                        extract!(img ImgLightningAnimLeftCap0023),
                        extract!(img ImgLightningAnimLeftCap0024),
                        extract!(img ImgLightningAnimLeftCap0025),
                        extract!(img ImgLightningAnimLeftCap0026),
                        extract!(img ImgLightningAnimLeftCap0027),
                        extract!(img ImgLightningAnimLeftCap0028),
                        extract!(img ImgLightningAnimLeftCap0029),
                        extract!(img ImgLightningAnimLeftCap0030),
                        extract!(img ImgLightningAnimLeftCap0031),
                        extract!(img ImgLightningAnimLeftCap0032),
                        extract!(img ImgLightningAnimLeftCap0033),
                        extract!(img ImgLightningAnimLeftCap0034),
                    ],
                    middle_1: vec![
                        extract!(img ImgLightningAnimMiddle10001),
                        extract!(img ImgLightningAnimMiddle10002),
                        extract!(img ImgLightningAnimMiddle10003),
                        extract!(img ImgLightningAnimMiddle10004),
                        extract!(img ImgLightningAnimMiddle10005),
                        extract!(img ImgLightningAnimMiddle10006),
                        extract!(img ImgLightningAnimMiddle10007),
                        extract!(img ImgLightningAnimMiddle10008),
                        extract!(img ImgLightningAnimMiddle10009),
                        extract!(img ImgLightningAnimMiddle10010),
                        extract!(img ImgLightningAnimMiddle10011),
                        extract!(img ImgLightningAnimMiddle10012),
                        extract!(img ImgLightningAnimMiddle10013),
                        extract!(img ImgLightningAnimMiddle10014),
                        extract!(img ImgLightningAnimMiddle10015),
                        extract!(img ImgLightningAnimMiddle10016),
                        extract!(img ImgLightningAnimMiddle10017),
                        extract!(img ImgLightningAnimMiddle10018),
                        extract!(img ImgLightningAnimMiddle10019),
                        extract!(img ImgLightningAnimMiddle10020),
                        extract!(img ImgLightningAnimMiddle10021),
                        extract!(img ImgLightningAnimMiddle10022),
                        extract!(img ImgLightningAnimMiddle10023),
                        extract!(img ImgLightningAnimMiddle10024),
                        extract!(img ImgLightningAnimMiddle10025),
                        extract!(img ImgLightningAnimMiddle10026),
                        extract!(img ImgLightningAnimMiddle10027),
                        extract!(img ImgLightningAnimMiddle10028),
                        extract!(img ImgLightningAnimMiddle10029),
                        extract!(img ImgLightningAnimMiddle10030),
                        extract!(img ImgLightningAnimMiddle10031),
                        extract!(img ImgLightningAnimMiddle10032),
                        extract!(img ImgLightningAnimMiddle10033),
                        extract!(img ImgLightningAnimMiddle10034),
                    ],
                    middle_2: vec![
                        extract!(img ImgLightningAnimMiddle20001),
                        extract!(img ImgLightningAnimMiddle20002),
                        extract!(img ImgLightningAnimMiddle20003),
                        extract!(img ImgLightningAnimMiddle20004),
                        extract!(img ImgLightningAnimMiddle20005),
                        extract!(img ImgLightningAnimMiddle20006),
                        extract!(img ImgLightningAnimMiddle20007),
                        extract!(img ImgLightningAnimMiddle20008),
                        extract!(img ImgLightningAnimMiddle20009),
                        extract!(img ImgLightningAnimMiddle20010),
                        extract!(img ImgLightningAnimMiddle20011),
                        extract!(img ImgLightningAnimMiddle20012),
                        extract!(img ImgLightningAnimMiddle20013),
                        extract!(img ImgLightningAnimMiddle20014),
                        extract!(img ImgLightningAnimMiddle20015),
                        extract!(img ImgLightningAnimMiddle20016),
                        extract!(img ImgLightningAnimMiddle20017),
                        extract!(img ImgLightningAnimMiddle20018),
                        extract!(img ImgLightningAnimMiddle20019),
                        extract!(img ImgLightningAnimMiddle20020),
                        extract!(img ImgLightningAnimMiddle20021),
                        extract!(img ImgLightningAnimMiddle20022),
                        extract!(img ImgLightningAnimMiddle20023),
                        extract!(img ImgLightningAnimMiddle20024),
                        extract!(img ImgLightningAnimMiddle20025),
                        extract!(img ImgLightningAnimMiddle20026),
                        extract!(img ImgLightningAnimMiddle20027),
                        extract!(img ImgLightningAnimMiddle20028),
                        extract!(img ImgLightningAnimMiddle20029),
                        extract!(img ImgLightningAnimMiddle20030),
                        extract!(img ImgLightningAnimMiddle20031),
                        extract!(img ImgLightningAnimMiddle20032),
                        extract!(img ImgLightningAnimMiddle20033),
                        extract!(img ImgLightningAnimMiddle20034),
                    ],
                    middle_3: vec![
                        extract!(img ImgLightningAnimMiddle30001),
                        extract!(img ImgLightningAnimMiddle30002),
                        extract!(img ImgLightningAnimMiddle30003),
                        extract!(img ImgLightningAnimMiddle30004),
                        extract!(img ImgLightningAnimMiddle30005),
                        extract!(img ImgLightningAnimMiddle30006),
                        extract!(img ImgLightningAnimMiddle30007),
                        extract!(img ImgLightningAnimMiddle30008),
                        extract!(img ImgLightningAnimMiddle30009),
                        extract!(img ImgLightningAnimMiddle30010),
                        extract!(img ImgLightningAnimMiddle30011),
                        extract!(img ImgLightningAnimMiddle30012),
                        extract!(img ImgLightningAnimMiddle30013),
                        extract!(img ImgLightningAnimMiddle30014),
                        extract!(img ImgLightningAnimMiddle30015),
                        extract!(img ImgLightningAnimMiddle30016),
                        extract!(img ImgLightningAnimMiddle30017),
                        extract!(img ImgLightningAnimMiddle30018),
                        extract!(img ImgLightningAnimMiddle30019),
                        extract!(img ImgLightningAnimMiddle30020),
                        extract!(img ImgLightningAnimMiddle30021),
                        extract!(img ImgLightningAnimMiddle30022),
                        extract!(img ImgLightningAnimMiddle30023),
                        extract!(img ImgLightningAnimMiddle30024),
                        extract!(img ImgLightningAnimMiddle30025),
                        extract!(img ImgLightningAnimMiddle30026),
                        extract!(img ImgLightningAnimMiddle30027),
                        extract!(img ImgLightningAnimMiddle30028),
                        extract!(img ImgLightningAnimMiddle30029),
                        extract!(img ImgLightningAnimMiddle30030),
                        extract!(img ImgLightningAnimMiddle30031),
                        extract!(img ImgLightningAnimMiddle30032),
                        extract!(img ImgLightningAnimMiddle30033),
                        extract!(img ImgLightningAnimMiddle30034),
                    ],
                    right_cap: vec![
                        extract!(img ImgLightningAnimRightCap0001),
                        extract!(img ImgLightningAnimRightCap0002),
                        extract!(img ImgLightningAnimRightCap0003),
                        extract!(img ImgLightningAnimRightCap0004),
                        extract!(img ImgLightningAnimRightCap0005),
                        extract!(img ImgLightningAnimRightCap0006),
                        extract!(img ImgLightningAnimRightCap0007),
                        extract!(img ImgLightningAnimRightCap0008),
                        extract!(img ImgLightningAnimRightCap0009),
                        extract!(img ImgLightningAnimRightCap0010),
                        extract!(img ImgLightningAnimRightCap0011),
                        extract!(img ImgLightningAnimRightCap0012),
                        extract!(img ImgLightningAnimRightCap0013),
                        extract!(img ImgLightningAnimRightCap0014),
                        extract!(img ImgLightningAnimRightCap0015),
                        extract!(img ImgLightningAnimRightCap0016),
                        extract!(img ImgLightningAnimRightCap0017),
                        extract!(img ImgLightningAnimRightCap0018),
                        extract!(img ImgLightningAnimRightCap0019),
                        extract!(img ImgLightningAnimRightCap0020),
                        extract!(img ImgLightningAnimRightCap0021),
                        extract!(img ImgLightningAnimRightCap0022),
                        extract!(img ImgLightningAnimRightCap0023),
                        extract!(img ImgLightningAnimRightCap0024),
                        extract!(img ImgLightningAnimRightCap0025),
                        extract!(img ImgLightningAnimRightCap0026),
                        extract!(img ImgLightningAnimRightCap0027),
                        extract!(img ImgLightningAnimRightCap0028),
                        extract!(img ImgLightningAnimRightCap0029),
                        extract!(img ImgLightningAnimRightCap0030),
                        extract!(img ImgLightningAnimRightCap0031),
                        extract!(img ImgLightningAnimRightCap0032),
                        extract!(img ImgLightningAnimRightCap0033),
                        extract!(img ImgLightningAnimRightCap0034),
                    ],
                },
                symbols: Symbols {
                    a: extract!(img ImgSymbolsA),
                    b: extract!(img ImgSymbolsB),
                    c: extract!(img ImgSymbolsC),
                    d: extract!(img ImgSymbolsD),
                    w: extract!(img ImgSymbolsW),
                }
            },
            fonts: Fonts {
                tonic_liver: extract!(font FontTonicLiver),
            },
            sounds: Sounds {
                theme: extract!(snd SndTheme),
            }
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
            &W  => &self.imgs.symbols.w,
        }
    }
}
