//! Module      : Bézier Curves
//! Description : Implementations of n-dimensional Bézier curves.
//! Version     : 0.1.0
//! Author      : Alexander Feterman-Naranjo <10951848+CubOfJudahsLion@users.noreply.github.com>
//! Created     : Thu Sep 10 04:02:17 PM CST 2020
//! Modified    : Thu 06/03/2021 12:12 PM


use std::cmp::{PartialOrd, Ordering};
use std::ops::{Add, Mul, Sub, Div};
use std::default::Default;


#[derive(Debug, Clone, Copy)]
//  Defines a point with an ordinate of type S (expected to be a scalar)
//  and one of type P (which can be any arbitrary type that meets the
//  constraints.) 
struct HybridPoint<S, P> {
    x: S,
    y: P,
}

impl<S: Copy, P: Copy> HybridPoint<S, P> {
    pub fn new(x: S, y: P) -> Self {
        HybridPoint{x, y}
    }
}

impl<S, P> Add for HybridPoint<S, P>
    where   P   : Add<Output=P> + Copy,
            S   : Add<Output=S> + Copy
{
    type Output = HybridPoint<S, P>;

    fn add(self, other: Self) -> Self::Output {
        HybridPoint {x: self.x + other.x, y: self.y + other.y,}
    }
}

impl<S, P> Add for &HybridPoint<S, P>
    where   P   : Add<Output=P> + Copy,
            S   : Add<Output=S> + Copy
{
    type Output = HybridPoint<S, P>;

    fn add(self, other: Self) -> Self::Output {
        HybridPoint {x: self.x + other.x, y: self.y + other.y,}
    }
}

impl<S, P> Sub for &HybridPoint<S, P>
    where   P   : Sub<Output=P> + Copy,
            S   : Sub<Output=S> + Copy
{
    type Output = HybridPoint<S, P>;

    fn sub(self, other: Self) -> Self::Output {
        HybridPoint {x: self.x - other.x, y: self.y - other.y,}
    }
}

impl<S, P> Mul<S> for HybridPoint<S, P>
    where   P   : Mul<S, Output=P> + Copy,
            S   : Mul<Output=S> + Copy
{
    type Output = HybridPoint<S, P>;

    fn mul(self, other: S) -> Self::Output {
        HybridPoint {x: self.x * other, y: self.y * other,}
    }
}

impl<S, P> Mul<S> for &HybridPoint<S, P>
    where   P   : Mul<S, Output=P> + Copy,
            S   : Mul<Output=S> + Copy
{
    type Output = HybridPoint<S, P>;

    fn mul(self, other: S) -> Self::Output {
        HybridPoint {x: self.x * other, y: self.y * other,}
    }
}

impl<S, P> Div<S> for &HybridPoint<S, P>
    where   P   : Div<S, Output=P> + Copy,
            S   : Div<Output=S> + Copy
{
    type Output = HybridPoint<S, P>;

    fn div(self, other: S) -> Self::Output {
        HybridPoint {x: self.x / other, y: self.y / other,}
    }
}


/// A Bézier-like object, with a generic interpolatable point type.
pub trait BezierLike<Precision, Point> {
    fn at(&self, t: Precision) -> Point;
}


#[derive(Clone, Copy, Debug)]
/// A structure having the minimal amount of points for a Bézier.
/// The name comes from its polynomial representation, which
/// is of degree two, therefore allowing for C1 (first derivative)
/// continuity.
pub struct QuadraticBezier<P> {
    p0      : P,
    control : P,
    p1      : P,
}


impl<P: Copy> QuadraticBezier<P> {
    #[allow(dead_code)]
    pub fn new(p0: P, control: P, p1: P) -> Self {
        QuadraticBezier {
            p0,
            control,
            p1,
        }
    }
}


impl<S, P> BezierLike<S, P> for QuadraticBezier<P>
    where   P:  Add<Output=P> + Mul<S, Output=P> + Copy,
            S:  From<u8> + Sub<Output=S> + Mul<Output=S> + Copy
{
    fn at(&self, t: S) -> P {
        let one_minus_t = <S as From<u8>>::from(1) - t;
        let a           = self.p0      * one_minus_t  + self.control * t;
        let b           = self.control * one_minus_t  + self.p1      * t;
        a * one_minus_t + b * t
    }
}


#[derive(Clone, Copy, Debug)]
pub enum GeneralizedBezierError {
    NotEnoughPoints,
}

#[derive(Clone, Debug)]
pub struct GeneralizedBezier<P>(Box<[P]>);


impl<P: Copy> GeneralizedBezier<P> {
    #[allow(dead_code)]
    pub fn new(point_set: &[P]) -> Result<Self, GeneralizedBezierError> {
        let len = point_set.len();
        if len < 3 {
            Err(GeneralizedBezierError::NotEnoughPoints)
        }
        else {
            let mut v = Vec::<P>::with_capacity(len);
            v.extend_from_slice(point_set);
            Ok(GeneralizedBezier(v.into_boxed_slice()))
        }
    }
}


impl<S, P> BezierLike<S, P> for GeneralizedBezier<P>
    where   P:  Add<Output=P> + Mul<S, Output=P> + Copy + Default,
            S:  Sub<Output=S> + Mul<Output=S> + From<u8> + Copy
{
    fn at(&self, t: S) -> P {
        let pts         = &self.0;
        let point_cnt   = pts.len();
        match point_cnt {
            0   =>  <P as Default>::default(),
            1   =>  pts[0],
            2   =>  {
                let one_minus_t = <S as From<u8>>::from(1) - t;
                pts[0] * one_minus_t + pts[1] * t
            },
            3   => {
                let one_minus_t = <S as From<u8>>::from(1) - t;
                    (pts[0] * one_minus_t + pts[1] * t) * one_minus_t 
                +   (pts[1] * one_minus_t + pts[2] * t) * t
            },
            _   =>  {
                let mut intermediate    = Vec::with_capacity(point_cnt - 1);
                let one_minus_t         = <S as From<u8>>::from(1) - t;
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


pub enum BezierControlPointMode<P> {
    Linear,
    Cubic,
    Known(Vec<P>),
}

pub struct PiecewiseBezier<S, P> {
    segments: Box<[GeneralizedBezier<P>]>,
    seg_lims: Box<[S]>,
    curr_seg: usize,
}

impl<S, P> PiecewiseBezier<S, P>
    where   P:  Add<Output=P> + Mul<S, Output=P> + Sub<Output=P> + Div<S, Output=P> + Copy,
            S:  Mul<Output=S> + Div<Output=S> + PartialOrd + From<u8> + Copy
{
    pub fn new(p0: (S, P), m01: BezierControlPointMode<P>, p1: (S, P), pts: &[(BezierControlPointMode<P>, S, P)]) -> Self {
        use BezierControlPointMode::*;
        let len         = pts.len();
        let c01         = match pts {
            &[]                     => vec![],
            &[(Known(v), x, y), ..] => {
            },
            [Liena
        };
        let mut in_pts  = vec![
            (Known(vec![]), p0.0, p1.1),
            (m01, p1.0, p1.1)
        ];
        in_pts.extend_from_slice(pts);
        for next_pt in in_pts {
            
        }
        let v           = Vec::<GeneralizedBezier<P>>::with_capacity(1 + pts.len());
        let prev_left   = 
    }
}
