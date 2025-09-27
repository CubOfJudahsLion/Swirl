//! # Utilities
//!
//! Functions and variables for 
//!
//! - *Version* : 0.1.0
//! - *Author*  : Alex Feterman
//! - *Created* : Fri 10/22/2021 07:37 AM
//! - *Modified*: Tue 04 Jan 2022 11:10:50 PM CST


use std::string::ToString;
use regex::{Captures, Regex};
use lazy_static::lazy_static;


lazy_static!{
    pub static ref FONT_EXTENSIONS  : Regex =   Regex::new(r"\.(?i)woff$").unwrap();
    pub static ref IMG_EXTENSIONS   : Regex =   Regex::new(r"\.(?i:svg|jpe?g|png)$").unwrap();
    pub static ref SND_EXTENSIONS   : Regex =   Regex::new(r"\.(?i:mp3|ogg)$").unwrap();
}


/// Changes a PascalCase name to a snake_case one.
pub fn pascal_to_snake(name: &str) -> String {
    lazy_static! {
        static ref WORD_LIMIT: Regex = Regex::new("(?P<lower>[a-z0-9])(?P<upper>[A-Z])").unwrap();
    }
    WORD_LIMIT
        .replace_all(
            &name,
            |cap: &Captures|
                cap["lower"].to_string() + "_" + &cap["upper"]
        )
        .to_lowercase()
}
