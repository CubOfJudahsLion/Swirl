//! # Module        :   _System_
//! - **Purpose**   :   Utilities to inspect system attributes
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Mon 06/13/2022 03:15 PM
//! - **Modified**  :   ~


use wasm_bindgen::{intern, JsValue};
use js_sys::RegExp;
use lazy_static::lazy_static;
use super::html::get_window;


/// Obtains the 'user agent'of the browser.
pub fn user_agent() -> Result<String, JsValue> {
    get_window()?.navigator().user_agent()
}


//  Some interned strings to speed such comparisons up.
lazy_static!{
    static ref CASE_INSENSITIVE : &'static str  = intern("i");
    static ref SAFARI           : &'static str  = intern("Safari");
    static ref FIREFOX          : &'static str  = intern("Firefox");
    static ref CHROME           : &'static str  = intern("Chrome");
    static ref BLACKBERRY       : &'static str  = intern("BlackBerry");
    static ref IOS              : &'static str  = intern("iP(hone|[ao]d)");
    static ref ANDROID          : &'static str  = intern("Android");
    static ref OPERA_MINI       : &'static str  = intern("Opera Mini");
    static ref MOBILE           : &'static str  = intern("Android|BlackBerry|iP([oa]d|hone)|Opera Mini");
}

pub fn is_safari() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&SAFARI, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}

pub fn is_firefox() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&FIREFOX, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}

pub fn is_chrome() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&CHROME, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}

pub fn is_blackberry() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&BLACKBERRY, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}

pub fn is_ios() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&IOS, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}

pub fn is_android() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&ANDROID, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}

pub fn is_opera_mini() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&OPERA_MINI, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}

pub fn is_mobile() -> bool {
    user_agent()
        .map(|ua| RegExp::new(&MOBILE, &CASE_INSENSITIVE).test(&ua))
        .unwrap_or(false)
}
