//! # Module        :   Timer
//! - **Purpose**   :   A restartable timer with millisecond precision.
//! - **Version**   :   0.2.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Tue 04/26/2022 08:35 PM
//! - **Modified**  :   ~


use js_sys::Date;


/// A simple millisecond-counting timer.
pub struct Timer(u64);

impl Timer {
    /// Creates a new idle timer. Set it
    /// running by calling `start()`.
    #[inline(always)]
    pub fn new() -> Self { Timer(Date::now() as u64) }

    /// Amount of ticks (milliseconds) that have
    /// transpired since the timer was started.
    #[inline(always)]
    pub fn ticks(&self) -> u64 { Date::now() as u64 - self.0 }

    /// Set the number of ticks to zero, whether running or stopped.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.0 = Date::now() as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::events::generic::tests::timer_future;
    use futures::FutureExt;


    #[wasm_bindgen_test]
    async fn timer() {
        let mut timer = Timer::new();
        timer_future(4).expect("Failed awaiting 6 millis").fuse().await.expect("await failure");
        assert!(timer.ticks() > 0);
        timer.reset();
        assert_eq!(timer.ticks(), 0);
    }
}
