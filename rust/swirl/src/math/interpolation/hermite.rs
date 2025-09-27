//! Module      :   Hermite
//! Description :   An interpolation method with C2 continuity, dependent
//!                 on [control] neighboring nodes. Polynomial degree = 3.
//! Version     :   0.1.0
//! Author      :   Alexander Feterman-Naranjo <10951848+CubOfJudahsLion@users.noreply.github.com>
//! Created     :   2020-11-03 14:17
//! Modified    :   Tue 08/31/2021 01:54 PM


use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Neg, Add, Mul, Sub, Div};



/** A curve interpolator that mantains C2 continuity, able to build piecewise curves by
    using tangents. This is the simplest possible version of the interpolator, which
    directly requires the two tangents (`m0`, `m1`) of the points (`p0` and `p1`,
    respectively.) The "moment" of the curve (`t`) must be normalized (i.e., in [0, 1].)
    To be usable by piece-wise curves, it also accepts a "width" parameter (`w`) that
    should contain the width of the x-ordinal interval, i.e., x<sub>k+1<sub> - 
    x<sub>k</sub> -- or, corresponding to parameter indexes, x<sub>1</sub> - 
    x<sub>0</sub>.
*/
#[allow(dead_code)]
pub fn hermite_tg_interpolation<P, S>(p0: P, m0: P, p1: P, m1: P, w: S, t: S) -> P
    where   P:  Mul<S, Output=P> + Add<Output=P> + Copy,
            S:  From<u8> + Mul<Output=S> + Sub<Output=S> + Add<Output=S> + Copy
{
    let t_sq            = t * t;
    let one             = <S as From<u8>>::from(1);
    let one_minus_t     = one - t;
    let one_minus_t_sq  = one_minus_t * one_minus_t;
    let two             = <S as From<u8>>::from(2);
    let three           = <S as From<u8>>::from(3);
    let h00             = (one + two * t) * one_minus_t_sq;
    let h10             = t * one_minus_t_sq;
    let h01             = t_sq * (three - two * t);
    let h11             = t_sq * (t - one);
    p0 * h00 + m0 * w * h10 + p1 * h01 + m1 * w * h11
}


/// Tangent calculation mode: *Finite difference*.
/// Requires previous and following point. Like Konachek-Bartels, it produces
/// two different tangents -- unless all three points form a straight line --
/// chances are they will differ.
#[inline]
pub fn linear_tangents<P, S>(p0: P, x0: S, p1: P, x1: S, p2: P, x2: S) -> (P, P)
    where   P:  Div<S, Output=P> + Sub<Output=P> + Copy,
            S:  Sub<Output=S> + Copy
{
    ((p1 - p0) / (x1 - x0), (p2 - p1) / (x2 - x1))
}

/// Tangent calculation mode: *Finite difference*.
/// Requires a previous, current and next point.
#[inline]
pub fn hermite_finite_different_tangent<P, S>(p0: P, x0: S, p1: P, x1: S, p2: P, x2: S) -> P
    where   P:  Div<S, Output=P> + Add<Output=P> + Sub<Output=P> + Copy,
            S:  From<u8> + Sub<Output=S> + Copy
{
    ((p1 - p0) / (x1 - x0) + (p2 - p1) / (x2 - x1)) / <S as From<u8>>::from(2)
}

/// Tangent calculation mode: *Cardinal spline*. Just requires a current and next point,
/// and a 'tension' parameter in [0,1], which says how "tightly" the curve is wounded
/// to the points. Do note that *Catmull-Rom* splines are a special case of Cardinal
/// splines where `tension` = 0.5.
#[inline]
pub fn hermite_cardinal_spline_tangent<P, S>(p0: P, x0: S, p2: P, x2: S, tension: S) -> P
    where   P: Div<S, Output=P> + Mul<S, Output=P> + Sub<Output=P> + Copy,
            S: PartialEq + From<u8> + Sub<Output=S> + Copy
{
    (p2 - p0) / (x2 - x0) * (<S as From<u8>>::from(1) - tension)
}


//  This type of spline requires extra factors for calculation.
struct KonacheckBartelsFactors<S> {
    two                 : S,
    one_minus_tension   : S,
    one_minus_bias      : S,
    one_plus_bias       : S,
    one_minus_continuity: S,
    one_plus_continuity : S,
}

impl<S> KonacheckBartelsFactors<S> 
    where   S: Add<Output=S> + Sub<Output=S> + From<u8> + Copy
{
    fn new(tension: S, bias: S, continuity: S) -> Self {
        let one                     = <S as From<u8>>::from(1);
        let two                     = <S as From<u8>>::from(2);
        let one_minus_tension       = one - tension;
        let one_minus_bias          = one - bias;
        let one_plus_bias           = one + bias;
        let one_minus_continuity    = one - continuity;
        let one_plus_continuity     = one + continuity;
        KonacheckBartelsFactors {
            two,
            one_minus_tension,
            one_minus_bias, one_plus_bias,
            one_minus_continuity, one_plus_continuity
        }
    }
}


/// Tangent calculation mode: *Konachek_Bartels*. Requires a total of three points,
/// (plus their *x* ordinate), and a set of parameters for *tension*, *bias* and
/// *continuity*. All of these parameters range in [-1, 1] and setting them all to zero
/// would yield a *Catmull-Rom* spline. For these four points, the spline produces not
/// one but **two** tangents for `p1` -- an incoming and an outgoing one -- which *will*
/// differ when `continuity` is not zero.
pub fn hermite_konachek_bartels_tangents<P, S>(
    p0: P, m0: S,
    p1: P, m1: S,
    p2: P, m2: S,
    tension: S, bias: S, continuity: S
) -> (P, P)
    where   P:  Mul<S, Output=P> + Div<S, Output=P> + Add<Output=P> + Sub<Output=P> + Copy,
            S:  Add<Output=S> + Sub<Output=S> + Mul<Output=S> + Div<Output=S> + From<u8> + Copy
{
    let KonacheckBartelsFactors {
        two,
        one_minus_tension,
        one_minus_bias, one_plus_bias,
        one_minus_continuity, one_plus_continuity
    } = KonacheckBartelsFactors::new(tension, bias, continuity);
    (       (   (p1 - p0) / (m1 - m0) * one_minus_tension * one_plus_bias  * one_minus_continuity
            +   (p2 - p1) / (m2 - m1) * one_minus_tension * one_minus_bias * one_plus_continuity
            )
        /   two
    ,       (   (p1 - p0) / (m1 - m0) * one_minus_tension * one_plus_bias  * one_plus_continuity
            +   (p2 - p1) / (m2 - m1) * one_minus_tension * one_minus_bias * one_minus_continuity
            )
        /   two
    )
}

/** Represents the different algorithms to calculate the tangents in a four-point
    Hermite interpolation. */
#[derive(Clone, Copy, Debug)]
pub enum HermiteTangentMode<S> where S: Copy {
    /// Sometimes, we just want a line.
    Linear,

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


/// An error emitted by the piecewise Hermite interpolator.
#[derive(Clone, Copy, Debug)]
pub enum HermitePiecewiseError {
    NotEnoughPoints,
    UnusedTangentMode,
}

/// A Hermite interpolator for a piecewise curve with *n* points, with tangent calculation
/// mode as a parameter. It also allows for endpoints to be "duplicated" so all points in
/// the curve are used for interpolation, in case the particular algorithm requires
/// extra points for it. Do note that the each point is a pair `(x_i, p_i)` with `p_i` the
/// interpolating point and `x_1` its respective ordinate in the graph: it should
/// belong to a sub-order.
///
/// The structure is optimized to favor monotonically increasing queries in the `x_i`s,
/// but it allows random indexing.
pub struct HermitePiecewiseInterpolator<S, P> {
    points      : Box<[(S, P)]>,    //  Ordinate+point pairs.
    tangents    : Box<[(P, P)]>,    //  List of tangent pairs.
    mru_idx     : usize,            //  Most-recently-used index.
    m_inv       : S,
    b           : S,                //  These last two reduce the MRU interval to [0,1]
}

impl<S, P> HermitePiecewiseInterpolator<S, P>
    where   P   : Mul<S, Output=P> + Div<S, Output=P> + Add<Output=P> + Sub<Output=P>
                + Default + Debug + Copy,
            S   : PartialOrd
                + Neg<Output=S> + Add<Output=S> + Sub<Output=S> + Mul<Output=S> + Div<Output=S>
                + From<u8> + Debug + Copy
{
    /// Creates a new (immutable) piecewise curve cubic interpolator
    /// using the *Finite Difference* method. Note that the two extremes
    /// in the `points` parameter are strictly control points, and
    /// not part of the curve.
    ///
    /// Every entry in the `points` parameter consists of an indexing
    /// ordinate (which must be at least a *partial order*) and a
    /// curve point. This list will be kept sorted internally, but
    /// it's not necessary for it to be previously ordered.
    ///
    /// In general, the structure is optimized for *monotonically
    /// increasing* queries over the indexing ordinate. Random access
    /// is allowed, but less performant.
    pub fn new(
        default_intp: HermiteTangentMode<S>, 
        points      : &[(S, P, Option<HermiteTangentMode<S>>)]
    ) -> Result<HermitePiecewiseInterpolator<S, P>, HermitePiecewiseError>
    {
        //console::log_1(&JsValue::from_str(&format!("points: {:?}", points)));
        let len = points.len();
        if len < 3 {
            Err(HermitePiecewiseError::NotEnoughPoints)
        }
        else if points[0].2.is_some() {
            Err(HermitePiecewiseError::UnusedTangentMode)
        }
        else {
            //let mut pointsv     = Vec::<(S, P)>::with_capacity(len);
            let mut pointsv     = points
                .iter()
                .map(|(s, p, _)| (*s, *p))
                .collect::<Vec<(S, P)>>();
            //pointsv.extend_from_slice(points);
            pointsv.sort_by(|&(a, _), &(b, _)|
                a.partial_cmp(&b).unwrap_or(Ordering::Equal)
            );
            //console::log_1(&JsValue::from_str(&format!("sorted pointsv: {:?}", &pointsv[1..len - 1])));
            let mut tangents    = Vec::<(P, P)>::with_capacity(len - 1);
            let mut pt_idx      = 1;
            loop {
                let tg_model                    = points[pt_idx].2.unwrap_or(default_intp);
                let (tg_in_out, needed_right)   = match &tg_model {
                    &HermiteTangentMode::Linear             => {
                        if pt_idx + 1 >= len {
                            Err(HermitePiecewiseError::NotEnoughPoints)
                        }
                        else {
                            let tangents = linear_tangents(
                                //p0: P, x0: S, p1: P, x1: S, p2: P, x2: S
                                points[pt_idx - 1].1, points[pt_idx - 1].0,
                                points[pt_idx    ].1, points[pt_idx    ].0,
                                points[pt_idx + 1].1, points[pt_idx + 1].0
                            );
                            Ok((tangents, 1))
                            //let zero = <P as Default>::default();
                            //Ok(((zero, zero), 1))
                        }
                    },
                    &HermiteTangentMode::Cardinal(tension)  => {
                        if pt_idx + 1 >= len {
                            Err(HermitePiecewiseError::NotEnoughPoints)
                        }
                        else {
                            let tangent = hermite_cardinal_spline_tangent(
                                //p0: P, x0: S, p1: P, x1: S, tension: S
                                points[pt_idx - 1].1, points[pt_idx - 1].0,
                                points[pt_idx + 1].1, points[pt_idx + 1].0,
                                tension
                            );
                            Ok(((tangent, tangent), 1))
                        }
                    },
                    &HermiteTangentMode::FiniteDifference   => {
                        if pt_idx + 1 >= len {
                            Err(HermitePiecewiseError::NotEnoughPoints)
                        }
                        else {
                            let tangent = hermite_finite_different_tangent(
                                //p0: P, x0: S, p1: P, x1: S, p2: P, x2: S
                                points[pt_idx - 1].1, points[pt_idx - 1].0,
                                points[pt_idx    ].1, points[pt_idx    ].0,
                                points[pt_idx + 1].1, points[pt_idx + 1].0
                            );
                            Ok(((tangent, tangent), 1))
                        }
                    },
                    &HermiteTangentMode::KonachekBartels {
                        tension, bias, continuity
                    }                                       => {
                        if pt_idx + 2 >= len {
                            Err(HermitePiecewiseError::NotEnoughPoints)
                        }
                        else {
                            let tangents = hermite_konachek_bartels_tangents(
                                //p0: P, p1: P, p2: P, p3: P,
                                //tension: S, bias: S, continuity: S
                                points[pt_idx - 1].1, points[pt_idx - 1].0, 
                                points[pt_idx    ].1, points[pt_idx    ].0, 
                                points[pt_idx + 1].1, points[pt_idx + 1].0,
                                tension, bias, continuity
                            );
                            Ok((tangents, 1))
                        }
                    },
                }?;
                if pt_idx + needed_right >= len {
                    Err(HermitePiecewiseError::NotEnoughPoints)?;
                }
                tangents.push(tg_in_out);
                if pt_idx + needed_right + 1 == len {
                    for i in 1..=needed_right {
                        if points[pt_idx + i].2.is_some() {
                            Err(HermitePiecewiseError::UnusedTangentMode)?;
                        }
                    }
                    pointsv.remove(0);
                    for _ in 0..needed_right {
                        let _ = pointsv.pop();
                    }
                    //console::log_1(&JsValue::from_str(&format!("points: {:?}", pointsv)));
                    //console::log_1(&JsValue::from_str(&format!("tangents: {:?}", tangents)));
                    break;
                }
                pt_idx += 1;
            }
            //  Assume queries will start at the first segment.
            let m_inv           = pointsv[1].0 - pointsv[0].0;
            let b               = -pointsv[0].0 / m_inv;
            //console::log_1(&JsValue::from_str(&format!("m: {:?}, b: {:?}", m, b)));
            Ok(HermitePiecewiseInterpolator {
                points  : Box::<[(S, P)]>::from(&pointsv[..]),
                tangents: Box::<[(P, P)]>::from(&tangents[..]),
                mru_idx : 0,
                m_inv,
                b,
            })
        }
    }

    /// Query the interpolator. In doing so, allow for it to update
    /// its internal query structure as needed.
    pub fn interpolate(&mut self, s: S) -> P {
        let len         = self.points.len();
        let prev_mru    = self.mru_idx;
        while s < self.points[self.mru_idx].0 && self.mru_idx > 0 {
            self.mru_idx -= 1;
        }
        while self.points[self.mru_idx + 1].0 < s && self.mru_idx + 1 < len {
            self.mru_idx += 1;
        }
        if self.mru_idx != prev_mru {
            self.m_inv  = self.points[self.mru_idx + 1].0 - self.points[self.mru_idx].0;
            self.b      = -self.points[self.mru_idx].0 / self.m_inv;
            //console::log_1(&JsValue::from_str(&format!(
            //    "updated mru to {} (index from {:?} to {:?}), m = {:?}, b = {:?}",
            //    self.mru_idx,
            //    self.points[self.mru_idx    ].0,
            //    self.points[self.mru_idx + 1].0,
            //    self.m,
            //    self.b
            //)));
        }
        let result = hermite_tg_interpolation(
            self.points[self.mru_idx    ].1, self.tangents[self.mru_idx    ].1,
            self.points[self.mru_idx + 1].1, self.tangents[self.mru_idx + 1].0,
            self.m_inv,
                (if s < self.points[0].0 {
                    self.points[0].0
                }
                else if s > self.points[len - 1].0 {
                    self.points[len - 1].0
                }
                else {
                    s
                })
            /   self.m_inv
            +   self.b
        );
        //console::log_1(&JsValue::from_str(&format!("#{:?}", result)));
        result
    }


    #[cfg(hermite_graph)]
    pub fn graph(
        this    : &mut HermitePiecewiseInterpolator<f32, f32>,
        ctx     : &CanvasRenderingContext2d
    ) -> Result<(), JsValue>
    {
        use wasm_bindgen::JsValue;
        use web_sys::{/*console,*/ CanvasRenderingContext2d};
        let len         = this.points.len();
        let x_max       = 3625.0f64;
        let x_res       = 1611.0f64 - 200.0;
        let x_m         = x_res / x_max;
        let y_max       = 180.0f64 * 60.0 + 20.0;
        let y_res       = 796.0f64 - 100.0;
        let y_m         = y_res / y_max;
        ctx.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.5"));
        ctx.fill_rect(0.0, 0.0, 1611.0, 796.0);
        ctx.set_line_width(1.0);
        ctx.set_global_alpha(1.0);
        let tg_len      = 30.0f64;
        for i in 0..len {
            let curr_x = this.points[i].0 as f64 * x_m + 100.0;
            let curr_y = this.points[i].1 as f64 * y_m +  50.0;
            //console::log_1(&JsValue::from_str(&format!("({:?},{:?})", curr_x, curr_y)));
            ctx.set_stroke_style(&JsValue::from_str("blue"));
            ctx.begin_path();
            ctx.move_to(curr_x, curr_y);
            ctx.ellipse(curr_x, curr_y, 2.0, 2.0, 0.0, 0.0, 2.0 * std::f64::consts::PI)?;
            ctx.ellipse(curr_x, curr_y, 4.0, 4.0, 0.0, 0.0, 2.0 * std::f64::consts::PI)?;
            ctx.stroke();
            ctx.set_stroke_style(&JsValue::from_str("rgba(255,255,0,0.66)"));
            ctx.set_line_width(6.0);
            ctx.begin_path();
            let m = this.tangents[i].0 as f64;
            let target_x = -tg_len/f64::sqrt(x_m*x_m+y_m*y_m*m*m);
            //console::log_1(&JsValue::from_str(&format!(
            //    "m: {:?}, curr_x: {:?}, curr_y: {:?}, target_x: {:?}, target_y: {:?}",
            //    m, curr_x, curr_y, target_x, target_x * m
            //)));
            ctx.move_to(curr_x, curr_y);
            ctx.line_to(target_x * x_m + curr_x, target_x * m * y_m + curr_y);
            ctx.stroke();
            ctx.set_stroke_style(&JsValue::from_str("rgba(0,255,255,0.66)"));
            ctx.begin_path();
            let m = this.tangents[i].1 as f64;
            let target_x = tg_len/f64::sqrt(x_m*x_m+y_m*y_m*m*m);
            //console::log_1(&JsValue::from_str(&format!(
            //    "m: {:?}, curr_x: {:?}, curr_y: {:?}, target_x: {:?}, target_y: {:?}",
            //    m, curr_x, curr_y, target_x, target_x * m
            //)));
            ctx.move_to(curr_x, curr_y);
            ctx.line_to(target_x * x_m + curr_x, target_x * m * y_m + curr_y);
            ctx.stroke();
            ctx.set_line_width(1.0);
        }
        let (left, right) = (this.points[0], this.points[len - 1]);
        let mut prev_x = left.0 as f64 * x_m + 100.0;
        let mut prev_y = left.1 as f64 * y_m +  50.0;
        //console::log_1(&JsValue::from_str(&format!("({:?},{:?})", prev_x, prev_y))); 
        ctx.set_stroke_style(&JsValue::from_str("rgba(255,0,0,0.5)"));
        ctx.begin_path();
        ctx.ellipse(prev_x, prev_y, 1.5, 1.5, 0.0, 0.0, 2.0 * std::f64::consts::PI)?;
        ctx.stroke();
        for i in 1..100 {
            let t = i as f32 / 99.0;
            let x_query = (1.0 - t) * left.0 as f32 + t * right.0 as f32;
            let curr_y = this.interpolate(x_query) as f64 * y_m + 50.0;
            let curr_x = x_query as f64 * x_m + 100.0;
            //console::log_1(&JsValue::from_str(&format!("({:?},{:?})", curr_x, curr_y)));
            ctx.set_stroke_style(&JsValue::from_str("rgba(255,0,0,0.5)"));
            ctx.begin_path();
            ctx.ellipse(curr_x, curr_y, 1.5, 1.5, 0.0, 0.0, 2.0 * std::f64::consts::PI)?;
            ctx.stroke();
            ctx.set_stroke_style(&JsValue::from_str("black"));
            ctx.begin_path();
            ctx.move_to(prev_x, prev_y);
            ctx.line_to(curr_x, curr_y);
            ctx.stroke();
            prev_y = curr_y;
            prev_x = curr_x;
        }
        Ok(())
    }
}
