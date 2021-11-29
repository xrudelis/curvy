use decorum::{Finite, Real};
use num_traits::{One, Zero};

use crate::geometry::error::*;
use crate::geometry::line::{Line, LineIntersection};
use crate::geometry::*;
use crate::geometry::{Intersects, Offset};
use std::backtrace::Backtrace;

// This way of defining a circular arc on the euclidean plane is useful for offsetting at right
// angles to the arc's tangents; we need only add or subtract from radius and everything else is
// constant for any offset.
#[derive(Copy, Clone, Debug)]
pub struct Arc<T: Value> {
    pub center: Point<T>,
    // radius must be positive.
    pub radius: Finite<T>,
    pub start_angle: Angle<T>,
    pub stop_diff: AngleDiff<T>,
}

impl<T: Value> Arc<T> {
    // start_angle is the angle of the line tangent to the arc and intersecting with
    // the start.
    pub fn new(
        start: Point<T>,
        stop: Point<T>,
        angle: Angle<T>,
    ) -> CurvyResult<Self> {
        if start == stop {
            return curvy_err!("Start, stop points are the same");
        }

        // Find the center point, which is the point along a line intersecting start
        // and of slope start_angle +/- 90deg, and equidistant to both start and stop.
        // Length of these two lines doesn't matter for us.
        let one = Finite::<T>::one();
        let _90deg = AngleDiff(Finite::<T>::FRAC_PI_2);
        let start_angle = angle + _90deg;
        let start_perpendicular =
            Line::from_point_angle(start, start_angle, one)?;
        let midpoint_angle = (stop - start).angle();
        let midpoint_perpendicular =
            Line::from_point_angle(start.midpoint(stop), midpoint_angle + _90deg, one)?;

        let center = match start_perpendicular.intersect(&midpoint_perpendicular) {
            | LineIntersection::OnePoint(point)
            | LineIntersection::OutOfBounds(point) => point,
            | _ => {
                return curvy_err!("Undefinable circular arc");
            }
        };

        let stop_delta = stop - center;
        let start_delta = start - center;
        let radius = start_delta.magnitude();
        let stop_diff = stop_delta.angle() - start_delta.angle();

        Ok(Self {
            center,
            radius,
            start_angle,
            stop_diff,
        })
    }

    pub fn from_center(
        center: Point<T>,
        start: Point<T>,
        stop: Point<T>,
    ) -> CurvyResult<Self> {
        let start_delta = start - center;
        let radius = start_delta.magnitude();
        let stop_delta = stop - center;
        // This is an overspecified constructor so we want to use an approximate
        // assertion to make sure it is properly over-specified.
        if abs_diff_ne!(radius.into_inner(), stop_delta.magnitude().into_inner()) {
            return curvy_err!("Undefinable circular arc");
        }
        let start_angle = start_delta.angle();
        let stop_diff = stop_delta.angle() - start_angle;
        Ok(Self {
            center,
            radius,
            start_angle,
            stop_diff,
        })
    }

    pub fn apply_bounded(self, t: Finite<T>) -> Option<Point<T>> {
        if t >= self.begin() && t <= self.end() {
            Some(self.apply(t))
        } else {
            None
        }
    }

    pub fn apply_angle(self, angle: Angle<T>) -> Point<T> {
        return self.center + Delta::magnitude_angle(self.radius, angle);
    }

    pub fn apply(self, t: Finite<T>) -> Point<T> {
        let angle = Angle(t / self.radius);
        return self.center + Delta::magnitude_angle(self.radius, angle);
    }

    pub fn signed_distance(self, point: Point<T>) -> Finite<T> {
        (point - self.center).angle().0 * self.radius
    }

    pub fn begin(self) -> Finite<T> {
        self.start_angle.radians() * self.radius
    }

    pub fn end(self) -> Finite<T> {
        (self.stop_angle()).radians() * self.radius
    }

    // start angle from center
    pub fn start_angle(self) -> Angle<T> {
        self.start_angle
    }

    // stop angle from center
    pub fn stop_angle(self) -> Angle<T> {
        self.start_angle + self.stop_diff
    }

    pub fn length(self) -> Finite<T> {
        self.stop_diff.radians() * self.radius
    }

    pub fn start(self) -> Point<T> {
        self.apply(self.begin())
    }

    pub fn stop(self) -> Point<T> {
        self.apply(self.end())
    }

    pub fn control_point(self) -> Point<T> {
        // If this arc were approximated by two tangent lines at each start and end, give
        // the intersection of those two lines.
        let pi_over_2 = Angle(Finite::<T>::FRAC_PI_2);
        let one = Finite::<T>::one();
        let start_tangent = Line::from_point_angle(
            self.start(),
            (self.start_angle() - pi_over_2).into(),
            one,
        )
        .unwrap();
        let stop_tangent = Line::from_point_angle(
            self.stop(),
            (self.stop_angle() - pi_over_2).into(),
            one,
        )
        .unwrap();
        match start_tangent.intersect(&stop_tangent) {
            | LineIntersection::OnePoint(point)
            | LineIntersection::OutOfBounds(point) => point,
            | _ => {
                panic!();
            }
        }
    }

    // If this arc were approximated by two tangent lines at each start and end, give
    // the distance from the start point to the intersection of these lines.
    pub fn curve_size(self) -> Finite<T> {
        self.start().distance(self.control_point())
    }

    pub fn sweep_flag(self) -> bool {
        matches!(
            self.start_angle().direction(self.stop_angle()),
            Direction::Counterclockwise
        )
    }
}

impl<T: Value> Offset<T> for Arc<T> {
    type OffsetResult = Self;
    fn offset(self, offset: Finite<T>) -> Self::OffsetResult {
        Self {
            center: self.center,
            radius: self.radius + offset,
            start_angle: self.start_angle,
            stop_diff: self.stop_diff,
        }
    }
}

pub enum ArcIntersectionPoint<T: Value> {
    InBounds(Point<T>),
    InArcBounds(Point<T>),
    InLineBounds(Point<T>),
    OutOfBounds(Point<T>),
}

impl<T: Value> ArcIntersectionPoint<T> {
    fn new(on_line_segment: bool, on_circular_arc: bool, point: Point<T>) -> Self {
        match (on_line_segment, on_circular_arc) {
            | (false, false) => ArcIntersectionPoint::OutOfBounds(point),
            | (false, true) => ArcIntersectionPoint::InArcBounds(point),
            | (true, false) => ArcIntersectionPoint::InLineBounds(point),
            | (true, true) => ArcIntersectionPoint::InBounds(point),
        }
    }
}

pub enum ArcIntersection<T: Value> {
    None,
    One(ArcIntersectionPoint<T>),
    Two(ArcIntersectionPoint<T>, ArcIntersectionPoint<T>),
    Many, // For Arc-Arc intersection only
}

impl<T: Value> Intersects<Line<T>> for Arc<T> {
    type Intersection = ArcIntersection<T>;

    fn intersect(self, line: &Line<T>) -> Self::Intersection {
        let line_point = line.point_nearest_origin();
        let line_distance = line.distance_from_origin;

        let delta = line_point - self.center;

        let a = (line_point.x * line_point.x + line_point.y * line_point.y)
            / (line_distance * line_distance);
        let b = (delta.dx * line_point.y - delta.dy * line_point.x) / line_distance;
        let c = delta.dx * delta.dx + delta.dy * delta.dy - self.radius * self.radius;

        let radicand = b * b - a * c;
        if radicand < Finite::<T>::zero() {
            return ArcIntersection::None;
        }

        let line_lower_bound = line.begin();
        let line_upper_bound = line.end();

        let self_min_theta = self.start_angle();
        let self_max_theta = self.stop_angle();

        if radicand == Finite::<T>::zero() {
            // Solutions equivalent
            let solution = -b / a;
            let point = line.point_along(solution);
            let theta = (point - self.center).angle();
            let point_on_line_segment =
                solution >= line_lower_bound && solution < line_upper_bound;
            let point_on_circle_segment = theta.between(self_min_theta, self_max_theta);
            return if point_on_line_segment && point_on_circle_segment {
                ArcIntersection::One(ArcIntersectionPoint::InBounds(point))
            } else {
                ArcIntersection::One(ArcIntersectionPoint::OutOfBounds(point))
            };
        }

        let sqrt = radicand.sqrt();
        let solution1 = (-b + sqrt) / a;
        let solution2 = (-b - sqrt) / a;

        let solution1_on_line_segment =
            solution1 >= line_lower_bound && solution1 < line_upper_bound;
        let solution2_on_line_segment =
            solution2 >= line_lower_bound && solution1 < line_upper_bound;

        let point1 = line.point_along(solution1);
        let point2 = line.point_along(solution2);

        let theta1 = (point1 - self.center).angle();
        let theta2 = (point2 - self.center).angle();

        let solution1_on_circle_segment =
            theta1.between(self_min_theta, self_max_theta);
        let solution2_on_circle_segment =
            theta2.between(self_min_theta, self_max_theta);

        ArcIntersection::Two(
            ArcIntersectionPoint::new(
                solution1_on_line_segment,
                solution1_on_circle_segment,
                point1,
            ),
            ArcIntersectionPoint::new(
                solution2_on_line_segment,
                solution2_on_circle_segment,
                point2,
            ),
        )
    }
}

impl<T: Value> Intersects<Arc<T>> for Line<T> {
    type Intersection = ArcIntersection<T>;
    fn intersect(self, arc: &Arc<T>) -> Self::Intersection {
        arc.intersect(&self)
    }
}

impl<T: Value> Intersects<Arc<T>> for Arc<T> {
    type Intersection = ArcIntersection<T>;
    fn intersect(self, arc: &Arc<T>) -> Self::Intersection {
        todo!()
    }
}
