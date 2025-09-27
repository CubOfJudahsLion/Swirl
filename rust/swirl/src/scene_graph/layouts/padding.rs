//! # Module        :   _Padding_
//! - **Purpose**   :   Represents -a request for blank space
//!                     around an object, with varying degrees
//!                     of specifity.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Mon 04/11/2022 09:29 PM
//! - **Modified**  :   Thu 06/23/2022 01:40 PM


use crate::math::geom::rect::Rect;
use crate::math::geom::dimensions::Dimensions;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Padding {
    NoPadding,
    Uniform(i16),
    Directional(Dimensions<i16>),
    EachEdge(Rect<i16>),
}

impl Padding {
    pub fn top(&self) -> i16 {
        use Padding::*;
        match self {
            EachEdge(rect)                          => rect.top,
            Directional(Dimensions {height, ..})    => *height,
            Uniform(n)                              => *n,
            NoPadding                               => 0,
        }
    }

    pub fn left(&self) -> i16 {
        use Padding::*;
        match self {
            EachEdge(rect)                      => rect.left,
            Directional(Dimensions {width, ..}) => *width,
            Uniform(n)                          => *n,
            NoPadding                           => 0,
        }
    }

    pub fn bottom(&self) -> i16 {
        use Padding::*;
        match self {
            EachEdge(rect)                          => rect.bottom,
            Directional(Dimensions {height, ..})    => *height,
            Uniform(n)                              => *n,
            NoPadding                               => 0,
        }
    }

    pub fn right(&self) -> i16 {
        use Padding::*;
        match self {
            EachEdge(rect)                      => rect.right,
            Directional(Dimensions {width, ..}) => *width,
            Uniform(n)                          => *n,
            NoPadding                           => 0, 
       }
    }

    pub fn as_rect(&self) -> Rect<i16> {
        use Padding::*;
        match self {
            EachEdge(rect)                              =>
                rect.clone(),
            Directional(Dimensions {width, height,})    =>
                Rect {
                    top     : *height,
                    left    : *width,
                    bottom  : *height,
                    right   : *width,
                },
            Uniform(n)                                  =>
                Rect {
                    top     : *n,
                    left    : *n,
                    bottom  : *n,
                    right   : *n,
                },
            NoPadding                                   =>
                Rect::<i16>::default(),
        }
    }
}

impl Default for Padding {
    #[inline(always)]
    fn default() -> Self {
        Padding::NoPadding
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn default_nopadding() {
        let padding = Padding::default();
        assert_eq!(padding, Padding::NoPadding);
        assert_eq!(padding.top(), 0);
        assert_eq!(padding.left(), 0);
        assert_eq!(padding.bottom(), 0);
        assert_eq!(padding.right(), 0);
        assert_eq!(padding.as_rect(), Rect {top: 0, left: 0, bottom: 0, right: 0,});
    }

    #[wasm_bindgen_test]
    fn uniform() {
        let padding = Padding::Uniform(10);
        assert_eq!(padding.top(), 10);
        assert_eq!(padding.left(), 10);
        assert_eq!(padding.bottom(), 10);
        assert_eq!(padding.right(), 10);
        assert_eq!(padding.as_rect(), Rect {top:10, left:10, bottom:10, right:10,});
    }

    #[wasm_bindgen_test]
    fn directional() {
        let padding = Padding::Directional(Dimensions {height: 5, width:15,});
        assert_eq!(padding.top(), 5);
        assert_eq!(padding.left(), 15);
        assert_eq!(padding.bottom(), 5);
        assert_eq!(padding.right(), 15);
        assert_eq!(padding.as_rect(), Rect{top:5, left:15, bottom:5, right:15});
    }

    #[wasm_bindgen_test]
    fn each_edge() {
        let dim_rect    = Rect::<i16> {top: 8, left: 4, bottom: 12, right: 40};
        let padding     = Padding::EachEdge(dim_rect.clone());
        assert_eq!(padding.top(), 8);
        assert_eq!(padding.left(), 4);
        assert_eq!(padding.bottom(), 12);
        assert_eq!(padding.right(), 40);
        assert_eq!(padding.as_rect(), dim_rect);
    }
}
