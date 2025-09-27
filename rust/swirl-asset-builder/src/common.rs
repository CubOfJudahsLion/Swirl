//! # Common
//!
//! Shared definitions useful to all modules.
//!
//! - *Version* : 0.1.1
//! - *Author*  : Alex Feterman
//! - *Created* : Fri 10/22/2021 08:07 AM
//! - *Modified*: Wed 05 Jan 2022 11:20:19 AM CST


use std::cell::RefCell;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;


/// Characterizes the type of media for a variant: image, font, etc.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MediaType {
    Font,
    Image,
    Sound,
}


/// A shared smart pointer.
#[derive(Clone)]
pub struct Shared<T>(Rc<RefCell<T>>);

impl<T> Shared<T> {
    pub fn new(t: T) -> Self { Shared(Rc::new(RefCell::new(t))) }
}

impl<T: Debug> Debug for Shared<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Shared<{:?}>", self.0))
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.as_ptr() }
    }
}

impl<T> DerefMut for Shared<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.as_ptr() }
    }
}
