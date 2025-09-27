//! Module  :   Rust-Promise
//! Purpose :   A re-implementation of promises
//!             natively in Rust.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 15 Feb 2022 09:04:11 PM CST
//! Modified:   ~


use std::cell::RefCell;
use std::rc::Rc;


enum PromiseState<ResolveT, RejectT> {
    Pending,
    Resolved(ResolveT),
    Rejected(RejectT),
    Consumed,
}

impl<ResolveT, RejectT> PromiseState<ResolveT, RejectT> {
    fn is_pending(&self) -> bool {
        match self {
            &PromiseState::Pending  => true,
            _                       => false,
        }
    }
}


pub struct RPromise<ResolveT, RejectT>(
    Rc<RefCell<PromiseState<ResolveT, RejectT>>>
);

impl<ResolveT, RejectT> RPromise<ResolveT, RejectT> {
    pub fn new(cb: &mut dyn FnMut(Box<dyn FnMut(ResolveT)>, Box<dyn FnMut(RejectT)>)) -> Self {
        let this    = Rc::new(RefCell::new(PromiseState::Pending));
        let resolve = {
            let this    = this.clone();
            move |resolved_value: ResolveT| {
                let pending = this.0.borrow().is_pending();
                if pending {
                    this.0.replace(PromiseState::Resolved(resolved_value));
                }
            }
        };
        let reject  = {
            let this    = this.clone();
            move |rejected_value: RejectT| {
                let pending = this.0.borrow().is_pending();
                if pending {
                    this.0.replace(PromiseState::Rejected(rejected_value));
                };
            }
        };
        this
    }

    pub fn 
}
