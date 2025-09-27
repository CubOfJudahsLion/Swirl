//! Module  :   Dimensions 
//! Purpose :   A type designing only the dimensions of an object, i.e.,
//!             its `width` and `height`, plus operations on it.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Mon 04/11/2022 03:44 PM
//! Modified:   Tue 04/26/2022 11:39 AM


use std::cmp::{PartialEq, Eq};
use std::default::Default;
use std::ops::{Add, AddAssign};

use crate::math::numeric::Numeric;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dimensions<N> {
    pub width   : N,
    pub height  : N,
}

impl<N: Copy> Dimensions<N> {
    #[inline(always)]
    pub fn new(width: N, height: N) -> Self {
        Dimensions {width, height,}
    }

    pub fn map_ref<O, F: FnMut(N) -> O>(&self, f: &mut F) -> Dimensions<O> {
        Dimensions {
            width   : f(self.width),
            height  : f(self.height),
        }
    }

    pub fn map<O, F: FnMut(N) -> O>(self, f: &mut F) -> Dimensions<O> {
        Dimensions {
            width   : f(self.width),
            height  : f(self.height),
        }
    }

    pub fn grow(&self, width_growth: N, height_growth: N) -> Dimensions<N>
        where   N: Add<Output=N>
    {
        Dimensions {
            width   : self.width  + width_growth,
            height  : self.height + height_growth,
        }
    }

    pub fn self_grow(&mut self, width_growth: N, height_growth: N)
        where   N: AddAssign<N>
    {
        self.width  += width_growth;
        self.height += height_growth;
    }
    
    pub fn max_both(&self, other: &Self) -> Self
        where   N: Numeric
    {
        Dimensions {
            width   : if self.width > other.width { self.width } else { other.width },
            height  : if self.height > other.height { self.height } else { other.height },
        }
    }
}

impl<N: Default> Default for Dimensions<N> {
    fn default() -> Self {
        Dimensions {
            width   : N::default(),
            height  : N::default(),
        }
    }
}


macro_rules! add_dims {
    (@ty v) => {  Dimensions<N> };
    (@ty r) => { &Dimensions<N> };
    ($l:ident, $r:ident) => {
        impl<N: Add<Output=N> + Copy> Add<add_dims!(@ty $r)> for add_dims!(@ty $l) {
            type Output = Dimensions<N>;
            fn add(self, other: add_dims!(@ty $r)) -> Self::Output {
                Dimensions {
                    width   : self.width  + other.width,
                    height  : self.height + other.height,
                }
            }
        }
    };
}

add_dims!{r, r}
add_dims!{r, v}
add_dims!{v, r}
add_dims!{v, v}


macro_rules! add_assign_dims {
    (@ty v) => {  Dimensions<M> };
    (@ty r) => { &Dimensions<M> };
    ($r:ident) => {
        impl<M: Copy, N: AddAssign<M> + Copy> AddAssign<add_assign_dims!(@ty $r)> for Dimensions<N> {
            fn add_assign(&mut self, other: add_assign_dims!(@ty $r)) {
                self.width  += other.width;
                self.height += other.height;
            }
        }
    };
}

add_assign_dims!{r}
add_assign_dims!{v}


#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn dimension_default_equality_and_map() {
        let mut d1 = Dimensions::<i16>::default();
        assert_eq!(d1, Dimensions {width: i16::default(), height: i16::default(),});
        d1.width = 3;
        d1.height = 5;
        assert_eq!(d1.map_ref(&mut |n| n + 3), Dimensions {width: 6, height: 8,});
        assert_eq!(d1.map(&mut |n| n as f32 * 0.5), Dimensions::<f32> {width: 1.5, height: 2.5,});
    }

    #[wasm_bindgen_test]
    fn dimension_grow_and_add() {
        let mut d1 = Dimensions::<i16> {width: 1, height: 2,};
        assert_eq!(d1.grow(3, 4), Dimensions {width: 4, height: 6,});
        d1.self_grow(5, 6);
        assert_eq!(d1, Dimensions {width: 6, height: 8,});
        let d2 = Dimensions::<i16> {width: 7, height: 8,};
        assert_eq!(&d1        + &d2,        Dimensions {width: 13, height: 16,});
        assert_eq!(d1.clone() + &d2,        Dimensions {width: 13, height: 16,});
        assert_eq!(&d1        + d2.clone(), Dimensions {width: 13, height: 16,});
        assert_eq!(d1.clone() + d2,         Dimensions {width: 13, height: 16,});
        let mut d3 = Dimensions::<i16> {width: 9, height: 10,};
        d1 += &d3;
        assert_eq!(d1, Dimensions {width: 15, height: 18,});
        d3.width = 11;
        d3.height = 12;
        d1 += d3;
        assert_eq!(d1, Dimensions {width: 26, height: 30,});
    }
}
