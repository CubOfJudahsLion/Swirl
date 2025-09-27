
//! Purpose :   A rectangle type and some accompanying utilities.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   2021-02-23 10:00
//! Modified:   Sat 06/11/2022 06:44 PM


use std::convert::From;
use std::default::Default;
use std::ops::{Add, Sub, Mul, Div, MulAssign, DivAssign};
use std::fmt::{self, Display, Formatter};

use super::super::numeric::*;
use super::dimensions::Dimensions;
use super::point::Point;
use super::transform::Transform;


/// A simple rectangle type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Rect<N> {
    pub top     : N,
    pub left    : N,
    pub bottom  : N,
    pub right   : N,
}


/// Bound calculation trait for a rectangle. It is separated from the implementation for
/// `Rect` as discrete areas (which are denote the right-most pixel as to be included as
/// part of the area work differently from continuous ones (which, from mathematical rigor,
/// don't.) Thus, it needs to be implemented differently for both types of numbers. The
/// trait simply declares an `INCLUSION` constant: it should be **0** for continuous numbers
/// (i.e., floats), and **1** for discrete ones (integers.)
pub trait RectBounds
    where   Self: Sized
{
    const INCLUSION: Self;
}

macro_rules! impl_rect_bounds {
    ($type:ident, $inclusion:expr) => {
        impl RectBounds for $type {
            const INCLUSION: $type = $inclusion;
        }
    };
}

impl_rect_bounds!{u8,    1}
impl_rect_bounds!{u16,   1}
impl_rect_bounds!{u32,   1}
impl_rect_bounds!{u64,   1}
impl_rect_bounds!{u128,  1}
impl_rect_bounds!{usize, 1}
impl_rect_bounds!{i8,    1}
impl_rect_bounds!{i16,   1}
impl_rect_bounds!{i32,   1}
impl_rect_bounds!{i64,   1}
impl_rect_bounds!{i128,  1}
impl_rect_bounds!{isize, 1}
impl_rect_bounds!{f32,   0.0}
impl_rect_bounds!{f64,   0.0}


impl<N: Default> Default for Rect<N> {
    fn default() -> Self {
        Rect {
            top     : N::default(),
            left    : N::default(),
            bottom  : N::default(),
            right   : N::default(),
        }
    }
}


impl<N: Copy> Rect<N> {
    /// Returns the corners of the rectangle as point coordinates,
    /// using clockwise order from the top-left, yielding:
    /// [top-left, top-right, bottom-right, bottom-left].
    pub fn clockwise_corners(&self) -> [Point<N>; 4] {
        [   Point::new(self.left,  self.top)
        ,   Point::new(self.right, self.top)
        ,   Point::new(self.right, self.bottom)
        ,   Point::new(self.left,  self.bottom)
        ]
    }

    /// Returns the corners of the rectangle as point coordinates,
    /// using counter-clockwise order from the top-right, yielding:
    /// [top-right, top-left, bottom-left, bottom-right].
    pub fn counterclockwise_corners(&self) -> [Point<N>; 4] {
        [   Point::new(self.right, self.top)
        ,   Point::new(self.left,  self.top)
        ,   Point::new(self.left,  self.bottom)
        ,   Point::new(self.right, self.bottom)
        ]
    }

    /// A function to map all components of a rectangle
    /// into another type using a function.
    pub fn map<O, F>(&self, mut f: F) -> Rect<O>
        where   O   :   From<N> + Copy,
                F   :   FnMut(N) -> O
    {
        Rect {
            top     : f(self.top),
            left    : f(self.left),
            bottom  : f(self.bottom),
            right   : f(self.right),
        }
    }

    /// Maps into a another type, taking into account differences in inclusion.
    pub fn map_type<O>(&self) -> Rect<O>
        where   N   :   Numeric
                    +   From<i16>
                    +   RectBounds
                    +   Add<Output=N>,
                O   :   Numeric
                    +   From<i16> + From<N>
                    +   RectBounds
                    +   Sub<Output=O>
    {
        Rect::<O>::new_by_dim(
            <O as From<N>>::from(self.top),
            <O as From<N>>::from(self.left),
            <O as From<N>>::from(self.height()), // + N::INCLUSION) - O::INCLUSION,
            <O as From<N>>::from(self.width())  //+ N::INCLUSION) - O::INCLUSION
        )
    }

    /// Maps into a another type like `map_type`, for conversions that might fail.
    pub fn try_map_type<O>(&self) -> Result<Rect<O>, InsufficientPrecisionError<N>>
        where   N   :   Numeric
                    +   From<i16>
                    +   RectBounds
                    +   Add<Output=N>,
                O   :   Numeric
                    +   From<i16> + TryAsFrom<N>
                    +   RectBounds
                    +   Sub<Output=O>
    {
        Ok(Rect::<O>::new_by_dim(
            <O as TryAsFrom<N>>::try_as_from(self.top)?,
            <O as TryAsFrom<N>>::try_as_from(self.left)?,
            <O as TryAsFrom<N>>::try_as_from(self.height() + N::INCLUSION)? - O::INCLUSION,
            <O as TryAsFrom<N>>::try_as_from(self.width()  + N::INCLUSION)? - O::INCLUSION
        ))
    }
}


impl<N> Rect<N>
    where N :   Numeric + From<i16> + PartialOrd
            +   RectBounds
            +   Add<Output=N> + Sub<Output=N>
            +   Copy
{
    pub fn new(top: N, left: N, bottom: N, right: N) -> Self {
        let (top, bottom)   = if top  <= bottom { (top,  bottom) } else { (bottom, top) };
        let (left, right)   = if left <= right  { (left, right) }  else { (right,  left) };
        Rect {top, left, bottom, right,}
    }

    pub fn new_by_dim(top: N, left: N, height: N, width: N) -> Self {
        let zero            =   <N as From<i16>>::from(0);
        let (top, bottom)   =   if height < zero {
                                    (top  + height + N::INCLUSION, top)
                                }
                                else {
                                    (top,  top  + height - N::INCLUSION)
                                };
        let (left, right)   =   if width  < zero {
                                    (left + width + N::INCLUSION, left)
                                }
                                else {
                                    (left, left + width - N::INCLUSION)
                                };
        Rect {top, left, bottom, right,}
    }

    pub fn new_enclose_all(pts: &[Point<N>]) -> Option<Rect<N>> {
        match pts.len() {
            0   => None,
            _   => {
                let [top, left, bottom, right]  = pts
                    .iter()
                    .fold(
                        [N::MAX, N::MAX, N::MIN, N::MIN],
                        |[top, left, bottom, right], &Point{x, y}| {
                            [   if y < top    { y } else { top }
                            ,   if x < left   { x } else { left }
                            ,   if y > bottom { y } else { bottom }
                            ,   if x > right  { x } else { right }
                            ]
                        }
                    );
                Some(Rect {top, left, bottom, right})
            }
        }
    }

    pub fn intersect(&self, other: &Rect<N>) -> Option<Rect<N>> {
        let left    = if self.left   <= other.left   { other.left }   else { self.left };
        let right   = if self.right  >= other.right  { other.right }  else { self.right };
        let top     = if self.top    <= other.top    { other.top }    else { self.top };
        let bottom  = if self.bottom >= other.bottom { other.bottom } else { self.bottom };
        if right < left || bottom < top {
            None
        }
        else {
            Some(Rect {left, right, top, bottom,})
        }
    }

    pub fn include(&self, other: &Rect<N>) -> Rect<N> {
        Rect {
            left    : if self.left      < other.left    { self.left }   else { other.left },
            right   : if self.right     > other.right   { self.right }  else { other.right },
            top     : if self.top       < other.top     { self.top }    else { other.top },
            bottom  : if self.bottom    > other.bottom  { self.bottom } else { other.bottom },
        }
    }

    pub fn contains(&self, pt: &Point<N>) -> bool {
            self.left <= pt.x && pt.x <= self.right
        &&  self.top  <= pt.y && pt.y <= self.bottom
    }

    pub fn width(&self) -> N { self.right - self.left + N::INCLUSION }

    pub fn set_width(&mut self, width: N) -> bool {
        if width >= N::INCLUSION {
            self.right = self.left + width - N::INCLUSION;
            true
        }
        else {
            false
        }
    }

    pub fn height(&self) -> N { self.bottom - self.top + N::INCLUSION }

    pub fn set_height(&mut self, height: N) -> bool {
        if height >= N::INCLUSION {
            self.bottom = self.top + height - N::INCLUSION;
            true
        }
        else {
            false
        }
    }

    pub fn dimensions(&self) -> Dimensions<N> {
        Dimensions {width: self.width(), height: self.height(),}
    }

    pub fn set_dimensions(&mut self, dims: &Dimensions<N>) -> bool {
            self.set_height(dims.height)
        &&  self.set_width(dims.width)
    }
}


macro_rules! rect_mul {
    ($ty:ty) => {
        impl<N: Mul<Output=N> + Copy> Mul<N> for $ty {
            type Output = Rect<N>;
            fn mul(self, scale: N) -> Self::Output {
                Rect {
                    top     : self.top      * scale,
                    left    : self.left     * scale,
                    bottom  : self.bottom   * scale,
                    right   : self.right    * scale,
                }
            }
        }
    };
}

rect_mul!{ Rect<N>}
rect_mul!{&Rect<N>}


impl<N: MulAssign + Copy> MulAssign<N> for Rect<N> {
    fn mul_assign(&mut self, scale: N) {
        self.top     *= scale;
        self.left    *= scale;
        self.bottom  *= scale;
        self.right   *= scale;
    }
}


macro_rules! rect_div {
    ($ty:ty) => {
        impl<N: Div<Output=N> + Copy> Div<N> for $ty {
            type Output = Rect<N>;
            fn div(self, scale: N) -> Self::Output {
                Rect {
                    top     : self.top      / scale,
                    left    : self.left     / scale,
                    bottom  : self.bottom   / scale,
                    right   : self.right    / scale,
                }
            }
        }
    };
}

rect_div!{ Rect<N>}
rect_div!{&Rect<N>}


impl<N: DivAssign + Copy> DivAssign<N> for Rect<N> {
    fn div_assign(&mut self, scale: N) {
        self.top     /= scale;
        self.left    /= scale;
        self.bottom  /= scale;
        self.right   /= scale;
    }
}


macro_rules! add_sub_rect_point {
    ($type_par:ident, $rect:ty, $point:ty) => {
        impl<$type_par> Add<$point> for $rect
            where   $type_par: Add<Output=$type_par> + Copy
        {
            type Output = Rect<$type_par>;

            fn add(self, pt: $point) -> Self::Output {
                Rect {
                    top     : self.top      + pt.y,
                    left    : self.left     + pt.x,
                    bottom  : self.bottom   + pt.y,
                    right   : self.right    + pt.x,
                }
            }
        }

        impl<$type_par> Sub<$point> for $rect
            where   $type_par: Sub<Output=$type_par> + Copy
        {
            type Output = Rect<$type_par>;

            fn sub(self, pt: $point) -> Self::Output {
                Rect {
                    top     : self.top      - pt.y,
                    left    : self.left     - pt.x,
                    bottom  : self.bottom   - pt.y,
                    right   : self.right    - pt.x,
                }
            }
        }
    };
}

add_sub_rect_point!{T,  Rect<T>,  Point<T>}
add_sub_rect_point!{T, &Rect<T>,  Point<T>}
add_sub_rect_point!{T,  Rect<T>, &Point<T>}
add_sub_rect_point!{T, &Rect<T>, &Point<T>}


macro_rules! mul_transform {
    (@left_type v)              => {  Rect<N> };
    (@left_type r)              => { &Rect<N> };
    (@right_type v)             => {  Transform<N> };
    (@right_type r)             => { &Transform<N> };
    (@rhs_expr v $var:ident)    => { &$var };
    (@rhs_expr r $var:ident)    => {  $var };
    ($lty:ident, $rty:ident)    => {
        impl<N> Mul<mul_transform!(@right_type $rty)> for mul_transform!(@left_type $lty)
            where   N   :   Add<Output=N> + Mul<Output=N> + Copy
        { 
            type Output = [Point<N>; 4];

            fn mul(self, rhs: mul_transform!(@right_type $rty)) -> Self::Output {
                let mut corners_    = self.clockwise_corners();
                for i in 0 .. 4 {
                    corners_[i] *= mul_transform!(@rhs_expr $rty rhs);
                }
                corners_
            }
        }
    };
}

mul_transform!{v, v}
mul_transform!{v, r}
mul_transform!{r, v}
mul_transform!{r, r}


impl<N: Display> Display for Rect<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Rect{{top:{}, left:{}, bottom:{}, right:{}}}",
            self.top,
            self.left,
            self.bottom,
            self.right
        )
    }
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    #[wasm_bindgen_test]
    fn defaults() {
        let rect1 = Rect::<f32>::default();
        assert_eq!(
            rect1,
            Rect {
                top     : f32::default(),
                left    : f32::default(),
                bottom  : f32::default(),
                right   : f32::default(),
            }
        );
    }

    #[wasm_bindgen_test]
    fn intersections_and_dims() {
        let rect1       = Rect::<f32>::new(10.0, 10.0, 30.0, 30.0);
        let mut rect2   = Rect::<f32>::new(20.0, 20.0, 40.0, 40.0);
        let mut recti   = rect1.intersect(&rect2);
        assert_eq!(recti, Some(Rect::new(20.0, 20.0, 30.0, 30.0)), "Intersection test 1.");
        rect2           = Rect::new(0.0, 0.0, 20.0, 15.0);
        recti           = rect1.intersect(&rect2);
        assert_eq!(recti, Some(Rect::new(10.0, 10.0, 20.0, 15.0)), "Intersection test 2.");
        assert_eq!(Some(5.0), recti.clone().map(|r| r.width()), "Intersecting width test.");
        assert_eq!(Some(10.0), recti.clone().map(|r| r.height()), "Intersecting height test.");
        rect2           = Rect::new(20.0, 50.0, 60.0, 60.0);
        recti           = rect1.intersect(&rect2);
        assert_eq!(recti, None);
        rect2           = Rect::new(50.0, 20.0, 60.0, 60.0);
        recti           = rect1.intersect(&rect2);
        assert_eq!(recti, None);
        rect2           = Rect::new(0.0, 0.0, 5.0, 5.0);
        recti           = rect1.intersect(&rect2);
        assert_eq!(None, recti.clone().map(|r| r.width()), "Non-intersecting width test.");
        assert_eq!(None, recti.map(|r| r.height()), "Non-intersecting height test.");
    }

    #[wasm_bindgen_test]
    fn point_containment() {
        let r = Rect::<f32>::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains(&Point::new(0.0, 0.0)));
        assert!(r.contains(&Point::new(10.0, 10.0)));
        assert!(r.contains(&Point::new(0.001, 9.999)));
        assert!(!r.contains(&Point::new(10.01, 10.01)));
        assert!(!r.contains(&Point::new(-0.0001, -0.0001)));
    }

    #[wasm_bindgen_test]
    fn dim_setters() {
        let mut r   = Rect::<i16>::new(5, 7, 15, 20);
        r.set_width(20);
        assert_eq!(r.width(), 20);
        r.set_height(30);
        assert_eq!(r.height(), 30);
        assert_eq!(r, Rect::new(5, 7, 34, 26));
        let mut r2  = Rect::<f32>::new(5.0, 7.0, 15.0, 20.0);
        r2.set_width(20.0);
        assert_eq!(r2.width(), 20.0);
        r2.set_height(30.0);
        assert_eq!(r2.height(), 30.0);
        assert_eq!(r2, Rect::new(5.0, 7.0, 35.0, 27.0));
    }

    #[wasm_bindgen_test]
    fn maps() {
        let rect1   = Rect::<i16>::new(1, 2, 3, 4);
        let rect2   = rect1.map(|n| n + 1);
        assert_eq!(rect2, Rect::new(2, 3, 4, 5));
        let rect3   = rect1.map_type::<f32>();
        assert_eq!(rect3, Rect::<f32>::new(1.0f32, 2.0, 4.0, 5.0));
    }

    #[wasm_bindgen_test]
    fn inclusions() {
        let rect1   = Rect::<i16>::new(0, 0, 10, 20);
        let mut rect2 = Rect::new(5, 5, 7, 7);
        assert_eq!(rect1.include(&rect2), rect1);
        rect2       = Rect::new(-5, -5, 30, 40);
        assert_eq!(rect1.include(&rect2), rect2);
    }

    #[wasm_bindgen_test]
    fn scaling() {
        let mut rect1 = Rect::<i16>::new(30, 60, 90, 120);
        assert_eq!(&rect1 * 2, Rect::new(60, 120, 180, 240), "*2");
        rect1 *= 5;
        assert_eq!(rect1, Rect::new(150, 300, 450, 600), "*=5");
        rect1 = rect1 / 75;
        assert_eq!(rect1, Rect::new(2, 4, 6, 8), "/75");
        rect1 /= 2;
        assert_eq!(rect1, Rect::new(1, 2, 3, 4), "/=2");
    }

    use super::super::point::*;
    #[wasm_bindgen_test]
    fn point_addition_subtraction() {
        let rect    = Rect::<f32>::new(10.0, 10.0, 20.0, 20.0);
        let point   = Point::<f32>::new(10.0, 20.5);
        assert_eq!(&rect + &point, Rect::new(30.50, 20.0, 40.5, 30.0));
        assert_eq!(&rect - &point, Rect::new(-10.5, 0.0, -0.5, 10.0));
    }

    #[wasm_bindgen_test]
    fn new_by_dim() {
        assert_eq!(Rect::<i16>::new(5, 5, 9, 9), Rect::new_by_dim(5, 5, 5, 5));
        assert_eq!(Rect::<f32>::new(5., 5., 10., 10.), Rect::new_by_dim(5., 5., 5., 5.));
    }

    #[wasm_bindgen_test]
    fn corner_ordering() {
        let r = Rect::<i16>::new(1, 2, 3, 4);
        assert_eq!(r.clockwise_corners(), [Point::new(2, 1), Point::new(4, 1), Point::new(4, 3), Point::new(2, 3)]);
        assert_eq!(r.counterclockwise_corners(), [Point::new(4, 1), Point::new(2, 1), Point::new(2, 3), Point::new(4, 3)]);
    }

    #[wasm_bindgen_test]
    fn points_enclosing() {
        const COUNT: usize = 10;
        let mut v = Vec::<Point<f64>>::with_capacity(COUNT);
        for _i in 0 .. COUNT {
            v.push(Point::new(1000.0 * js_sys::Math::random(), 1000.0 * js_sys::Math::random()));
        }
        let r = Rect::new_enclose_all(&v);
        assert!(
            r   .map(|r| v
                    .iter()
                    .all(|&Point {x, y}|
                            r.left <= x && x <= r.right
                        &&  r.top  <= y && y <= r.bottom
                    )
                )
                .unwrap_or(false)
        );
    }

    #[wasm_bindgen_test]
    fn transform() {
        let r = Rect::<i16>::new(1, 2, 11, 12);
        let t = Transform::<i16>::new_from_set_transform_args(1, 2, 3, 4, 5, 6);
        assert_eq!(r * t, [Point::new(10, 14), Point::new(20, 34), Point::new(50, 74), Point::new(40, 54)]);
    }
}
