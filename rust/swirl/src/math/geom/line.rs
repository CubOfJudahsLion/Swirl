//! Module  :   Line
//! Purpose :   Basic operations on representations of a line.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Wed 12 Jan 2022 04:36:50 PM CST
//! Modified:   Wed 09 Feb 2022 08:57:41 AM CST


use std::ops::Deref;

use super::super::numeric::Floating;
use super::point::*;


/// Self-explanatory. Direct `LineEquation` creation
/// can fail under the listed conditions.
pub enum LineEquationCreationError {
    BCannotBeZero,
    NaNAndInfiniteDisallowed,
}


/// Representation of the equation of a line.
#[derive(Clone, Debug)]
pub struct LineEquation<N> {
    //  We'll use the Ax+By=C form. It's easier
    //  to manipulate in equation systems and
    //  it makes the parallel vector (i.e, `(a,b)`)
    //  easily apparent.
    a: N,
    b: N,
    c: N,
}

impl<N: Copy> LineEquation<N> {
    #[inline(always)]
    pub fn a(&self) -> N { self.a }

    #[inline(always)]
    pub fn b(&self) -> N { self.b }

    #[inline(always)]
    pub fn c(&self) -> N { self.c }

    /// Returns a vector perpendicular to that of
    /// the line which points towards the 'inside'
    /// of it.
    #[inline]
    pub fn perp_vec(&self) -> Point<N> {
        Point::<N>::new(self.a, self.b)
    }
}

impl<N> LineEquation<N>
    where   N           : Floating + From<i16>,
            Point<N>    : RealPoint<N>
{
    /// Creates a `LineEquation` directly from its eventual components,
    /// as long as `b` is not zero and none are `NaN` or infinite.
    pub fn new(a: N, b: N, c: N) -> Result<Self, LineEquationCreationError> {
        if b == N::from(0) {
            Err(LineEquationCreationError::BCannotBeZero)
        }
        else if a.is_nan() || a.is_infinite()
            ||  b.is_nan() || b.is_infinite()
            ||  c.is_nan() || c.is_infinite()
        {
            Err(LineEquationCreationError::NaNAndInfiniteDisallowed)
        }
        else {
            Ok(LineEquation{a, b, c,})
        }
    }

    /// Calculates a line equation from two points. The line will
    /// have a direction, i.e., it will go _from_ `p1` _to_ `p2`.
    /// If parameter order were reversed, all of the factors
    /// would be sign-inverted.
    pub fn new_from_points(p1: &Point<N>, p2: &Point<N>) -> LineEquation<N> {
        let dy  = p2.y - p1.y;
        let dx  = p2.x - p1.x;
        LineEquation {
            a   : -dy,
            b   : dx,
            c   : dx * p1.y - dy * p1.x,
        }
    }

    /// Applies the line as a function.
    pub fn apply_as_function(&self, x: N) -> N {
        (self.c - self.a * x) / self.b
    }

    /// Checks whether a point is "inside" the line. As **SWIRL** prefers
    /// a clockwise representation of lines (and polygons), this means every
    /// point that, if we turn the graph so the line direction is "up",
    /// lies to the right of the line.
    pub fn point_inside(&self, p: &Point<N>) -> bool {
        //  Yes, the test is that simple. It's a dot-product minus
        //  an offset. The offshot is that anything yielding a
        //  positive value is 'inside'.
        self.a * p.x + self.b * p.y - self.c > N::from(0)
    }

    /// Finds the intersection of two lines -- if any --
    /// using an abbreviated Gaussian Elimination method.
    pub fn intersection(l: &LineEquation<N>, m: &LineEquation<N>) -> Option<Point<N>> {
        //  We'll begin with the following augmented matrix:
        //  [ l.a  l.b | l.c ]
        //  [ m.a  m.b | m.c ]
        //  We'd train to obtain a diagonal matrix, but since
        //  our input matrix is so small, we can not just skip
        //  the iterative arrangements but skip a few steps.
        let zero    = N::from(0);
        //  Calculate a determinant first.
        let det     = l.a * m.b - l.b * m.a;
        //  A zero determinant means no solution.
        if det == zero {
            None
        }
        else {
            let (x_row, y_row)  =
                if /*m.b == zero ||*/ l.a == zero {
                    (&m, &l)
                }
                else {
                    (&l, &m)
                };
            let y               =
                if y_row.a == zero {
                    //  If x is zero, then we have by = c, so y = c/b.
                    y_row.c / y_row.b
                }
                else {
                    //  Otherwise, Gaussian elimination dictates
                    //  scaling and adding the x-row to nullify the
                    //  x-variable in the y-row and leave us with
                    //  by = c.
                    let x_elim_factor = -y_row.a / x_row.a;
                    (x_row.c * x_elim_factor + y_row.c) / (x_row.b * x_elim_factor + y_row.b)
                };
            let x               =
                //  The same ideas are used here,
                //  just in different variables.
                if x_row.b == zero {
                    x_row.c / x_row.a
                }
                else {
                    let y_elim_factor = -x_row.b / y_row.b;
                    (y_row.c * y_elim_factor + x_row.c) / (y_row.a * y_elim_factor + x_row.a)
               };
            Some(Point::new(x, y))
        }
    }

    /// Determines whether the segment `p0->p1` points towards this line.
    pub fn points_towards_self(&self, (p0, p1): (&Point<N>, &Point<N>)) -> bool {
        let p0_in   = self.point_inside(p0);
        if p0_in != self.point_inside(p1) {
            //  Vectors that cross the line are
            //  always pointing away from it.
            false
        }
        else {
            let zero    = N::from(0);
            let p_diff  = p1 - p0;
            let perp    = self.perp_vec();
            if p0_in {
                //  If p0 and p1 are inside, then they must point
                //  in opposite direction to the perpendicular
                //  (a negative dot-product) to be pointing at
                //  the line.
                perp.dot(&p_diff) < zero
            }
            else {
                //  If they're out, their dot-product with the
                //  perpendicular would be positive.
                perp.dot(&p_diff) > zero
            }
        }
    }
}

/// Equality for `LineEquation`s happens only when
/// all three components are equal.
impl<N: PartialEq> PartialEq<LineEquation<N>> for LineEquation<N> {
    fn eq(&self, rhs: &LineEquation<N>) -> bool {
            self.a == rhs.a
        &&  self.b == rhs.b
        &&  self.c == rhs.c
    }
}


//  Handy limit checking function that we'll use plenty below.
#[inline]
fn inside_limits<N: PartialOrd + Copy>((limit0, limit1): (N, N), value: N) -> bool {
    let (left_limit, right_limit) =
        if limit0 < limit1 {
            (limit0, limit1)
        }
        else {
            (limit1, limit0)
        };
    left_limit <= value && value <= right_limit
}


/// A **Line Segment** that is described by
/// - Its two endpoints
/// - A corresponding `LineEquation`, describing the line that extends
///   beyond said endpoints.
pub struct LineSegment<N> {
    p0      : Point<N>,
    p1      : Point<N>,
    line_eq : LineEquation<N>,
}

impl<N> LineSegment<N>
    where   N           : Floating + From<i16>,
            Point<N>    : RealPoint<N>
{
    /// Creates a line segment from its endpoints. Note that
    /// order is important: the segment is considered to have
    /// *direction*, going from `p0` to `p1`, and so does the
    /// derived `LineEquation`.
    pub fn new(p0: &Point<N>, p1: &Point<N>) -> Self {
        LineSegment {
            p0      : p0.clone(),
            p1      : p1.clone(),
            line_eq : LineEquation::new_from_points(p0, p1),
        }
    }

    /// Obtain the two endpoints (provided in order.)
    #[inline(always)]
    pub fn endpoints(&self) -> (&Point<N>, &Point<N>) {
        (&self.p0, &self.p1)
    }

    /// Calculates whether a single point lies in the segment, with a given tolerance.
    pub fn point_in_segment(&self, q: &Point<N>, tol: N) -> bool {
            inside_limits((self.p0.x, self.p1.x), q.x)
        &&  N::is_close(
                self.line_eq.apply_as_function(q.x),
                q.y,
                tol
            )
    }

    /// Calculates the possible intersection of two line segments.
    pub fn intersection(s: &Self, t: &Self) -> Option<Point<N>> {
        let int = LineEquation::intersection(&s.line_eq, &t.line_eq)?;
        //  Another handy limit checking function -- sans limit sorting.
        #[inline(always)]
        fn inside_ordered_limits<N: PartialOrd>((left_limit, right_limit): (N, N), value: N) -> bool {
            left_limit <= value && value <= right_limit
        }
        //  Obtains the narrowest limits from two limit sets.
        fn narrowest<N: Floating>(pts: &[(N, N); 2]) -> (/*min*/ N, /*max*/ N) {
            let pair0       = if pts[0].0 < pts[0].1 { (pts[0].0, pts[0].1) } else { (pts[0].1, pts[0].0) };
            let pair1       = if pts[1].0 < pts[1].1 { (pts[1].0, pts[1].1) } else { (pts[1].1, pts[1].0) };
            (N::max(pair0.0, pair1.0), N::min(pair0.1, pair1.1))
        }
        //
        if      inside_ordered_limits(narrowest(&[(s.p0.x, s.p1.x), (t.p0.x, t.p1.x)]), int.x)
            &&  inside_ordered_limits(narrowest(&[(s.p0.y, s.p1.y), (t.p0.y, t.p1.y)]), int.y)
        {
            Some(int)
        }
        else {
            None
        }
    }
}

impl<N> Deref for LineSegment<N> {
    type Target = LineEquation<N>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.line_eq
    }
}


#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    #[wasm_bindgen_test]
    fn line_equation_construction() {
        for m1 in 0 .. 6 {
            for b1 in 0 .. 6 {
                let f1 = |x: f32| (m1 as f32) * x + (b1 as f32);
                let l1 = LineEquation::new_from_points(&Point::new(1., f1(1.)), &Point::new(5., f1(5.)));
                for x in 0 .. 10 {
                    assert_eq!(f1(x as f32), l1.apply_as_function(x as f32));
                }
            }
        }
    }

    //  Equality test that takes into account limited precision.
    fn reasonably_equal(a: f32, b: f32) -> bool {
        const DELTA: f32 = 0.0005; //  f32 precision stinks, does't it.
        f32::abs(a - b) <= DELTA
    }

    #[wasm_bindgen_test]
    fn known_line_intersections() {
        struct LineEqTest {
            m1: f32, o1: f32,           // First equation (o instead of b to avoid conflict)
            a1: f32, b1: f32, c1: f32,  // in linear form
            m2: f32, o2: f32,           // Second equation
            a2: f32, b2: f32, c2: f32,
            int_x: f32,                 // X coord of intersection
            int_y: f32,                 // Y coord
        }
        let known_intersections: &[LineEqTest] = &[
            LineEqTest {m1: 5., o1:  1., a1: -20., b1: 4., c1:   4.,
                        m2: 2., o2:  5., a2:  -8., b2: 4., c2:  20.,
                        int_x: 4./3., int_y: 23./3.,},
            LineEqTest {m1: 3., o1: -8., a1: -12., b1: 4., c1: -32.,
                        m2: 1., o2: -4., a2:  -4., b2: 4., c2: -16.,
                        int_x: 2.,    int_y: -2.,},
            LineEqTest {m1: 0., o1:  7., a1:   0., b1: 4., c1:  28.,
                        m2: 3., o2:  0., a2: -12., b2: 4., c2:   0.,
                        int_x: 7./3., int_y:  7.},
        ];
        //
        for int in known_intersections {
            let f1 = |x: f32| (int.m1 as f32) * x + (int.o1 as f32);
            let f2 = |x: f32| (int.m2 as f32) * x + (int.o2 as f32);
            console_log!(
                "Testing: {}x + {} ({}x + {}y = {}) vs {}x + {} ({}x + {}y = {}), 
                expected intersection at ({}, {}).",
                int.m1, int.o1, int.a1, int.b1, int.c1,
                int.m2, int.o2, int.a2, int.b2, int.c2,
                int.int_x, int.int_y
            );
            let l1 = LineEquation::new_from_points(&Point::new(1., f1(1.)), &Point::new(5., f1(5.)));
            let expected1 = LineEquation {a: int.a1, b: int.b1, c: int.c1,};
            assert_eq!(&l1, &expected1, "{:?} is not the same as {:?}", &l1, &expected1);
            let l2 = LineEquation::new_from_points(&Point::new(1., f2(1.)), &Point::new(5., f2(5.)));
            let expected2 = LineEquation {a: int.a2, b: int.b2, c: int.c2,};
            assert_eq!(&l2, &expected2, "{:?} is not the same as {:?}", &l2, &expected2);
            let p  = LineEquation::intersection(&l1, &l2);
            assert!(p.is_some());
            let p  = p.unwrap();
            assert!(
                reasonably_equal(int.int_y, p.y),
                "expected Y intersection {} isn't reasonably close to obtained intersection {}",
                int.int_y, p.y
            );
            assert!(
                reasonably_equal(int.int_x, p.x),
                "expected X intersecton {} isn't reasonably close to obtained intersection {}",
                int.int_x, p.x
            );
            assert!(
                reasonably_equal(p.y, f1(p.x)),
                "obtained Y {} isn't reasonably close to f1(x) = {}",
                p.y, f1(p.x)
            );
            assert!(
                reasonably_equal(p.y, f2(p.x)),
                "obtained Y {} isn't reasonably close to f2(x) = {}",
                p.y, f2(p.x)
            );
        }
    }

    #[wasm_bindgen_test]
    fn inside_line() {
        use rand::prelude::*;
        use super::super::transform::Transform;
        let mut rng =   thread_rng();
        for _ in 0 .. 10 {
            //  First, we generate a line equation at random.
            let m       =   rng.gen_range(-10.0f32 ..= 10.);
            let b       =   rng.gen_range(-10.0f32 ..= 10.);
            let f       =   |x: f32| m * x + b;
            let p1_x    =   rng.gen_range(-10.0f32 ..= 10.);
            let mut p2_x: f32;
            loop {
                p2_x = rng.gen_range(-10. ..= 10.);
                if p2_x != p1_x {
                    break;
                }
            }
            let p2_x    =   p2_x;
            let p1      =   Point::new(p1_x, f(p1_x));
            let p2      =   Point::new(p2_x, f(p2_x));
            let l       =   LineEquation::new_from_points(&p1, &p2);
            //  Now we calculate an angle between the two points...
            let angle   =   f32::atan2(p2.y - p1.y, p2.x - p1.x);
            //  ...and create a transformation from it,
            let trans   =   Transform::new_rotation(-angle);
            let p1      =   p1 * &trans;
            let p2      =   p2 * &trans;
            //  so that the two points will have the same _y_.
            assert!(reasonably_equal(p1.y, p2.y));
            //  Now comes the fun part: we'll generate a series
            //  of points at random.
            let pts     =   (0 .. 25)
                .map(|_| Point::new(
                    rng.gen_range(-10.0 ..= 10.0f32),
                    rng.gen_range(-10.0 ..= 10.0)
                ))
                .collect::<Vec<_>>();
            pts.iter().for_each(|p| assert!(((p * &trans).y >= p2.y) == l.point_inside(p))); 
        }
    }

    #[wasm_bindgen_test]
    fn known_segment_intersections() {
        struct SegmentIntTest {
            m1: f32, b1: f32,
            m2: f32, b2: f32,
            tests: [((f32, f32), bool); 3],
        }
        let segment_tests = [
            SegmentIntTest {
                m1: 5., b1:  1.,
                m2: 2., b2:  5.,
                tests: [((-10.,  -5.), false), ((0., 5.), true), ((5., 10.), false)]
            },
            SegmentIntTest {
                m1: 3., b1: -8.,
                m2: 1., b2: -4.,
                tests: [((-10., -5.), false), ((1., 4.), true), ((5., 8.), false)],
            },
            SegmentIntTest {
                m1: 0., b1:  7.,
                m2: 3., b2:  0.,
                tests: [((2., 4.), true), ((7., 12.), false), ((12., 17.), false)],
            },
            SegmentIntTest {
                m1: 2., b1:  7.,
                m2: 8., b2: -4.,
                tests: [((-10., -5.), false), ((-2.5, 2.5), true), ((7.5, 12.5), false)],
            }
        ];
        for seg in segment_tests {
            let f1 = |x: f32| -> f32 { seg.m1 * x + seg.b1 };
            let f2 = |x: f32| -> f32 { seg.m2 * x + seg.b2 };
            for &((x0, x1), intersects) in &seg.tests
            {
                let p0 = Point::new(x0, f1(x0));
                let p1 = Point::new(x1, f1(x1));
                let q0 = Point::new(x0, f2(x0));
                let q1 = Point::new(x1, f2(x1));
                //assert_eq!(line_segment_intersection((&p0, &p1), (&q0, &q1)).is_some(), intersects);
                assert_eq!(
                    LineSegment::intersection(
                        &LineSegment::new(&p0, &p1),
                        &LineSegment::new(&q0, &q1)
                    ).is_some(),
                    intersects
                );
            }
        }
    }

    #[wasm_bindgen_test]
    fn point_in_segment() {
        assert!(
            LineSegment::new(&Point::<f64>::new(1., 1.), &Point::new(3., 3.))
                .point_in_segment(
                    &Point::new(2., 2.),
                    f64::EPSILON * 16.
                )
        );
        assert!(
            !LineSegment::new(&Point::<f64>::new(1., 1.), &Point::new(3., 3.))
                .point_in_segment(
                    &Point::new(2., 2.1),
                    f64::EPSILON * 16.
                )
        );
        assert!(
            !LineSegment::new(&Point::<f64>::new(1., 1.), &Point::new(3., 3.))
                .point_in_segment(
                    &Point::new(4., 4.),
                    f64::EPSILON * 16.
                )
        );
    }

    #[wasm_bindgen_test]
    fn points_to() {
        let p0      = Point::new(0.0f32, 1.);
        let p1      = Point::new(1.0f32, 2.);
        let pair_p  = (&p0, &p1);
        let line_p  = LineEquation::new_from_points(&p0, &p1);
        //  Parallels never point at each other.
        let q0      = Point::new(0.0f32, 0.);
        let q1      = Point::new(1.0f32, 1.);
        let pair_q  = (&q0, &q1);
        let line_q  = LineEquation::new_from_points(&q0, &q1);
        assert!(!line_q.points_towards_self(pair_p));
        assert!(!line_p.points_towards_self(pair_q));
        //  Towards from inside
        assert!(line_p.points_towards_self((&Point::<f32>::new(4., -4.), &Point::new(2., 0.))));
        //  Towards from outside
        assert!(line_p.points_towards_self((&Point::<f32>::new(-4., 1.), &Point::new(0., 2.))));
        //  Away from inside
        assert!(!line_p.points_towards_self((&Point::<f32>::new(2., 0.), &Point::new(4., -4.))));
        //  Away from outside
        assert!(!line_p.points_towards_self((&Point::<f32>::new(0., 2.), &Point::new(-4., 1.))));
        //  Crossing starting from inside
        assert!(!line_p.points_towards_self((&Point::<f32>::new(-2., 3.), &Point::new(3., -1.))));
        //  Crossing starting from outside
        assert!(!line_p.points_towards_self((&Point::<f32>::new(3., -1.), &Point::new(-2., 3.))));
    }
}
