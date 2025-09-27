//! Module  :   Result-as-promise
//! Purpose :   Transforms a `Result` object with `JsValue` errors
//!             into a `Promise`
//! Version :   0.2.0
//! Author  :   Alex Feterman
//! Created :   Tue 08 Mar 2022 09:01:54 AM CST
//! Modified:   ~


use js_sys::Promise;


pub trait IntoPromise {
    fn into_promise(self) -> Promise;
    fn solve_promise(self, resolve: Function, reject: Function);
}

impl<T> IntoPromise<T> for Result<T, JsValue> {
    fn into_promise(self) -> Promise {
        match self {
            Ok(_)   => 
        }
    }
}
