//! # Pretty-Printer
//!
//! Generates the resulting code out of the IR.
//!
//! - *Version* : 0.2.0
//! - *Author*  : Alex Feterman
//! - *Created* : Tue 10/26/2021 12:53 PM
//! - *Modified*: Wed 05 Jan 2022 09:13:24 AM CST


use std::fmt::Debug;

pub fn pretty_print<D: Debug>(d: D) -> String { format!("{:?}", d) }
