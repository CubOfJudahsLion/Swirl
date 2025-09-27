//! Module      : Bézier Curves
//! Description : Implementations of n-dimensional Bézier curves.
//! Version     : 0.1.0
//! Author      : Alexander Feterman-Naranjo <10951848+CubOfJudahsLion@users.noreply.github.com>
//! Created     : Thu Sep 10 04:02:17 PM CST 2020
//! Modified    : Mon Feb  1 08:44:23 PM CST 2021



extern crate alloc;

use core::ops::{Add, Mul, Sub};
use core::marker::Copy;
use core::convert::From;
use core::default::Default;
use alloc::vec::Vec;


/// A Bézier-like object, with a generic interpolatable point type.
pub trait BezierLike<Precision> {
    type Point;
    fn points<'c>(&'c self) -> Vec<&'c Self::Point>;
    fn at(&self, t: Precision) -> Self::Point;
}


#[derive(Clone, Debug)]
/// A structure having the minimal amount of points for a Bézier.
pub struct CubicBezier<N> {
    p0      : N,
    control : N,
    p1      : N,
}


impl<N: Copy> CubicBezier<N> {
    #[allow(dead_code)]
    pub fn new(start: N, control: N, end: N) -> Self {
        CubicBezier {
            p0      : start,
            control : control,
            p1      : end,
        }
    }
}


impl<N, P> BezierLike<P> for CubicBezier<N>
    where   N:  Add<Output=N> + Mul<P, Output=N> + Copy,
            P:  From<f32> + Sub<Output=P> + Copy
{
    type Point      = N;

    fn points<'c>(&'c self) -> Vec<&'c Self::Point> { vec![&self.p0, &self.control, &self.p1] }

    fn at(&self, t: P) -> Self::Point {
        let one_minus_t = <P as From<f32>>::from(1.0f32) - t;
        let a           = self.p0      * one_minus_t  + self.control * t;
        let b           = self.control * one_minus_t  + self.p1      * t;
        a * one_minus_t + b * t
    }
}


#[derive(Clone, Debug)]
pub struct GeneralizedBezier<N>(Vec<N>);


impl<N: Copy> GeneralizedBezier<N> {
    #[allow(dead_code)]
    pub fn new(point_set: &[N]) -> GeneralizedBezier<N> {
        let mut v = Vec::with_capacity(point_set.len());
        for p in point_set {
            v.push(*p)
        }
        GeneralizedBezier(v)
    }
}


impl<N, P> BezierLike<P> for GeneralizedBezier<N>
    where   N:  Add<Output=N> + Mul<P, Output=N> + Copy + Default,
            P:  Sub<Output=P> + From<f32> + Copy
{
    type Point = N;

    fn points<'a>(&'a self) -> Vec<&'a Self::Point> {
        let &GeneralizedBezier(ref pts) = self;
        let mut v: Vec<&'a N> = Vec::with_capacity(pts.len());
        for i in pts {
            v.push(i);
        }
        v
    }

    fn at(&self, t: P) -> Self::Point {
        let &GeneralizedBezier(ref pts) = self;
        let point_cnt                   = pts.len();
        match point_cnt {
            0   =>  <N as Default>::default(),
            1   =>  pts[0],
            _   =>  {
                let mut intermediate    = Vec::with_capacity(point_cnt - 1);
                let one_minus_t         = <P as From<f32>>::from(1.0f32) - t;
                for i in 0 .. point_cnt - 1 {
                    intermediate.push(pts[i] * one_minus_t + pts[i + 1] * t);
                }
                for i in (0 .. point_cnt - 1).rev() {
                    for j in 0 .. i {
                        intermediate[i] = intermediate[j] * one_minus_t + intermediate[j + 1] * t;
                    }
                }
                intermediate[0]
            }
        }
    }
}

