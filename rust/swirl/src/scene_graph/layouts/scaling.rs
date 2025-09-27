//! # Module        :   _Scaling_
//! - **Purpose**   :   Scale declarations for layouts.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Fri 17 Jun 2022 09:34:59 PM CST
//! - **Modified**  :   Sat 06/18/2022 11:10 PM


use std::default::Default;


/// Different ways to indicate scaling (or its absence.)
#[derive(Clone, Debug)]
pub enum LayoutScaling {
    NoScaling,
    Scaling(f32, f32),
    UniformScaling(f32),
}


impl Default for LayoutScaling {
    #[inline(always)]
    fn default() -> Self { LayoutScaling::NoScaling }
}


impl LayoutScaling {
    /// Obtains scale spec in a uniform format.
    pub fn get(&self) -> Option<(f32, f32)> {
        use LayoutScaling::*;
        match self {
            &NoScaling          => None,
            &Scaling(x, y)      => Some((x, y)),
            &UniformScaling(s)  => Some((s, s)),
        }
    }

    pub fn x(&self) -> Option<f32> {
        use LayoutScaling::*;
        match self {
            &NoScaling          => None,
            &Scaling(x, _)      => Some(x),
            &UniformScaling(s)  => Some(s),
        }
    }

    #[inline(always)]
    pub fn x1(&self) -> f32 {
        self.x().unwrap_or(1.)
    }

    pub fn y(&self) -> Option<f32> {
        use LayoutScaling::*;
        match self {
            &NoScaling          => None,
            &Scaling(_, y)      => Some(y),
            &UniformScaling(s)  => Some(s),
        }
    }

    #[inline(always)]
    pub fn y1(&self) -> f32 {
        self.y().unwrap_or(1.)
    }

    /// Gets the scale, but returns 1 for
    /// both dimensions if `NoScaling` is
    /// indicated.
    pub fn get_def1(&self) -> (f32, f32) {
        use LayoutScaling::*;
        match self {
            &NoScaling          => (1., 1.),
            &Scaling(x, y)      => (x, y),
            &UniformScaling(s)  => (s, s),
        }
    }
}
