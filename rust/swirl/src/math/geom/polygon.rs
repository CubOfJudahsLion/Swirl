//! Module  :   Polygon
//! Purpose :   Operations on 3-to-n-sided polygons.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Sep 21 08:01:51 PM CST 2021
//! Modified:   Wed 03/02/2022 11:45 AM


use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Index};
use std::convert::TryFrom;

#[cfg(debug_assertions)]
use wasm_bindgen::{JsValue, /*throw_str*/};

#[cfg(debug_assertions)]
use web_sys::console;

use super::super::numeric::*;
use super::point::*;
use super::rect::*;
use super::transform::Transform;
use super::line::*;


/// Tests whether a collection of edges self-intersects. The edges are
/// described as a set of points, each connecting to the next and
/// the last connecting to the first.
pub fn point_set_self_intersects<N>(pts: &[Point<N>]) -> bool
    where   N       : Floating + From<i16> + Copy,
            Point<N>: RealPoint<N>
{
    let len = pts.len();
    if len < 3 {
        //  Less than 3 vertices cannot even form a polygon. No intersections.
        false
    }
    else if len == 3 {
        //  We'll consider it an intersection if the 3 points are collinear.
        N::is_close(
            LineEquation::new_from_points(&pts[0], &pts[1]).apply_as_function(pts[2].x),
            pts[2].y,
            N::EPSILON * N::from(8)
        )
    }
    else {
        for i in 0 .. len - 2 {
            let (p0, p1) = (&pts[i], &pts[i + 1]);
            if *p0 == *p1 { return true; }
            for j in i + 2 .. (if i == 0 { len - 1 } else { len }) {
                let (q0, q1) = (&pts[j], &pts[(j + 1) % len]);
                if LineSegment::intersection(&LineSegment::new(p0, p1), &LineSegment::new(q0, q1)).is_some() {
                    return true;
                }
            }
        }
        false
    }
}


/// Tests whether a polygon is convex. The polygon is described as
/// a set of points that form each an edge with the next point, and
/// the last with the first.
pub fn point_set_is_convex<N>(pts: &[Point<N>]) -> bool
    where   N       : Floating + From<i16> + Copy + Debug,
            Point<N>: RealPoint<N>
{
    let len = pts.len();
    if len < 3 {
        //  That's not even a polygon so no, it's not convex.
        //console::log_1(&JsValue::from_str("less than 3 points"));
        false
    }
    else if len == 3 {
        //console::log_1(&JsValue::from_str("3 points"));
        //  Triangles are convex unless they're collinear
        !N::is_close(
            LineEquation::new_from_points(&pts[0], &pts[1]).apply_as_function(pts[2].x),
            pts[2].y,
            N::EPSILON * N::from(8)
        )
    }
    else {
        //  4 or more edges: a polygon is convex if
        //  a)  The angles add up to a full turn (i.e., *2π* or
        //      *-2π*.) This rules out edge loops.
        //  b)  Angle differences between edges are all either
        //      positive or negative.
        //console::log_1(&JsValue::from_str(&format!("{} points", len)));
        let angles          = (0 .. len)
            .map(|i| {
                let p0  = &pts[i];
                let p1  = &pts[(i + 1) % len];
                N::atan2(p1.y - p0.y, p1.x - p0.x)
            });
        let zero            = N::from( 0);
        let one             = N::from( 1);
        let minus_one       = N::from(-1);
        let mut prev_angle  = angles.clone().last().unwrap();
        let mut diff_sum    = zero;
        let mut orientation : Option<N> = None;
        for curr_angle in angles {
            let curr_diff        =
                match curr_angle - prev_angle {
                    n if n < -N::PI     => n + N::TAU,
                    n if n >  N::PI     => n - N::TAU,
                    n                   => n,
                };
            let curr_orientation = if curr_diff < zero { minus_one } else { one };
            if let Some(orientation) = orientation {
                //  Successive turns must have the same orientation,
                //  i.e., their product must be 1. This discards
                //  polygons with collinear points as well.
                if orientation * curr_orientation != one {
                    return false;
                }
            }
            else {
                orientation = Some(curr_orientation);
            }
            prev_angle           = curr_angle;
            diff_sum            += curr_diff;
        }
        //  Angle sum should be close to tau (= 2*pi.)
        N::is_close(diff_sum.abs(), N::TAU, N::from(2) / N::from(1000)) // Many forgiving. Wow.
    }
}


/// An error returned when a variable dependent on point values
/// is non-numeric.
#[derive(Debug, Clone, Copy)]
pub enum PointSetError {
    CannotComparePoints,
    TooManyPoints,
}

/// Creates a copy of the points and sorts it to form a clockwise set.
/// The algorithm will return an error if the points have non-comparable
/// values (e.g., `NaN`.)
pub fn sort_point_set_clockwise<N>(pts: &mut [Point<N>]) -> Result<(), PointSetError>
    where   N       : Floating + From<i16> + Copy,
            Point<N>: RealPoint<N>
{
    let zero            = N::from(0);
    let len             = N::from(
        <i16 as TryFrom<usize>>::try_from(pts.len())
            .map_err(|_| PointSetError::TooManyPoints)?
    );
    let center          = pts.iter().fold(
        Point::new(zero, zero),
        |acc, curr| acc + curr / len
    );
    let atan_pt         = |p: &Point<N>| -> N { N::atan2(p.y - center.y, p.x - center.x) };
    let mut sort_erred  = false;
    pts.sort_by(|p, q|
        N::partial_cmp(&atan_pt(p), &atan_pt(q))
            .unwrap_or_else(|| { sort_erred = true; Ordering::Equal })
    );
    if sort_erred {
        Err(PointSetError::CannotComparePoints)
    }
    else {
        Ok(())
    }
}


/// A polygon construction error.
#[derive(Clone, Copy, Debug)]
pub enum ConvexPolygonError {
    NotEnoughPoints,
    PointsNotConvex,
    CannotComparePoints,
    TooManyPoints,
}

impl From<PointSetError> for ConvexPolygonError {
    fn from(pse: PointSetError) -> Self {
        match pse {
            PointSetError::CannotComparePoints  => ConvexPolygonError::CannotComparePoints,
            PointSetError::TooManyPoints        => ConvexPolygonError::TooManyPoints,
        }
    }
}


/// Polygons seen as directed cyclic graphs and represented
/// as point arrays: a collection of points, with every two
/// consecutive points forming a clockwise edge, including
/// one from the last to first point.
#[derive(Clone, Debug)]
pub struct ConvexPolygon<N>(
    //  Storage invariants: the points should be stored in
    //  clockwise order, each forming an edge with the next.
    Box<[Point<N>]>
);


impl<N> Index<usize> for ConvexPolygon<N> {
    type Output = Point<N>;

    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}


impl<N: Copy> ConvexPolygon<N> {
    /// Obtain all of the points in the polygon.
    #[inline(always)]
    pub fn points(&self) -> &[Point<N>] { &*self.0 }

    pub fn edge_indexes(&self) -> Vec<(usize, usize)> {
        let len = self.0.len();
        (0 .. len)
            .map(|i| (i, (i + 1) % len))
            .collect()
    }

    pub fn edges(&self) -> Vec<(&Point<N>, &Point<N>)> {
        let len = self.0.len();
        (0 .. len)
            .map(|i| (&self.0[i], &self.0[(i + 1) % len]))
            .collect()
    }
}


//  Implement reference/value combinations of multiplication
//  of a`ConvexPolygon` by a `Transform`.
macro_rules! mul_trans {
    (@left_type v)                  => {  ConvexPolygon<N> };
    (@left_type r)                  => { &ConvexPolygon<N> };
    (@right_type v)                 => {  Transform<N> };
    (@right_type r)                 => { &Transform<N> };
    (@right_expr v $var:ident)      => { &$var };
    (@right_expr r $var:ident)      => {  $var };
    ($left: ident, $right: ident)   => {
        impl<N> Mul<mul_trans!(@right_type $right)> for mul_trans!(@left_type $left)
            where N: Mul<Output=N> + Add<Output=N> + Sub<Output=N> + Copy
        {
            type Output = ConvexPolygon<N>;

            fn mul(self, t: mul_trans!(@right_type $right)) -> Self::Output {
                ConvexPolygon(
                    self.0
                        .iter()
                        .map(|p| p * mul_trans!(@right_expr $right t))
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                )
            }
        }
    };
}
mul_trans!{v, v}
mul_trans!{v, r}
mul_trans!{r, v}
mul_trans!{r, r}


impl<N> ConvexPolygon<N>
    where   N       : Floating + From<i16> + Debug + Copy,
            Point<N>: RealPoint<N>
{
    /// Create a convex polygon from a rectangle, optionally tranformed.
    pub fn new_from_rect(r: &Rect<N>, opt_trans: Option<&Transform<N>>) -> Self {
        let pts = match opt_trans {
            Some(t)     => r * t,
            _           => r.clockwise_corners(),
        };
        ConvexPolygon(pts.to_vec().into_boxed_slice())
    }


    /// Creates a copy from a sequence of points. The algorithm will
    /// sort the points clockwise and check that they form a convex
    /// polygon. If so, a `ConvexPolygon` is returned; otherwise,
    /// a `ConvexPolygonError` is yielded.
    pub fn new_from_points(pts: &[Point<N>]) -> Result<Self, ConvexPolygonError> {
        let len = pts.len();
        if len < 3 {
            Err(ConvexPolygonError::NotEnoughPoints)
        }
        else {
            let mut pts = pts.iter().map(|p| p.clone()).collect::<Vec<_>>();
            sort_point_set_clockwise(&mut pts).map_err(ConvexPolygonError::from)?;
            if point_set_is_convex(&pts) {
                Ok(ConvexPolygon(pts.into_boxed_slice()))
            }
            else {
                Err(ConvexPolygonError::PointsNotConvex)
            }
        }
    }

    //  Calculates whether a set of points contains a given point.
    //  Assumes clockwise direction and convexity.
    fn point_set_contains(ps: &[Point<N>], q: &Point<N>) -> bool {
        let len = ps.len();
        (0 .. len)
            .all(|i| LineEquation::new_from_points(&ps[i], &ps[(i + 1) % len]).point_inside(q))
    }

    /// Calculates whether the polygon contains a given point.
    #[inline(always)]
    pub fn contains(&self, q: &Point<N>) -> bool {
        Self::point_set_contains(self.points(), q)
    }

    //  This is the algorithm that runs all shape intersection calculations.
    //  The other, more type-specific routines simply pass their unwrapped point sets.
    //  The algorithm has _O(|p|+|q|)_ complexity. Note that *sufficient point-set
    //  cardinality*, *convexity* and *clockwise-ordering* are **assumed**.
    fn intersect_convex_point_sets<'a, 'b>(p: &'a [Point<N>], q: &'b [Point<N>]) -> Option<Vec<Point<N>>> {
        //  A type to track which edge and from which set of points we're using.
        struct EdgeTracker<'t, N> {
            set: &'t [Point<N>],
            idx: usize,
        }
        impl<'t, N> EdgeTracker<'t, N> {
            #[inline(always)]
            //  Create new tracker, set to index zero.
            fn new(set: &'t [Point<N>]) -> EdgeTracker<'t, N> {
                EdgeTracker { set, idx: 0, }
            }
            //  Obtains the two points making up the current edge.
            #[inline]
            fn get_edge(&self) -> (&'t Point<N>, &'t Point<N>) {
                let len = self.set.len();
                //  As we might have to visit some edges repeatedly
                //  BUT also need to know when we've wrapped around,
                //  we need modulo arithmetic.
                (&self.set[self.idx % len], &self.set[(self.idx + 1) % len])
            }
            #[inline] 
            fn advance(&mut self) {
                self.idx += 1;
            }
            //  Informs whether we've gone past the last item.
            #[inline(always)]
            fn turn_completed(&self) -> bool {
                self.idx >= self.set.len()
            }
        }
        //
        //  We won't modify source point sets. Might as well cache their length.
        //  We'll store all intersections.
        let p_len               = p.len();
        let q_len               = q.len();
        let point_origins_equal =
            Box::new(|a: &PointOrigin, b: &PointOrigin| -> bool {
                use PointOrigin::*;
                match (a, b) {
                    (&P(p0),        &P(p1))         => p0 % p_len == p1 % p_len,
                    (&Q(q0),        &Q(q1))         => q0 % q_len == q1 % q_len,
                    (&Intersection, &Intersection)  => true,
                    _                               => false,
                }
            }) as Box<dyn Fn(&PointOrigin, &PointOrigin) -> bool>;
        //
        //  Indicates where a given point comes from.
        #[derive(Clone, Copy, Debug)]
        enum PointOrigin { P(usize), Q(usize), Intersection, }
        //
        //  How we record an intersection. It's useful to know
        //  both the preceding and the succeeding points to put
        //  the resulting polygon together later. If the previous
        //  or following point is `None`, it means to use the
        //  previous/next intersection point.
        #[derive(Clone, Debug)]
        struct Intersection<N> {
            from    : PointOrigin,  //  Inner point, immediately preceding the intersection.
            outer   : PointOrigin,  //  Outer point, starting the segment that bisects the one
                                    //  that begins with the inner point.
            pt      : Point<N>,     //  Intersection coordinates.
            next    : PointOrigin,  //  Following point.
        }
        //
        //  BEGIN algorithm.
        let mut intersections   : Vec<Intersection<N>>
                                = Vec::with_capacity(usize::min(p_len, q_len) << 1);
                                    //  ^Every edge will intersect at most two others in
                                    //  a convex polygon. And of course intersection
                                    //  count is limited to the one having the least edges.
        let mut p_tr            = EdgeTracker::new(p);
        let mut q_tr            = EdgeTracker::new(q);
        //  We'll loop until both edge trackers have
        //  circled back to the first intersection or
        //  (if no intersection are found) to their
        //  initial points.
        loop {
            #[cfg(debug_assertions)] {
                console::log_1(&JsValue::from_str("Looping"));
            }
            //  If we've wrapped around both edge sets and
            //  found no intersections, end the search.
            if      (   p_tr.turn_completed()
                    ||  q_tr.turn_completed()
                    )
                &&  intersections.len() == 0
            {
                #[cfg(debug_assertions)] {
                    console::log_1(&JsValue::from_str("No intersections, ending from completed turns."));
                    match (p_tr.turn_completed(), q_tr.turn_completed()) {
                        (true,  true)   => console::log_1(&JsValue::from_str("P and Q's turns ended.")),
                        (true,  false)  => console::log_1(&JsValue::from_str("P's turn ended.")),
                        (false, true)   => console::log_1(&JsValue::from_str("Q's turn ended.")),
                        (false, false)  => console::log_1(&JsValue::from_str("...wha? This shouldn't have happened.")),
                    }
                }
                break;
            }
            //  Obtain current edges.
            let (p0, p1)    = p_tr.get_edge();
            let p_seg       = LineSegment::new(p0, p1);
            let (q0, q1)    = q_tr.get_edge();
            let q_seg       = LineSegment::new(q0, q1);
            //  Knowing which points are outside each other's
            //  segment will help us several times in the steps below.
            let q1_inside_p = p_seg.point_inside(&q1);
            //  Do we have an intersection?
            if let Some(pt) = LineSegment::intersection(&p_seg, &q_seg) {
                #[cfg(debug_assertions)] {
                    console::log_1(&JsValue::from_str(&format!("Intersection found (@{:?}), p: {}, q: {}", &pt, p_tr.idx, q_tr.idx)));
                }
                //  First we have to look into the first intersection point.
                let int_len = intersections.len();
                if int_len > 0 {
                    //  Is it the same as the one we just found?
                    //  (The process to calculate the point would've
                    //  been the same both times. so we can actually use
                    //  strict equality comparisons here.)
                    if intersections[0].pt == pt {
                        //  Then it's time to wrap things up.
                        //
                        //  One last look at the last intersection
                        //  to see if it's collinear with the first.
                        //  If that's the case, they're the immediate
                        //  predecessor/successor of each other and
                        //  we inform them of that.
                        if      /*point_origins_equal(&intersections[0].outer, &intersections[int_len - 1].from)
                            ||*/  point_origins_equal(&intersections[0].from,  &intersections[int_len - 1].outer)
                        {
                            intersections[int_len - 1].next = PointOrigin::Intersection;
                            intersections[0].from           = PointOrigin::Intersection;
                            #[cfg(debug_assertions)] {
                                console::log_1(&JsValue::from_str("Equality to first intersection. Loop ends."));
                            }
                        }
                        break;
                    }
                    else {
                        #[cfg(debug_assertions)] {
                            console::log_1(&JsValue::from_str("Not equal to first intersection"));
                        }
                    }
                }
                //  Still here? Then it's a new intersection and
                //  we record it with its immediate preceding
                //  and succeeding points.
                //
                //  1st, create a "naïve" intersection. It's
                //  naïve because it's not aware (yet) that it
                //  can be immediately succeeded and/or preceded
                //  by other intersection points.
                let p_origin        = PointOrigin::P(p_tr.idx);
                                        //  ^No, no module aritmetic here.
                                        //  We'll need real (non-modulo)
                                        //  ordering when reconstructing
                                        //  the target polygon.
                let q_origin        = PointOrigin::Q(q_tr.idx);
                let (from, outer)   =
                    if q1_inside_p {
                        (p_origin, q_origin)
                    }
                    else {
                        (q_origin, p_origin)
                    };
                let mut new_int     = Intersection {
                    from,
                    outer,
                    next:   if q1_inside_p {
                                PointOrigin::Q(q_tr.idx + 1)
                            }
                            else {
                                PointOrigin::P(p_tr.idx + 1)
                            },
                    pt,
                };
                if int_len > 0 {
                    //  Now we look at the last recorded intersection.
                    //  See if they are successive.
                    if      point_origins_equal(&intersections[int_len - 1].outer, &new_int.from)
                        // ||  point_origins_equal(&intersections[int_len - 1].from,  &new_int.outer)
                    {
                        intersections[int_len - 1].next = PointOrigin::Intersection;
                        new_int.from                    = PointOrigin::Intersection;
                    }
                }
                //  Finally, we add the new non-naïve intersection.
                intersections.push(new_int);
            }
            //  Now we advance one of the two edges.  The implemented
            //  strategy gives us O(|p|+|q|) times.) It starts with which
            //  of the eges points at the other.
            let p_points_at_q   = q_seg.points_towards_self(p_seg.endpoints());
            let q_points_at_p   = p_seg.points_towards_self(q_seg.endpoints());
            if p_points_at_q == q_points_at_p {
                //  When both point at each other OR none of them points
                //  at the other, we advance whichever has its ending
                //  vertex outside outside the other's line.
                #[cfg(debug_assertions)] {
                    console::log_1(&JsValue::from_str(&format!("{} point at each other", if q_points_at_p { "Both" } else { "None" })));
                }
                if q1_inside_p {
                    p_tr.advance();
                    #[cfg(debug_assertions)] {
                        console::log_1(&JsValue::from_str(&format!("Advanced P to {} because it's outside", p_tr.idx)));
                    }
                }
                else {
                    q_tr.advance();
                    #[cfg(debug_assertions)] {
                        console::log_1(&JsValue::from_str(&format!("Advanced Q to {} because it's outside", q_tr.idx)));
                    }
                }
            }
            //  In all other cases, we advance the
            //  one that points at the other.
            else if q_points_at_p {
                q_tr.advance();
                #[cfg(debug_assertions)] {
                    console::log_1(&JsValue::from_str(&format!("Advanced Q to {} because it points at P", q_tr.idx)));
                }
            }
            else {
                p_tr.advance();
                #[cfg(debug_assertions)] {
                    console::log_1(&JsValue::from_str(&format!("Advanced P to {} because it points at Q", p_tr.idx)));
                }
            }
        }
        #[cfg(debug_assertions)] {
            intersections.iter().for_each(|i| console::log_1(&JsValue::from(&format!("{:?}", i))));
        }
        //  Now we'll build a result.
        if intersections.len() == 0 {
            //  When there are no intersection,
            //  it's either one of three things:
            //  - `p` is entirely contained by `q`
            //  - `q` is entirely contained by `p`
            //  - There is no interesection
            if Self::point_set_contains(p, &q[0]) {
                //  q is contained, return q.
                Some(q.iter().map(|p| p.clone()).collect())
            }
            else if Self::point_set_contains(q, &p[0]) {
                //  p is contained in q, return p.
                Some(p.iter().map(|p| p.clone()).collect())
            }
            else {
                None
            }
        }
        else {
            //  If we do have a set of intersections,
            //  we need to reconstruct the resulting
            //  polygon from it.
            Some({
                let mut out_verts   = Vec::<Point<N>>::with_capacity(p_len + q_len);
                let     int_len     = intersections.len();
                let mut int_idx     = 0;
                loop {
                    out_verts.push(intersections[int_idx].pt.clone());
                    match intersections[int_idx].next {
                        PointOrigin::P(p_start)     => {
                            let next_int_idx = (int_idx + 1) % int_len;
                            if let &PointOrigin::P(p_end) = &intersections[next_int_idx].from {
                                let p_end = if next_int_idx < int_idx { p_end + p_len } else { p_end };
                                for p_idx in p_start ..= p_end {
                                    out_verts.push(p[p_idx % p_len]);
                                }
                            }
                            else {
                                panic!("Intersection reconstruction: departing point P, target not P.");
                            }
                        },
                        PointOrigin::Q(q_start)     => {
                            let next_int_idx = (int_idx + 1) % int_len;
                            if let &PointOrigin::Q(q_end) = &intersections[next_int_idx].from {
                                let q_end = if next_int_idx < int_idx { q_end + q_len } else { q_end };
                                for q_idx in q_start ..= q_end {
                                    out_verts.push(q[q_idx % q_len]);
                                }
                            }
                            else {
                                panic!("Intersection reconstruction: departing point Q, target not Q.");
                            }
                        },
                        _                           => (),
                    }
                    int_idx += 1;
                    if int_idx == int_len {
                        break;
                    }
                }
                out_verts
            })
        }
    }

    /// Polygon intersection with a rectangle.
    #[inline(always)]
    pub fn intersect_rectangle(&self, r: &Rect<N>) -> Option<Vec<Point<N>>>
        where N: RectBounds
    {
        Self::intersect_convex_point_sets(
            &*self.0,
            &r.clockwise_corners()
        )
    }

    /// Polygon-polygon intersection.
    #[inline(always)]
    pub fn intersect(&self, other: &Self) -> Option<Vec<Point<N>>> {
        Self::intersect_convex_point_sets(&*self.0, &*other.0)
    }
}

//  Implementation of conversion from value and reference rectangles.
macro_rules! from_rect {
    (@from_type v)          => {  Rect<N> };
    (@from_type r)          => { &Rect<N> };
    (@new_arg v $var:ident) => { & $var };
    (@new_arg r $var:ident) => {   $var };
    ($v_or_r:ident)         => {
        impl<N> From<from_rect!(@from_type $v_or_r)> for ConvexPolygon<N>
        where   N       : Floating + From<i16> + RectBounds + Debug + Copy,
                Point<N>: RealPoint<N>,
        {
            #[inline(always)]
            fn from(r: from_rect!(@from_type $v_or_r)) -> Self {
                ConvexPolygon::<N>::new_from_rect(from_rect!(@new_arg $v_or_r r), None)
            }
        }
    };
}
from_rect!{v}
from_rect!{r}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    #[wasm_bindgen_test]
    fn point_set_self_intersection() {
        //  No points, no intersect.
        assert!(!point_set_self_intersects(&[] as &[Point<f32>]));
        //  No full poly, no intersect.
        assert!(!point_set_self_intersects(&[Point::<f32>::new(1., 1.), Point::new(2., 2.)]));
        //  Triangles usually don't self-intersect...
        assert!(!point_set_self_intersects(&[
            Point::<f32>::new(1., 1.),
            Point::new(2., 2.),
            Point::new(2., 1.)
        ]));
        //  ... unless the points are collinear.
        assert!(point_set_self_intersects(&[
            Point::<f32>::new(1., 1.),
            Point::new(2., 2.),
            Point::new(3., 3.)
        ]));
        //  Bow tie shape. Should intersect.
        assert!(point_set_self_intersects(&[
            Point::<f32>::new(-1., -1.),
            Point::new( 1.,  1.),
            Point::new( 1., -1.),
            Point::new(-1.,  1.)
        ]));
        //  Diamond. No intersects.
        assert!(!point_set_self_intersects(&[
            Point::<f32>::new(-1., 0.),
            Point::new(0.,  1.),
            Point::new(1.,  0.),
            Point::new(0., -1.),
        ]));
    }

    #[wasm_bindgen_test]
    fn point_set_convexity() {
        //  No points, no polygon, so it yields false.
        assert!(!point_set_is_convex(&[] as &[Point<f32>]));
        //  Not enough points, same result.
        assert!(!point_set_is_convex(&[Point::<f32>::new(1., 1.), Point::new(2., 2.)]));
        //  Triangles convex unless collinear.
        assert!(point_set_is_convex(&[
            Point::<f32>::new(1., 1.),
            Point::new(2., 2.),
            Point::new(2., 1.)
        ]));
        assert!(!point_set_is_convex(&[
            Point::<f32>::new(1., 1.),
            Point::new(2., 2.),
            Point::new(3., 3.)
        ]));
        //  Bow tie -- or other self-intersecting shapes --
        //  should not pass the convexity test. 
        assert!(!point_set_is_convex(&[
            Point::<f32>::new(-1., -1.),
            Point::new( 1.,  1.),
            Point::new( 1., -1.),
            Point::new(-1.,  1.)
        ]));
        //  Diamond. No intersects. Should pass.
        assert!(point_set_is_convex(&[
            Point::<f32>::new(-1., 0.),
            Point::new(0.,  1.),
            Point::new(1.,  0.),
            Point::new(0., -1.),
        ]));
        //  Diamond in reverse. Should pass.
        //  Turn sense shouldn't affect convexity
        //  analysis.
        assert!(point_set_is_convex(&[
            Point::<f32>::new(0., -1.),
            Point::new( 1., 0.),
            Point::new( 0., 1.),
            Point::new(-1., 0.),
        ]));
    }

    #[wasm_bindgen_test]
    fn point_set_clockwise_sorting() {
        let mut pts = [
            Point::<f32>::new(1., 0.),
            Point::new(0., -1.),
            Point::new(0., 1.),
            Point::new(-1., 0.),
        ];
        assert!(sort_point_set_clockwise(&mut pts[..]).is_ok());
        let mut sorted_clone = pts.to_vec();
        assert!(sort_point_set_clockwise(&mut sorted_clone[..]).is_ok());
        assert!(pts.iter().eq(sorted_clone.iter()));
        let mut angle_diffs = pts
            .iter()
            .chain(pts.iter().take(1))
            .map(|p| f32::atan2(p.y, p.x))
            .scan(None as Option<f32>, |st, curr| {
                match *st {
                    Some(prev) => {
                        let diff =
                            match curr - prev {
                                n if n < -f32::PI   => {
                                    n + f32::TAU
                                },
                                n if n >  f32::PI   => {
                                    n - f32::TAU
                                },
                                n                   => {
                                    n
                                }
                            };
                        *st = None;
                        Some(diff)
                    },
                    None    => {
                        *st = Some(curr);
                        None
                    },
                }
            });
        assert!(angle_diffs.all(|diff| diff > 0.));
    }

    #[wasm_bindgen_test]
    async fn known_convex_poly_intersection() {
        use std::cell::Cell;
        use std::rc::Rc;
        use wasm_bindgen::{closure::Closure, JsCast, JsValue};
        use wasm_bindgen_futures::JsFuture;
        use js_sys::{Function, Promise};
        use web_sys::{CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement, HtmlLabelElement};
        use crate::utils::html::*;
        //
        let doc     = get_document().expect("Could not get document object");
        let body    = get_body().expect("Cannot get body.");
        let canvas  = doc.create_element("canvas")
            .and_then(|cnv| cnv.dyn_into::<HtmlCanvasElement>().map_err(|e| e.unchecked_into()))
            .expect("No document");
        canvas.set_width(600);
        canvas.set_height(400);
        assert!(body.append_child(&canvas).is_ok());
        let context = canvas.get_context("2d")
            .and_then(|opt_cnv| opt_cnv.ok_or_else(|| JsValue::NULL))
            .and_then(|cnv| cnv.dyn_into::<CanvasRenderingContext2d>().map_err(|e| e.unchecked_into()))
            .expect("No context");
        /*let p = [
            Point::new(157.13f64, 120.92),
            Point::new(409.67, 121.36),
            Point::new(397.03, 208.5),
            Point::new(339.66, 290.43),
            Point::new(238.67, 317.25),
            Point::new(180.73, 246.89),
        ];
        let q = [
            Point::new(157.23f64, 266.23),
            Point::new(203.9, 152.95),
            Point::new(282.87, 48.62),
            Point::new(409.13, 266.50),
        ];*/
        let p =
            [   Point { x: 266.44082424513044, y: 284.2894664582833 }
	          ,   Point { x: 260.87704547935687, y: 260.30110801606077 }
	          ,   Point { x: 455.08387110325634, y: 95.6809427779022 }
	          ,   Point { x: 449.13449224619364, y: 383.1292415740533 }
	          ,   Point { x: 437.707965428697, y: 385.8016907055611 }
	          ,   Point { x: 273.8915268143765, y: 303.079238681731 }
            ];
	      let q =
            [   Point { x: 395.47144072774705, y: 101.82959956040109 }
	          ,   Point { x: 361.5306445030952, y: 276.6368423519541 }
	          ,   Point { x: 334.0360118500615, y: 296.6782585337278 }
	          ,   Point { x: 287.6412686869669, y: 313.6549757686166 }
	          ,   Point { x: 264.75261791048604, y: 316.14892471181724 }
	          ,   Point { x: 240.2995810874601, y: 314.9254084453099 }
	          ,   Point { x: 153.84639777269544, y: 272.2056743385012 }
            ];
        let r = ConvexPolygon::intersect_convex_point_sets(&p, &q)
            .expect("Should have produced an intersection");
        context.set_fill_style(&JsValue::from_str("#F8F6F0"));
        context.fill_rect(0., 0., 600., 400.);
        context.set_line_width(1.);
        context.set_stroke_style(&JsValue::from_str("#000080"));
        context.begin_path();
        let last_p_idx = p.len() - 1;
        let last_q_idx = q.len() - 1;
        context.move_to(p[last_p_idx].x, p[last_p_idx].y);
        p.iter().for_each(|p| context.line_to(p.x, p.y));
        context.stroke();
        context.set_fill_style(&JsValue::from_str("#000080"));
        p.iter().enumerate().for_each(|(i, p)| context.fill_text(&format!("P{}", i), p.x, p.y - 5.).expect("Could not label p points"));
        context.set_stroke_style(&JsValue::from_str("#604000"));
        context.begin_path();
        context.move_to(q[last_q_idx].x, q[last_q_idx].y);
        q.iter().for_each(|q| context.line_to(q.x, q.y));
        context.stroke();
        context.set_fill_style(&JsValue::from_str("#604000"));
        q.iter().enumerate().for_each(|(i, q)| context.fill_text(&format!("Q{}", i), q.x, q.y - 5.).expect("Could not label q points"));
        context.set_fill_style(&JsValue::from_str("rgba(255, 255, 0, 0.67)"));
        context.set_line_width(1.5);
        context.set_stroke_style(&JsValue::from_str("#FF0000"));
        context.begin_path();
        let last_r_idx = r.len() - 1;
        context.move_to(r[last_r_idx].x, r[last_r_idx].y);
        r.iter().for_each(|r| context.line_to(r.x, r.y));
        context.fill();
        context.begin_path();
        context.move_to(r[last_r_idx].x, r[last_r_idx].y);
        r.iter().for_each(|r| context.line_to(r.x, r.y));
        context.stroke();
        //  It *should* be an 8-vertex polygon.
        console_log!("result point set: ");
        r.iter().for_each(|r| console_log!("(x:{},y:{})", r.x, r.y));
        //assert_eq!(r.len(), 8, "Result set should have 8 points");
        let question_label = doc
            .create_element("label")
            .and_then(|button| button.dyn_into::<HtmlLabelElement>().map_err(|obj| obj.unchecked_into()))
            .expect("Couldn't create label");
        question_label.set_inner_html("Is the intersection result correct?");
        let yes_button = doc
            .create_element("button")
            .and_then(|button| button.dyn_into::<HtmlButtonElement>().map_err(|obj| obj.unchecked_into()))
            .expect("Couldn create button");
        yes_button.set_inner_html("Yes");
        let no_button = doc
            .create_element("button")
            .and_then(|button| button.dyn_into::<HtmlButtonElement>().map_err(|obj| obj.unchecked_into()))
            .expect("Couldn't create button");
        no_button.set_inner_html("No");
        let click_mem       : Rc<Cell<Option<(Closure<dyn FnMut()>, Closure<dyn FnMut()>)>>>
                            = Rc::new(Cell::new(None));
        let prom_mem        : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                            = Rc::new(Cell::new(None));
        let mut prom_fn     = {
            let body            = body.clone();
            let canvas          = canvas.clone();
            let click_mem       = click_mem.clone();
            let question_label  = question_label.clone();
            let yes_button      = yes_button.clone();
            let no_button       = no_button.clone();
            let prom_mem        = prom_mem.clone();
            move |resolve: Function, reject: Function| {
                let click_yes   = {
                    let body            = body.clone();
                    let canvas          = canvas.clone();
                    let click_mem       = click_mem.clone();
                    let question_label  = question_label.clone();
                    let yes_button      = yes_button.clone();
                    let no_button       = no_button.clone();
                    let prom_mem        = prom_mem.clone();
                    let resolve         = resolve.clone();
                    Closure::wrap({
                        Box::new(move || {
                            let _ = body.remove_child(&question_label);
                            let _ = body.remove_child(&yes_button);
                            let _ = body.remove_child(&no_button);
                            let _ = body.remove_child(&canvas);
                            yes_button.set_onclick(None);
                            no_button.set_onclick(None);
                            let _ = resolve.call0(&JsValue::NULL);
                            prom_mem.set(None);
                            click_mem.set(None);
                        }) as Box<dyn FnMut()>
                    })
                };
                let click_no    =  {
                    let body            = body.clone();
                    let canvas          = canvas.clone();
                    let click_mem       = click_mem.clone();
                    let question_label  = question_label.clone();
                    let yes_button      = yes_button.clone();
                    let no_button       = no_button.clone();
                    let prom_mem        = prom_mem.clone();
                    Closure::wrap({
                        Box::new(move || {
                            let _ = body.remove_child(&question_label);
                            let _ = body.remove_child(&yes_button);
                            let _ = body.remove_child(&no_button);
                            let _ = body.remove_child(&canvas);
                            yes_button.set_onclick(None);
                            no_button.set_onclick(None);
                            let _  = reject.call0(&JsValue::NULL);
                            prom_mem.set(None);
                            click_mem.set(None);
                        }) as Box<dyn FnMut()>
                    })
                };
                yes_button.set_onclick(Some(click_yes.as_ref().unchecked_ref()));
                no_button.set_onclick(Some(click_no.as_ref().unchecked_ref()));
                click_mem.set(Some((click_yes, click_no)));
            }
        };
        let prom = Promise::new(&mut prom_fn);
        prom_mem.set(Some(Box::new(prom_fn)));
        assert!(body.append_child(&doc.create_element("br").expect("Can't create <br/> node")).is_ok());
        assert!(body.append_child(&question_label).is_ok());
        assert!(body.append_child(&doc.create_element("br").expect("Can't create <br/> node")).is_ok());
        assert!(body.append_child(&yes_button).is_ok());
        assert!(body.append_child(&doc.create_element("br").expect("Can't create <br/> node")).is_ok());
        assert!(body.append_child(&no_button).is_ok());
        no_button.scroll_into_view();
        assert!(<JsFuture as From<Promise>>::from(prom).await.is_ok());
    }

    #[wasm_bindgen_test]
    async fn random_convex_poly_intersection() {
        use std::cell::Cell;
        use std::rc::Rc;
        use rand::{self, Rng};
        use wasm_bindgen::{closure::Closure, JsCast, JsValue};
        use wasm_bindgen_futures::JsFuture;
        use js_sys::{Function, Promise};
        use web_sys::{CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement, HtmlInputElement, HtmlLabelElement};
        use crate::utils::html::*;
        //
        let doc         = get_document().expect("Could not get document object");
        let body        = get_body().expect("Cannot get body.");
        //
        let count_mem   = Rc::new(Cell::new(0u32));
        assert!({
            let count       = count_mem.clone();
            let prompt_lbl  = doc.create_element("label")
                .and_then(|sel| sel.dyn_into::<HtmlLabelElement>().map_err(|e| e.unchecked_into()))
                .expect("Couldn't create text for random experiment count selector");
            prompt_lbl.set_inner_html("Pick an amount of random polygon intersection experiments.");
            let sel_input   = doc.create_element("input")
                .and_then(|sel| sel.dyn_into::<HtmlInputElement>().map_err(|e| e.unchecked_into()))
                .expect("Couldn't create random experiment count selector");
            sel_input.set_type("number");
            sel_input.set_min("0");
            sel_input.set_max("100");
            sel_input.set_step("1");
            sel_input.set_value("7");
            let ok_btn      = doc.create_element("button")
                .and_then(|btn| btn.dyn_into::<HtmlButtonElement>().map_err(|e| e.unchecked_into()))
                .expect("Couldn't create button to proceed after experiment count selection");

            ok_btn.set_inner_html("Ok");
            let prom_mem    : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                            = Rc::new(Cell::new(None));
            let mut prom_fn = {
                let body        = body.clone();
                let prompt_lbl  = prompt_lbl.clone();
                let sel_input   = sel_input.clone();
                let ok_btn      = ok_btn.clone();
                let count       = count.clone();
                let prom_mem    = prom_mem.clone();
                move |resolve: Function, _reject: Function| {
                    let click_mem   : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                                    = Rc::new(Cell::new(None));
                    let click_cb    = {
                        let body        = body.clone();
                        let prompt_lbl  = prompt_lbl.clone();
                        let sel_input   = sel_input.clone();
                        let ok_btn      = ok_btn.clone();
                        let count       = count.clone();
                        let prom_mem    = prom_mem.clone();
                        let click_mem   = click_mem.clone();
                        Closure::wrap(Box::new(move || {
                            count.set(sel_input.value_as_number() as u32);
                            let _ = body.remove_child(&prompt_lbl);
                            let _ = body.remove_child(&sel_input);
                            let _ = body.remove_child(&ok_btn);
                            let _ = resolve.call0(&JsValue::NULL);
                            prom_mem.set(None);
                            click_mem.set(None);
                        }) as Box<dyn FnMut()>)
                    };
                    ok_btn.set_onclick(Some(click_cb.as_ref().unchecked_ref()));
                    click_mem.set(Some(click_cb));
                }
            };
            let prom = Promise::new(&mut prom_fn);
            prom_mem.set(Some(Box::new(prom_fn)));
            assert!(body.append_child(&prompt_lbl).is_ok());
            assert!(body.append_child(&sel_input).is_ok());
            assert!(body.append_child(&ok_btn).is_ok());
            <JsFuture as From<Promise>>::from(prom)
        }.await.is_ok());
        let count = count_mem.get();
        console_log!("Generating {} random polygon intersection experiments.", count);
        //
        let canvas  = doc.create_element("canvas")
            .and_then(|cnv| cnv.dyn_into::<HtmlCanvasElement>().map_err(|e| e.unchecked_into()))
            .expect("No document");
        canvas.set_width(600);
        canvas.set_height(400);
        assert!(body.append_child(&canvas).is_ok());
        console_log!("Created and inserted canvas");
        let context = canvas.get_context("2d")
            .and_then(|opt_cnv| opt_cnv.ok_or_else(|| JsValue::NULL))
            .and_then(|cnv| cnv.dyn_into::<CanvasRenderingContext2d>().map_err(|e| e.unchecked_into()))
            .expect("No context");
        console_log!("Obtained context");
        for _ in 0..count {
            const RADIUS: f64   = 150.;
            let p_sides         = rand::thread_rng().gen_range(3 ..= 8usize);
            console_log!("Sides for p: {}", p_sides);
            let p_delta         = Point::new(rand::random::<f64>() * 300.0, rand::random::<f64>() * 100.0);
            let mut p           = (0 .. p_sides)
                .map(|_| {
                    let angle   = rand::random::<f64>() * f64::TAU;
                    let x       = angle.cos() * RADIUS;
                    let y       = angle.sin() * RADIUS;
                    Point::new(x + RADIUS, y + RADIUS) + p_delta
                })
                .collect::<Vec<Point<f64>>>();
            console_log!("Created p ({} vertices)", p.len());
            assert!(sort_point_set_clockwise(&mut p).is_ok());
            console_log!(
                "{}",
                p.iter()
                    .fold(
                        format!("Sorted p ({} vertices):", p.len()),
                        |acc, p| acc + &format!("\n\t{:?}", p)
                    )
            );
            let q_sides         = rand::thread_rng().gen_range(3 ..= 8usize);
            console_log!("Sides for q: {}", q_sides);
            let q_delta         = Point::new(rand::random::<f64>() * 300.0, rand::random::<f64>() * 100.0);
            let mut q           = (0 .. q_sides)
                .map(|_| {
                    let angle   = rand::random::<f64>() * f64::TAU;
                    let x       = angle.cos() * RADIUS;
                    let y       = angle.sin() * RADIUS;
                    Point::new(x + RADIUS, y + RADIUS) + q_delta
                })
                .collect::<Vec<Point<f64>>>();
            console_log!("Created q ({} vertices)", q.len());
            assert!(sort_point_set_clockwise(&mut q).is_ok());
            console_log!(
                "{}",
                q.iter()
                    .fold(
                        format!("Sorted q ({} vertices):", q.len()),
                        |acc, q| acc + &format!("\n\t{:?}", q)
                    )
            );
            context.set_line_width(1.);
            context.set_fill_style(&JsValue::from_str("#F8F6F0"));
            context.fill_rect(0., 0., 600., 400.);
            context.set_stroke_style(&JsValue::from_str("#000080"));
            context.begin_path();
            let last_p_idx = p.len() - 1;
            let last_q_idx = q.len() - 1;
            context.move_to(p[last_p_idx].x, p[last_p_idx].y);
            p.iter().for_each(|p| context.line_to(p.x, p.y));
            context.stroke();
            context.set_fill_style(&JsValue::from_str("#000080"));
            p.iter().enumerate().for_each(|(i, p)| context.fill_text(&format!("P{}", i), p.x, p.y - 5.).expect("Could not label p points"));
            context.set_stroke_style(&JsValue::from_str("#604000"));
            context.begin_path();
            context.move_to(q[last_q_idx].x, q[last_q_idx].y);
            q.iter().for_each(|q| context.line_to(q.x, q.y));
            context.stroke();
            context.set_fill_style(&JsValue::from_str("#604000"));
            q.iter().enumerate().for_each(|(i, q)| context.fill_text(&format!("Q{}", i), q.x, q.y - 5.).expect("Could not label q points"));
            console_log!("Graphed p and q");
            let r = ConvexPolygon::intersect_convex_point_sets(&p, &q);
            console_log!("Calculated r (intersection)");
            match &r {
                &Some(ref r) => {
                    context.set_line_width(1.75);
                    context.set_fill_style(&JsValue::from_str("rgba(255, 255, 0, 0.5)"));
                    context.set_stroke_style(&JsValue::from_str("#FF0000"));
                    context.begin_path();
                    let last_r_idx = r.len() - 1;
                    context.move_to(r[last_r_idx].x, r[last_r_idx].y);
                    r.iter().for_each(|r| context.line_to(r.x, r.y));
                    context.fill();
                    context.begin_path();
                    context.move_to(r[last_r_idx].x, r[last_r_idx].y);
                    r.iter().for_each(|r| context.line_to(r.x, r.y));
                    context.stroke();
                },
                _ => {
                    context.set_fill_style(&JsValue::from_str("black"));
                    assert!(context.fill_text("No intersection", 5., 395.).is_ok());
                },
            }
            if r.is_some() {
                console_log!("Graphed r");
            }
            else {
                console_log!("No intersection to graph");
            }
            //
            let question_label = doc
                .create_element("label")
                .and_then(|button| button.dyn_into::<HtmlLabelElement>().map_err(|obj| obj.unchecked_into()))
                .expect("Couldn't create label");
            question_label.set_inner_html("Is the intersection correct?");
            let yes_button = doc
                .create_element("button")
                .and_then(|button| button.dyn_into::<HtmlButtonElement>().map_err(|obj| obj.unchecked_into()))
                .expect("Couldn create button");
            yes_button.set_inner_html("Yes");
            let no_button = doc
                .create_element("button")
                .and_then(|button| button.dyn_into::<HtmlButtonElement>().map_err(|obj| obj.unchecked_into()))
                .expect("Couldn't create button");
            no_button.set_inner_html("No");
            let br0 = doc.create_element("br").expect("Can't create <br/> node");
            let br1 = doc.create_element("br").expect("Can't create <br/> node");
            let br2 = doc.create_element("br").expect("Can't create <br/> node");
            console_log!("Created result-assessment controls");
            let click_mem       : Rc<Cell<Option<(Closure<dyn FnMut()>, Closure<dyn FnMut()>)>>>
                                = Rc::new(Cell::new(None));
            let prom_mem        : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                                = Rc::new(Cell::new(None));
            let mut prom_fn     = {
                let body            = body.clone();
                let click_mem       = click_mem.clone();
                let question_label  = question_label.clone();
                let yes_button      = yes_button.clone();
                let no_button       = no_button.clone();
                let br0             = br0.clone();
                let br1             = br1.clone();
                let br2             = br2.clone();
                let prom_mem        = prom_mem.clone();
                move |resolve: Function, reject: Function| {
                    let click_yes   = {
                        let body            = body.clone();
                        let click_mem       = click_mem.clone();
                        let question_label  = question_label.clone();
                        let yes_button      = yes_button.clone();
                        let no_button       = no_button.clone();
                        let br0             = br0.clone();
                        let br1             = br1.clone();
                        let br2             = br2.clone();
                        let prom_mem        = prom_mem.clone();
                        let prom_mem        = prom_mem.clone();
                        let resolve         = resolve.clone();
                        Closure::wrap({
                            Box::new(move || {
                                let _ = body.remove_child(&question_label);
                                let _ = body.remove_child(&yes_button);
                                let _ = body.remove_child(&no_button);
                                let _ = body.remove_child(&br0);
                                let _ = body.remove_child(&br1);
                                let _ = body.remove_child(&br2);
                                yes_button.set_onclick(None);
                                no_button.set_onclick(None);
                                let _ = resolve.call0(&JsValue::NULL);
                                prom_mem.set(None);
                                click_mem.set(None);
                            }) as Box<dyn FnMut()>
                        })
                    };
                    let click_no    =  {
                        let body            = body.clone();
                        let click_mem       = click_mem.clone();
                        let question_label  = question_label.clone();
                        let yes_button      = yes_button.clone();
                        let no_button       = no_button.clone();
                        let br0             = br0.clone();
                        let br1             = br1.clone();
                        let br2             = br2.clone();
                        let prom_mem        = prom_mem.clone();
                        Closure::wrap({
                            Box::new(move || {
                                let _ = body.remove_child(&question_label);
                                let _ = body.remove_child(&yes_button);
                                let _ = body.remove_child(&no_button);
                                let _ = body.remove_child(&br0);
                                let _ = body.remove_child(&br1);
                                let _ = body.remove_child(&br2);
                                yes_button.set_onclick(None);
                                no_button.set_onclick(None);
                                let _  = reject.call0(&JsValue::NULL);
                                prom_mem.set(None);
                                click_mem.set(None);
                            }) as Box<dyn FnMut()>
                        })
                    };
                    yes_button.set_onclick(Some(click_yes.as_ref().unchecked_ref()));
                    no_button.set_onclick(Some(click_no.as_ref().unchecked_ref()));
                    click_mem.set(Some((click_yes, click_no)));
                    console_log!("Created and registered click callbacks");
                }
            };
            let prom = Promise::new(&mut prom_fn);
            console_log!("Created promise");
            prom_mem.set(Some(Box::new(prom_fn)));
            console_log!("Memorized promise");
            assert!(body.append_child(&br0).is_ok());
            assert!(body.append_child(&question_label).is_ok());
            assert!(body.append_child(&br1).is_ok());
            assert!(body.append_child(&yes_button).is_ok());
            assert!(body.append_child(&br2).is_ok());
            assert!(body.append_child(&no_button).is_ok());
            no_button.scroll_into_view();
            console_log!("Inserted and scrolled to result-assessment controls");
            assert!(<JsFuture as From<Promise>>::from(prom).await.is_ok());
        }
        assert!(body.remove_child(&canvas).is_ok());
    }
}
