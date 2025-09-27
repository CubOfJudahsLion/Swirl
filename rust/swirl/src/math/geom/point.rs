//! Module      :   Point
//! Description :   Bidimensional points implemented with arithmetic operations
//!                 (as complex numbers) for SWIRL.
//! Version     :   0.1.0
//! Author      :   Alexander Feterman-Naranjo <10951848+CubOfJudahsLion@users.noreply.github.com>
//! Created     :   Thu Sep 10 04:02:17 PM CST 2020
//! Modified    :   Fri 21 Jan 2022 10:50:41 AM CST


use std::cmp::{PartialEq, Eq};
use std::convert::From;
use std::default::Default;
use std::fmt::{self, Display, Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::super::numeric::Floating;
use super::Transform;


/// A simple two-dimensional point, to be treated as a complex number.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}


impl<M, N> From<&Point<M>> for Point<N>
    where   N   :   From<M>,
            M   :   Copy
{
    fn from(pm: &Point<M>) -> Self {
        Point {
            x: <N as From<M>>::from(pm.x),
            y: <N as From<M>>::from(pm.y),
        }
    }
}


impl<N: Display> Display for Point<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Point{{x:{}, y:{},}})", self.x, self.y)
    }
}


macro_rules! impl_neg {
    (@for $type:ty) => {
        impl<N: Neg<Output=N> + Copy> Neg for $type {
            type Output = Point<<N as Neg>::Output>;

            fn neg(self) -> Self::Output {
                Point {x: -self.x, y: -self.y}
            }
        }
    };
    () => {
        impl_neg!{@for  Point<N>}
        impl_neg!{@for &Point<N>}
    };
}

impl_neg!{}


macro_rules! impl_add {
    (@for $left_type:ty, $right_type:ty) => {
        impl<N> Add<$right_type> for $left_type
            where   N   :   Add<Output=N> + Copy
        {
            type Output = Point<N>;

            fn add(self, other: $right_type) -> Self::Output {
                Point {x: self.x + other.x, y: self.y + other.y}
            }
        }
    };
    () => {
        impl_add!{@for  Point<N>,  Point<N>}
        impl_add!{@for &Point<N>,  Point<N>}
        impl_add!{@for  Point<N>, &Point<N>}
        impl_add!{@for &Point<N>, &Point<N>}
    };
}

impl_add!{}


macro_rules! impl_add_assign {
    (@for $right_type:ty) => {
        impl<N> AddAssign<$right_type> for Point<N>
            where   N   :   AddAssign<N> + Copy
        {
            fn add_assign(&mut self, other: $right_type) {
                self.x += other.x;
                self.y += other.y;
            }
        }
    };
    () => {
        impl_add_assign!{@for  Point<N>}
        impl_add_assign!{@for &Point<N>}
    };
}

impl_add_assign!{}


macro_rules! impl_sub {
    (@for $left_type:ty, $right_type:ty) => {
        impl<N> Sub<$right_type> for $left_type
            where   N:  Sub<Output=N> + Copy
        {
            type Output = Point<N>;

            fn sub(self, other: $right_type) -> Self::Output {
                Point {x: self.x - other.x, y: self.y - other.y}
            }
        }
    };
    () => {
        impl_sub!{@for  Point<N>,  Point<N>}
        impl_sub!{@for &Point<N>,  Point<N>}
        impl_sub!{@for  Point<N>, &Point<N>}
        impl_sub!{@for &Point<N>, &Point<N>}
    };
}

impl_sub!{}


macro_rules! impl_sub_assign {
    (@for $right_type:ty) => {
        impl<N> SubAssign<$right_type> for Point<N>
            where   N   :   SubAssign<N> + Copy
        {
            fn sub_assign(&mut self, other: $right_type) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }
    };
    () => {
        impl_sub_assign!{@for  Point<N>}
        impl_sub_assign!{@for &Point<N>}
    };
}

impl_sub_assign!{}


macro_rules! impl_mul {
    (@for $left_type:ty, $right_type:ty) => {
        impl<N> Mul<$right_type> for $left_type
            where   N   :   Mul<Output=N> + Add<Output=N> + Sub<Output=N> + Copy
        {
            type Output = Point<N>;

            fn mul(self, other: $right_type) -> Self::Output {
                Point {
                    x: self.x * other.x - self.y * other.y,
                    y: self.x * other.y + self.y * other.x,
                }
            }
        }
    };
    () => {
        impl_mul!{@for  Point<N>,  Point<N>}
        impl_mul!{@for &Point<N>,  Point<N>}
        impl_mul!{@for  Point<N>, &Point<N>}
        impl_mul!{@for &Point<N>, &Point<N>}
    };
}

impl_mul!{}


macro_rules! impl_mul_scalar {
    (@for $type_left:ty) => {
        impl<N> Mul<N> for $type_left
            where   N   :   Mul<Output=N> + Copy
        {
            type Output = Point<N>;

            fn mul(self, other: N) -> Self::Output {
                Point {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }
    };
    () => {
        impl_mul_scalar!{@for  Point<N>}
        impl_mul_scalar!{@for &Point<N>}
    };
}

impl_mul_scalar!{}


impl<N> MulAssign<N> for Point<N>
    where   N   :   MulAssign<N> + Copy
{
    fn mul_assign(&mut self, other: N) {
        self.x *= other;
        self.y *= other;
    }
}


macro_rules! impl_mul_assign {
    (@for $type_right:ty) => {
        impl<N> MulAssign<$type_right> for Point<N>
            where   N   :   Mul<Output=N> + Add<Output=N> + Sub<Output=N> + Copy
        {
            fn mul_assign(&mut self, other: $type_right) {
                let old_x   = self.x;
                self.x      = old_x * other.x - self.y * other.y;
                self.y      = old_x * other.y + self.y * other.x;
            }
        }
    };
    () => {
        impl_mul_assign!{@for  Point<N>}
        impl_mul_assign!{@for &Point<N>}
    };
}

impl_mul_assign!{}


macro_rules! impl_div_scalar {
    (@for $type_left:ty) => {
        impl <N> Div<N> for $type_left
            where   N   :   Div<Output=N> + Copy
        {
             type Output = Point<N>;

             fn div(self, other: N) -> Self::Output {
                Point {
                    x: self.x / other,
                    y: self.y / other,
                }
            }
        }
    };
    () => {
        impl_div_scalar!{@for  Point<N>}
        impl_div_scalar!{@for &Point<N>}
    };
}

impl_div_scalar!{}


impl<N> DivAssign<N> for Point<N>
    where   N   :   DivAssign + Copy
{
     fn div_assign(&mut self, other: N) {
        self.x /= other;
        self.y /= other;
    }
}


macro_rules! impl_div {
    (@for $type_left:ty, $type_right:ty) => {
        impl<N> Div<$type_right> for $type_left
            where   N   :   Add<Output=N>
                        +   Sub<Output=N>
                        +   Mul<Output=N>
                        +   Div<Output=N>
                        +   Copy
        {
            type Output = Point<N>;

            fn div(self, other: $type_right) -> Self::Output {
                let denom = other.x * other.x + other.y * other.y;
                Point { 
                    x: (self.x * other.x + self.y * other.y) / denom,
                    y: (self.y * other.x - self.x * other.y) / denom,
               }
            }
        }
    };
    () => {
        impl_div!{@for  Point<N>,  Point<N>}
        impl_div!{@for &Point<N>,  Point<N>}
        impl_div!{@for  Point<N>, &Point<N>}
        impl_div!{@for &Point<N>, &Point<N>}
    };
}

impl_div!{}


macro_rules! impl_div_assign {
    (@for $type_right:ty) => {
        impl<N> DivAssign<$type_right> for Point<N>
            where   N   :   Mul<Output=N>
                        +   Div<Output=N>
                        +   Add<Output=N>
                        +   Sub<Output=N>
                        +   Copy
        {
            fn div_assign(&mut self, other: $type_right) {
                let denom   = other.x * other.x + other.y * other.y;
                let temp_x  = (self.x * other.x + self.y * other.y) / denom;
                self.y      = (self.y * other.x - self.x * other.y) / denom;
                self.x      = temp_x;
            }
        }
    };
    () => {
        impl_div_assign!{@for  Point<N>}
        impl_div_assign!{@for &Point<N>}
    };
}

impl_div_assign!{}


impl<N: Copy> Point<N> {
     pub fn new(xval: N, yval: N) -> Self {
        Point {
            x: xval,
            y: yval,
        }
    }

    pub fn from_x(xval: N) -> Self
        where   N:  Default
    {
        Point {
            x: xval,
            y: <N as Default>::default(),
        }
    }

    pub fn from_y(yval: N) -> Self
        where   N:  Default
    {
        Point {
            x: <N as Default>::default(),
            y: yval,
        }
    }

    pub fn conjugate(&self) -> Self
        where   N:  Neg<Output=N>
    {
        Point {
            x:   self.x,
            y:  -self.y,
        }
    }

    #[inline(always)]
    pub fn im(&self) -> N { self.y }

    #[inline(always)]
    pub fn re(&self) -> N { self.x }

    pub fn set(&mut self, xval: N, yval: N) {
        self.x = xval;
        self.y = yval;
    }

    pub fn set_both(&mut self, val: N) {
        self.x = val;
        self.y = val;
    }

    pub fn dot(&self, other: &Self) -> N
        where   N:  Mul<Output=N> + Add<Output=N>
    {
        self.x * other.x + self.y * other.y
    }

    pub fn copy_from(&mut self, src: &Point<N>) {
        self.x = src.x;
        self.y = src.y;
    }

    pub fn map<O, F>(&self, mut f: F) -> Point<O>
        where   F   :   FnMut(N) -> O
    {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }

    #[inline]
    pub fn map_type<O: From<N>>(&self) -> Point<O> {
        self.map(<O as From<N>>::from)
    }
}


pub trait RealPoint<N> {
    fn dist(&self) -> N;
    fn arg(&self) -> N;
    fn polar(&self) -> (N, N);
    fn rotate_radians(&self, radians: N) -> Self;
    fn rotate_degrees(&self, radians: N) -> Self;
    fn exp(&self) -> Self;
    fn normalize(&self) -> Self;
    fn is_close(&self, to: &Self, tolerance: N) -> bool;
}


macro_rules! real_point {
    ($t: ident) => {
        impl RealPoint<$t> for Point<$t> {
            fn dist(&self) -> $t {
                (self.x * self.x + self.y * self.y).sqrt()
            }

            #[inline(always)]
            fn arg(&self) -> $t {
                return $t::atan2(self.y, self.x)
            }

            #[inline(always)]
            fn polar(&self) -> ($t, $t) {
                (self.dist(), self.arg())
            }

            fn rotate_radians(&self, radians: $t) -> Self {
                self * Point{x: <$t>::cos(radians), y: <$t>::sin(radians),}
            }

            #[inline(always)]
            fn rotate_degrees(&self, degrees: $t) -> Self {
                self.rotate_radians(degrees * std::$t::consts::PI / 180.0)
            }

            fn exp(&self) -> Self {
                let x_pow = self.x.exp();
                Point {
                    x: x_pow * self.y.cos(),
                    y: x_pow * self.y.sin(),
                }
            }

            fn normalize(&self) -> Self {
                self / self.dist()
            }

            fn is_close(&self, to: &Self, tolerance: $t) -> bool {
                    <$t as Floating>::is_close(self.x, to.x, tolerance)
                &&  <$t as Floating>::is_close(self.y, to.y, tolerance)
            }
        }
    }
}

real_point!{f32}
real_point!{f64}


macro_rules! impl_mul_transform {
    (@ltype r)              => { &Point<N> };
    (@ltype v)              => {  Point<N> };
    (@rtype r)              => { &Transform<N> };
    (@rtype v)              => {  Transform<N> };
    ($l:ident, $r:ident)    => {
        impl<N> Mul<impl_mul_transform!(@rtype $r)> for impl_mul_transform!(@ltype $l)
            where   N   : Add<Output=N> + Mul<Output=N> + Copy
        {
            type Output = Point<N>;

            fn mul(self, rhs: impl_mul_transform!(@rtype $r)) -> Self::Output {
                Point {
                    x: self.x * rhs.0[0][0] + self.y * rhs.0[1][0] + rhs.0[2][0],
                    y: self.x * rhs.0[0][1] + self.y * rhs.0[1][1] + rhs.0[2][1],
                }
            }
        }
    }
}

impl_mul_transform!{v, v}
impl_mul_transform!{v, r}
impl_mul_transform!{r, v}
impl_mul_transform!{r, r}


macro_rules! impl_mul_assign_transform {
    (@rtype r) => { &Transform<N> };
    (@rtype v) => {  Transform<N> };
    ($r:ident) => {
        impl<N> MulAssign<impl_mul_transform!(@rtype $r)> for Point<N>
            where   N   : Add<Output=N> + Mul<Output=N> + Copy
        {
            fn mul_assign(&mut self, rhs: impl_mul_transform!(@rtype $r)) {
                let old_x   = self.x;
                self.x      = old_x * rhs.0[0][0] + self.y * rhs.0[1][0] + rhs.0[2][0];
                self.y      = old_x * rhs.0[0][1] + self.y * rhs.0[1][1] + rhs.0[2][1];
            }
        }
    }
}

impl_mul_assign_transform!{v}
impl_mul_assign_transform!{r}


#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn arithmetic() {
        let mut p1 = Point::<f32>::new(0.5, 1.0) + Point::<f32>::new(1.5, 1.0);
        assert_eq!(p1, Point::<f32>::new(2.0, 2.0));
        assert_eq!(-&p1, Point::new(-2.0, -2.0));
        p1 += Point::new(0.5, 1.0);
        assert_eq!(p1, Point::new(2.5, 3.0));
        p1 = Point::new(4.0, 8.0) - Point::new(3.0, 4.0);
        assert_eq!(p1, Point::new(1.0, 4.0));
        p1 -= Point::new(0.5, 2.5);
        assert_eq!(p1, Point::new(0.5, 1.5));
        p1 = p1 * 2.0;
        assert_eq!(p1, Point::new(1.0, 3.0));
        p1 *= 3.0;
        assert_eq!(p1, Point::new(3.0, 9.0));
        p1 = p1 * Point::new(-2.0, 2.0);
        assert_eq!(p1, Point::new(-24.0, -12.0));
        p1 *= Point::new(-0.25, 0.25);
        assert_eq!(p1, Point::new(9.0, -3.0));
        p1 = p1 / 2.0;
        assert_eq!(p1, Point::new(4.5, -1.5));
        p1 /= -0.5;
        assert_eq!(p1, Point::new(-9.0, 3.0));
        p1 = p1 / Point::new(1.0, -1.0);
        assert_eq!(p1, Point::new(-6.0, -3.0));
        p1 /= Point::new(-1.0, -1.0);
        assert_eq!(p1, Point::new(4.5, -1.5));
    }

    use std::f32::consts::PI;
    #[wasm_bindgen_test]
    fn rot() {
        let p1 = Point::<f32>::new(1.0, 0.0);
        assert_eq!(p1.rotate_radians(PI/2.0), p1.rotate_degrees(90.0));
        assert!((p1.rotate_radians(PI/2.0) - Point::new(0.0, 1.0)).dist() < 0.000001);
        assert!((p1.rotate_radians(PI/4.0) - Point::new(0.7071067811865475, 0.7071067811865476)).dist() < 0.000001);
    }

    #[wasm_bindgen_test]
    fn complex() {
        //  Dot product
        assert_eq!(Point::<f32>::new(0.5, 1.0).dot(&Point::new(2.0, 1.0)), 2.0);
        let p1 = Point::<i16>::new(3, 4);
        //  Complex-only operations
        assert_eq!(p1.conjugate(), Point::new(3, -4));
        assert_eq!(p1.im(), 4);
        assert_eq!(p1.re(), 3);
        assert_eq!(Point::<f32>::new(3.0, 4.0).dist(), 5.0);
        let (module, argument) = Point::<f32>::new(-3.535533905932737, -3.535533905932739).polar();
        assert!((Point::new(module, f32::cos(argument)) - Point::new(5.0, f32::cos(5.0*PI/4.0))).dist() < 0.000001);
        assert!((Point::<f32>::new(0.0, 2.0 * PI).exp() - Point::new(1.0, 0.0)).dist() < 0.000001);
        assert!((Point::<f32>::new(3.0, 4.0).normalize() - Point::new(0.6, 0.8)).dist() < 0.000001);
    }

    #[wasm_bindgen_test]
    fn coercers_setters_and_copiers() {
        let mut p1 = Point::<i16>::new(1, 2);
        let p2: Point<f32> = <Point<f32> as From<&Point<i16>>>::from(&p1);
        assert_eq!(p1.map(|n| <f32 as From<i16>>::from(n) * 0.5), Point::<f32>::new(0.5, 1.0));
        assert_eq!(p1.map_type::<f64>(), Point::<f64>::new(1.0, 2.0));
        assert_eq!(p2, Point::new(1.0, 2.0));
        p1 = Point::<i16>::from_x(3);
        assert_eq!(p1, Point::new(3, 0));
        p1 = Point::from_y(4);
        assert_eq!(p1, Point::new(0, 4));
        p1.set(5, 6);
        assert_eq!(p1, Point::new(5, 6));
        p1.set_both(7);
        assert_eq!(p1, Point::new(7, 7));
        let p2 = Point::<i16>::new(8, 9);
        p1.copy_from(&p2);
        assert_eq!(p1, p2);
        assert_eq!(Point::from(&p1), p1);
    }

    #[wasm_bindgen_test]
    fn transform() {
        let p = Point::<f32>::new(1.0, 2.0);
        let t = Transform::<f32>::new_from_set_transform_args(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(p * t, Point::new(12.0, 16.0));
    }


    #[wasm_bindgen_test]
    fn closeness() {
        //  Both points close or gone.
        assert!(Point::new(1.0f64, 1.0).is_close(&Point::new(1.001, 0.999), 0.0011));
        assert!(!Point::new(1.0f64, 1.0).is_close(&Point::new(1.001, 0.999), 0.0005));
        //  Should fail if only one point fails.
        assert!(!Point::new(1.0f64, 1.0).is_close(&Point::new(1.0002, 0.999), 0.0005));
    }
}
