//! Module      : Interpolation
//! Description : A module with several types of interpolation methods, some also useful for curve-building
//! Version     : 0.1.0
//! Author      : Alexander Feterman-Naranjo <10951848+CubOfJudahsLion@users.noreply.github.com>
//! Created     : 2020-11-03 14:17
//! Modified    : 2021-02-22 18:32


//
use std::ops::{Add, Mul, Sub, Div};
//use core::default::Default;
use super::point::{Point, RealPoint};


/** Simplest interpolation type: linear. Offers no continuity for piecewise curves.
    The moment of the curve (t) must be in [0, 1]. */
#[allow(dead_code)]
pub fn linear_interpolation<P, S>(left: P, right: P, t: S) -> P
    where   P:  Mul<S, Output=P> + Add<P, Output=P> + Copy,
            S:  From<u8> + Sub<Output=S> + Copy
{
    left * (<S as From<u8>>::from(1) - t) + right * t
}


/** The simplest method to keep piecewise continuity is to have a derivable interpolating
    function that yields the same for a node regardless of whether it is the left or
    right one. The Cosine method is ideal for this. */
#[allow(dead_code)]
pub fn cosine_interpolation<P, S>

/** A curve interpolator that mantains C1 continuity, able to build piecewise curves by
    using tangents. This is the simplest possible version of the interpolator, which
    directly requires the two tangents (m0, m1) of the points (p0 and p1, respectively.)
    The "moment" of the curve (t) must be normalized (i.e., in [0, 1].)
*/
#[allow(dead_code)]
pub fn hermite_tg_interpolation<P, S>(p0: P, m0: P, p1: P, m1: P, t: S) -> P
    where   P:  Mul<S, Output=P> + Add<Output=P> + Copy,
            S:  From<u8> + Mul<Output=S> + Sub<Output=S> + Add<Output=S> + Copy
{
    let t2      = t * t;
    let one     = <S as From<u8>>::from(1);
    let one_t   = one - t;
    let one_t_2 = one_t * one_t;
    let two     = <S as From<u8>>::from(2);
    let three   = <S as From<u8>>::from(3);
    let h00     = (one + two * t) * one_t_2;
    let h10     = t * one_t_2;
    let h01     = t2 * (three - two * t);
    let h11     = t2 * (t - one);
    p0 * h00 + m0 * h10 + p1 * h01 + m1 * h11
}


/** Represents the different algorithms to calculate the tangents in a four-point
    Hermite interpolation. */
#[allow(dead_code)]
pub enum HermiteTangentMode<S> {
    /// The finite difference merely averages the two difference vectors
    /// between this and the previous and next points.
    FiniteDifference,

    /// The cardinal spline method uses the difference vector from the
    /// the next point as a base and a tension parameter in [0, 1]. If
    /// it's 0, all tangents are zero and C1 continuity is lost. If it's 1,
    /// we obtain a Catmull-Rom spline.
    Cardinal(S),

    /// The Konachek-Bartels method uses the most control parameters, all
    /// varying in the [-1, 1] range. Setting them all to zero yields a
    /// Catmull-Rom spline. The meaning of the parameters is as follows:
    ///
    ///     Tension     T = +1 → Tight              T = −1 → Round
    ///     Bias        B = +1 → Post Shoot         B = −1 → Pre shoot
    ///     Continuity  C = +1 → Inverted corners   C = −1 → Box corners
    KonachekBartels {
        tension     : S,
        bias        : S,
        continuity  : S,
    },
}

/** This is a Hermite interpolator that uses four points instead, i.e., it assumes
    (and preserves) curve continuity. It interpolates between p1 and p2, using
    external points p0 and p3 to calculate the tangents. */
#[allow(dead_code)]
pub fn hermite_interpolation<P, S>(p0: P, p1: P, p2: P, p3: P, t: S, mode: HermiteTangentMode<S>) -> P
    where   P:  Mul<S, Output=P> + Div<S, Output=P> + Sub<Output=P> + Add<Output=P> + RealPoint<P> + Copy,
            S:  From<u8> + Mul<Output=S> + Sub<Output=S> + Add<Output=S> + Copy
{
    let d01         = p1 - p0;
    let d12         = p2 - p1;
    let d23         = p3 - p2;
    use HermiteTangentMode::*;
    let (m1, m2)    = match mode {
        FiniteDifference                             => {
            let two = <S as From<u8>>::from(2);
            ((d01 + d12) / two, (d12 + d23) / two)
        },
        Cardinal(tension)                            => {
            let one_minus_tension  = <S as From<u8>>::from(1) - tension;
            (d12 * one_minus_tension, d23 * one_minus_tension)
        },
        KonachekBartels{tension, bias, continuity}  => {
            let one                     = <S as From<u8>>::from(1);
            let two                     = <S as From<u8>>::from(2);
            let one_minus_tension       = one - tension;
            let one_minus_bias          = one - bias;
            let one_plus_bias           = one + bias;
            let one_minus_continuity    = one - continuity;
            let one_plus_continuity     = one + continuity;
            (   (   d01 * (one_minus_tension * one_plus_bias  * one_plus_continuity)  / two
                +   d12 * (one_minus_tension * one_minus_bias * one_minus_continuity) / two
                )
            ,   (   d12 * (one_minus_tension * one_plus_bias  * one_minus_continuity) / two
                +   d23 * (one_minus_tension * one_minus_bias * one_plus_continuity)  / two
                )
            )
        },
    };
    hermite_tg_interpolation(p1, m1, p2, m2, t)
}


/** Generic (n-point) Bézier-interpolation, using De Casteljau's algorithm.
    This version of the algorithm allows a one-shot utilization of the algorithm;
    for repeated queries in a same set of control points, use the bezier module. */
#[allow(dead_code)]
pub fn bezier_nary_interpolation<P, S>(pts: &[P], t: S) -> P
    where   P:  Mul<S, Output=P> + Add<Output=P> + Default + Copy,
            S:  From<f32> + Sub<Output=S> + Copy
{
    // De Casteljau uses successive linear interpolations between every two points until it's down to one.
    match pts {
        &[]     =>  <P as Default>::default(),
        &[pt]   =>  pt,
        pts     =>  {
            let mut intermediate    = Vec::with_capacity(pts.len() - 1);
            let one_minus_t         = <S as From<f32>>::from(1.0) - t;
            for i in 0 .. pts.len() - 1 {
                intermediate.push(pts[i] * one_minus_t + pts[i + 1] * t);
            }
            for i in (0 .. pts.len() - 1).rev() {
                for j in 0 .. i {
                    intermediate[j] = intermediate[j] * one_minus_t + intermediate[j + 1] * t;
                }
            }
            intermediate[0]
        },
    }
}



/** Spherical linear interpolation. The term may sound like a contradiction, but it's an interpolation
    meant for rotations (hence, only valid for bidimensional objects or beyond.) It's a geometric
    formula that's linear angle-wise; in the unit circle or sphere, it would be an angular
    interpolation between two normalized vectors. */
trait Slerp<Precision> {
    fn slerp(self, other: Self, t: Precision) -> Self;
}

macro_rules! float_point2d_slerp {
    ($t:ty) => {
        impl Slerp<$t> for Point<$t> {
            fn slerp(self, other: Self, t: $t) -> Self {
                let dot     = self.normalize().dot(other.normalize());
                let sindot  = dot.acos().sin();
                self * (((1.0 - t) * dot).sin() / sindot) + other * ((t * dot).sin() / sindot)
            }
        }
    }
}

float_point2d_slerp!{f32}
float_point2d_slerp!{f64}
