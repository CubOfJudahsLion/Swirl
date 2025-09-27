//! Module  :   Timer
//! Purpose :   A restartable timer with millisecond precision (which fails.)
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Mon 04/04/2022 11:07 AM
//! Modified:   Tue 04/26/2022 06:32 PM


use std::cell::{Cell, RefCell};
use std::ops::Drop;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Function;
use super::html::get_window;


/// A simple millisecond-counting timer.
pub struct Timer {
    ticks   : Rc<Cell<u64>>,
    interval: Option<(Closure<dyn FnMut()>, i32)>,
    callback: Rc<RefCell<Option<Box<dyn FnMut(u64)>>>>,
}


impl Timer {
    /// Creates a new idle timer. Set it
    /// running by calling `start()`.
    #[inline(always)]
    pub fn new() -> Self {
        Timer {
            ticks   : Rc::new(Cell::new(0)),
            interval: None,
            callback: Rc::new(RefCell::new(None)),
        }
    }

    /// Creates a new running timer.
    /// The parameter `start_asap` implies starting the ticking as soon as
    /// possible instead of awaiting for the next millisecond.
    #[inline]
    pub fn new_start_dyn(start_asap: bool, opt_tick_fn: Option<Box<dyn FnMut(u64)>>) -> Result<Self, JsValue> {
        let mut timer = Self::new();
        timer.start_dyn(start_asap, opt_tick_fn).map(|_| timer)
    }

    /// Simpler version of `new_start_dyn` without optional timer function.
    #[inline(always)]
    pub fn new_start(start_asap: bool) -> Result<Self, JsValue> {
        Self::new_start_dyn(start_asap, None)
    }

    /// Simpler version of `new_start_dyn` without optional timer function.
    #[inline(always)]
    pub fn new_start_with<F: FnMut(u64) + 'static>(start_asap: bool, tick_fn: F) -> Result<Self, JsValue> {
        Self::new_start_dyn(start_asap, Some(Box::new(tick_fn) as Box<dyn FnMut(u64)>))
    }

    /// Amount of ticks (milliseconds) that have
    /// transpired since the timer was started.
    #[inline(always)]
    pub fn ticks(&self) -> u64 { self.ticks.get() }

    /// Set the number of ticks to zero, whether running or stopped.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.ticks.set(0);
    }

    /// Stops a running timer. does nothing if it wasn't
    /// running. Preserves the tick count.
    pub fn stop(&mut self) -> Result<(), JsValue> {
        if let &mut Some((_, intv)) = &mut self.interval {
            get_window()?.clear_interval_with_handle(intv);
            self.interval = None;
        }
        let has_callback    = self.callback.borrow().is_some();
        if has_callback {
            self.callback.replace(None);
        }
        Ok(())
    }

    /// Starts a timer, regardless of its previous state. If it was
    /// running, it's stopped and restarted. In either case, the
    /// tick count is to 0 (discarding previous results.) The function
    /// accepts a parameter for relative immediacy (`start_asap`) which,
    /// if affirmative, indicates that the first timer tick happens as
    /// soon as possible (i.e., after the JS queue is emptied) instead
    /// of after the next millisecond. A second argument (`opt_tick_fn`)
    /// allows users to specify a callback that receives the tick count
    /// (as an `u64`) as parameter.
    pub fn start_dyn(&mut self, start_asap: bool, opt_tick_fn: Option<Box<dyn FnMut(u64)>>) -> Result<(), JsValue> {
        self.stop()?;
        self.ticks.set(0);
        self.callback   = Rc::new(RefCell::new(opt_tick_fn));
        let timer_fn    = {
            let ticks       = self.ticks.clone();
            let callback    = self.callback.clone();
            Closure::wrap(Box::new(move || {
                let curr_ticks = ticks.get();
                callback.borrow_mut().as_mut().map(|callback| callback(curr_ticks));
                ticks.set(curr_ticks + 1);
            }) as Box<dyn FnMut()>)
        };
        let wnd         = get_window()?;
        if start_asap {
            //wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
            //    timer_fn.as_ref().unchecked_ref(),
            //    0
            //)?;
            timer_fn
                .as_ref()
                .unchecked_ref::<Function>()
                .call1(&JsValue::NULL, &JsValue::from_f64(0.))
                .expect_throw("swirl::utils::timer::start_dyn(): Couldn't call timer ASAP.");
            self.ticks.set(1);
        }
        let intv        = wnd
            .set_interval_with_callback_and_timeout_and_arguments_0(
                timer_fn.as_ref().unchecked_ref(),
                1
            )?;
        self.interval   = Some((timer_fn, intv));
        Ok(())
    }

    /// Starts a timer like `start_dyn()`, but assuming no custom timer function.
    #[inline(always)]
    pub fn start(&mut self, start_asap: bool) -> Result<(), JsValue> {
        self.start_dyn(start_asap, None)
    }

    /// Starts a timer like `start_dyn()`. However, this version
    /// is polymorphic over the function type.
    #[inline(always)]
    pub fn start_with<F: FnMut(u64) + 'static>(&mut self, start_asap: bool, tick_fn: F) -> Result<(), JsValue> {
        self.start_dyn(start_asap, Some(Box::new(tick_fn) as Box<dyn FnMut(u64)>))
    }

    /// Tells whether the timer is ticking (`true`) or inactive (`false`.)
    #[inline(always)]
    pub fn is_running(&self) -> bool { self.interval.is_some() }

    /// Indicates whether the timer has a custom callback (i.e., specified
    /// through the first parameters in `new_start_dyn`, `new_start`,
    /// `start_dyn` or `start`.
    #[inline(always)]
    pub fn has_custom_callback(&self) -> bool { self.callback.borrow().is_some() }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use js_sys::Date;
    use web_sys::console;
    use crate::events::generic::tests::timer_future;
    use futures::FutureExt;


    #[wasm_bindgen_test]
    async fn timer() {
        let mut timer = Timer::new();
        assert!(!timer.is_running());
        assert!(!timer.has_custom_callback());
        timer.start(false).expect("Failed to start timer");
        assert!(timer.is_running());
        assert!(!timer.has_custom_callback());
        timer_future(2).expect("Failed awaiting 2 millis").fuse().await.expect("await failure");
        timer.stop().expect("Failed to stop timer");
        assert!(timer.ticks() >= 1);
        timer.reset();
        assert_eq!(timer.ticks(), 0);
        assert!(!timer.is_running());
        let sum = Rc::new(Cell::new(0u64));
        let sum_fn = {
            let sum         = sum.clone();
            let start_time  = Date::now();
            move |tick: u64| {
                sum.set(sum.get() + tick);
                console::info_1(&JsValue::from_str(&format!(
                    "Timer test, tick# {}, millis since start: {}, sum: {}",
                    tick,
                    Date::now() - start_time,
                    sum.get()
                )));
            }
        };
        timer.start_with(true, sum_fn).expect("Failed to start timer with custom timer function");
        assert!(timer.is_running());
        assert!(timer.has_custom_callback());
        timer_future(20).expect("Failed awaiting 20 millis").fuse().await.expect("await failure");
        timer.stop().expect("Failed to stop timer");
        assert!(!timer.is_running());
        assert!(!timer.has_custom_callback());
        assert!(sum.get() >= 6);
    }
}
