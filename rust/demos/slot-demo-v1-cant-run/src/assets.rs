//! Module  :   Assets
//! Purpose :   Creates a single record owning all the audiovisual
//!             assets required for the game.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu Mar 11 09:00:52 AM CST 2021
//! Modified:   Mon 07/05/2021 01:40 PM


use std::result::Result;
use std::ops::Index;
use std::collections::HashMap;
use wasm_bindgen::throw_str;
use web_sys::HtmlImageElement;
use crate::res::{self, ResourceSet};
use swirl::loader::manual::Loaded;
use crate::utils::*;


pub struct AssetRetrievalError(pub String);


pub struct Single<T> {
    pub bkg         : T,
    pub interface   : T,
}

impl<T> Index<res::img::Single> for Single<T> {
    type Output = T;

    fn index(&self, img: res::img::Single) -> &Self::Output {
        use res::img::Single::*;
        match img {
            Background  => &self.bkg,
            Interface   => &self.interface,
        }
    }
}

impl<T> Single<T> {
    #[allow(dead_code)]
    pub fn new(bkg: T, interface: T) -> Single<T> {
        Single {bkg, interface}
    }

    pub fn map<U, F>(&self, f: &mut F) -> Single<U>
        where   F   : FnMut(&T) -> U
    {
        Single {
            bkg         : f(&self.bkg),
            interface   : f(&self.interface),
        }
    }
}


pub struct PlayButton<T> {
    pub normal  : T,
    pub over    : T,
    pub down    : T,
}

macro_rules! playbutton_resbutton_index {
    ($t:path) => {
        impl<T> Index<$t> for PlayButton<T> {
            type Output = T;

            fn index(&self, img: $t) -> &Self::Output {
                use $t::*;
                match img {
                    Normal  => &self.normal,
                    Over    => &self.over,
                    Down    => &self.down,
                }
            }
        }
    };
}

playbutton_resbutton_index!{res::img::PlayButton}
//playbutton_resbutton_index!{res::img::StopButton}

impl<T> PlayButton<T> {
    #[allow(dead_code)]
    pub fn new(normal: T, over: T, down: T) -> PlayButton<T> {
        PlayButton {normal, over, down,}
    }

    #[allow(dead_code)]
    pub fn map<U, F: FnMut(&T) -> U>(&self, f: &mut F) -> PlayButton<U> {
        PlayButton {
            normal  : f(&self.normal),
            over    : f(&self.over),
            down    : f(&self.down),
        }
    }
}

//pub type StopButton<T> = PlayButton<T>;


pub struct Icon<T: 'static>(EnumSetIndexed<res::img::Icon, T>);

impl<T: Sized> Index<res::img::Icon> for Icon<T> {
    type Output = T;

    fn index<'a>(&'a self, img: res::img::Icon) -> &'a Self::Output {
        &self.0[img]
    }
}

impl<T: Sized> Icon<T> {
    pub fn new_mapped<F: FnMut(&res::img::Icon) -> T>(mut f: F) -> Icon<T> {
        Icon(EnumSetIndexed::<res::img::Icon, T>::new_mapped(&mut f))
    }

    #[allow(dead_code)]
    pub fn map<U, F: FnMut(&T) -> U>(&self, f: &mut F) -> Icon<U> {
        let mapped = EnumSetIndexed::<res::img::Icon, U>::new_mapped(&mut |t| f(&self[*t]));
        Icon(mapped)
    }
}


pub struct Set<T: 'static> {
    pub single      : Single<T>,
    pub play_button : PlayButton<T>,
    //pub stop_button : StopButton<T>,
    pub icon        : Icon<T>,
}

impl<T> Set<T> {
    #[allow(dead_code)]
    pub fn map<U, F: FnMut(&T) -> U>(&self, mut f: F) -> Set<U> {
        Set {
            single      : self.single.map(&mut f),
            play_button : self.play_button.map(&mut f),
            //stop_button : self.stop_button.map(&mut f),
            icon        : self.icon.map(&mut f),
        }
    }
}

macro_rules! sub_index {
    ($($member:ident : $type:ident),+) => {
        $(
            impl<T> Index<res::img::$type> for Set<T> {
                type Output = <$type<T> as Index<res::img::$type>>::Output;

                #[inline(always)]
                fn index(&self, i: res::img::$type) -> &Self::Output {
                    self.$member.index(i)
                }
            }
        )+
    };
}

sub_index!{single: Single, play_button: PlayButton, /*stop_button: StopButton,*/ icon: Icon}

impl<T> Index<res::img::Set> for Set<T> {
    type Output = T;

    fn index(&self, idx: res::img::Set) -> &Self::Output {
        use res::img::Set;
        match idx {
            Set::S(s)  => self.single.index(s),
            Set::PB(p) => self.play_button.index(p),
            //Set::SB(s) => self.stop_button.index(s),
            Set::I(i)  => self.icon.index(i)
        }
    }
}


pub mod img {
    use super::*;

    fn recover_asset(dict: &mut HashMap<res::All, Loaded>, key: &res::All) -> Result<HtmlImageElement, AssetRetrievalError> {
        if let Some(asset) = dict.remove(key) {
            if let Loaded::Image(img) = asset {
                Ok(img)
            }
            else {
                Err(AssetRetrievalError(format!("assets.rs: {} is not an image.", key)))
            }
        }
        else {
            Err(AssetRetrievalError(format!("Image {}: file {} not found.", key, key.file_for())))
        }
    }

    pub type Single = super::Single<HtmlImageElement>;

    pub fn new_single_images<'a>(dict: &mut HashMap<res::All, Loaded>) -> Result<Single, AssetRetrievalError> {
        use res::{All::Img, img::{Set::S, Single::*}};
        let bkg         = recover_asset(dict, &Img(S(Background)))?;
        let interface   = recover_asset(dict, &Img(S(Interface)))?;
        Ok(Single {bkg, interface,})
    }


    pub type PlayButton = super::PlayButton<HtmlImageElement>;

    pub fn new_play_button_images(dict: &mut HashMap<res::All, Loaded>) -> Result<PlayButton, AssetRetrievalError> {
        use res::{All::Img, img::{Set::PB, PlayButton::*}};
        let normal  = recover_asset(dict, &Img(PB(Normal)))?;
        let over    = recover_asset(dict, &Img(PB(Over)))?;
        let down    = recover_asset(dict, &Img(PB(Down)))?;
        Ok(PlayButton {normal, over, down,})
    }


    //pub type StopButton = super::StopButton<HtmlImageElement>;

    //pub fn new_stop_button_images(dict: &mut HashMap<res::All, Loaded>) -> Result<PlayButton, AssetRetrievalError> {
    //    use res::{All::Img, img::{Set::SB, StopButton::*}};
    //    let normal  = recover_asset(dict, &Img(SB(Normal)))?;
    //    let over    = recover_asset(dict, &Img(SB(Over)))?;
    //    let down    = recover_asset(dict, &Img(SB(Down)))?;
    //    Ok(PlayButton {normal, over, down,})
    //}


    pub type Icon = super::Icon<HtmlImageElement>;

    pub fn new_icon_images<'a>(dict: &mut HashMap<res::All, Loaded>) -> Result<Icon, AssetRetrievalError> {
        Ok(
            super::Icon::new_mapped(
                |i| {
                    if let Some(Loaded::Image(hi)) = dict
                        .remove(&res::All::Img(res::img::Set::I(*i)))
                    {
                        hi
                    }
                    else {
                        throw_str(&format!(
                            "Resource error: {} is not an image or its file ({}) was not found.",
                            i, i.file_for()
                        ))
                    }
                }
            )
        )
    }


    pub type Set = super::Set<HtmlImageElement>;

    pub fn new_image_set<'a>(dict: &mut HashMap<res::All, Loaded>) -> Result<Set, AssetRetrievalError> {
        Ok(Set {
            single      : new_single_images(dict)?,
            play_button : new_play_button_images(dict)?,
            //stop_button : new_stop_button_images(dict)?,
            icon        : new_icon_images(dict)?,
        })
    }
}


pub mod font {
    use super::*;

    pub struct Set {
        pub tonic_liver : String,
        pub teko        : String,
    }

    pub fn new_font_set(dict: &mut HashMap<res::All, Loaded>) -> Result<Set, AssetRetrievalError> {
        Ok(Set {
            tonic_liver :
                if let Some(Loaded::Font(str)) =
                    dict.remove(&res::All::Font(res::font::Set::TonicLiver))
                {
                    Ok(str)
                }
                else {
                    Err(AssetRetrievalError(format!(
                        "Font \"{}\" ({}) not found.",
                        res::font::Set::TonicLiver,
                        res::font::Set::TonicLiver.file_for()
                    )))
                }?,
            teko        :
                if let Some(Loaded::Font(str)) =
                    dict.remove(&res::All::Font(res::font::Set::TekoSemibold))
                {
                    Ok(str)
                }
                else {
                    Err(AssetRetrievalError(format!(
                        "Font \"{}\" ({}) not found.",
                        res::font::Set::TekoSemibold,
                        res::font::Set::TekoSemibold.file_for()
                    )))
                }?,
        })
    }

    impl Index<res::font::Set> for Set {
        type Output = str;

        fn index(&self, s: res::font::Set) -> &Self::Output {
            use res::font::Set::*;
            match s {
                TonicLiver      => &self.tonic_liver,
                TekoSemibold    => &self.teko,
            }
        }
    }
}


pub struct All {
    pub images  : img::Set,
    pub fonts   : font::Set,
}

impl All {
    pub fn new(dict: &mut HashMap<res::All, Loaded>) -> Result<All, AssetRetrievalError> {
        Ok(All {
            images  : img::new_image_set(dict)?,
            fonts   : font::new_font_set(dict)?,
        })
    }
}
