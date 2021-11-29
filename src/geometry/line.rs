use decorum::{Finite, Real};

use crate::geometry::error::*;
use crate::geometry::*;
use crate::geometry::{Intersects, Offset};
use std::backtrace::Backtrace;

// This way of defining a line segment on the euclidean plane is useful for offsetting at right
// angles to the direction of the line; we need only add or subtract from distance_from_origin.
#[derive(Copy, Clone, Debug)]
pub struct Line<T: Value> {
    pub angle: Angle<T>,
    // distance_from_origin, can be negative for lines of different orientation
    pub distance_from_origin: Finite<T>,
    // stop and start are the distance from the point on the line closest to the origin.
    // If stop < start, then the line is considered to have negative length, and no
    // points exist on the line; this is usually not desired.
    begin: Finite<T>,
    end: Finite<T>,
}

impl<T: Value> Line<T> {
    pub fn new(start: Point<T>, stop: Point<T>) -> CurvyResult<Self> {
        if start == stop {
            return curvy_err!("Start, stop points are the same");
        }

        let line_delta = stop - start;
        let line_angle: Angle<T> = line_delta.into();

        let d1 = (start - Point::origin()).rotate(-line_angle);
        let d2 = (stop - Point::origin()).rotate(-line_angle);

        let begin = d1.dx;
        let end = d2.dx;

        assert_gt!(end, begin);

        let (distance_from_origin, begin, end) = if begin < end {
            (d1.dy, begin, end)
        } else {
            (-d1.dy, end, begin)
        };

        Ok(Self {
            angle: line_angle,
            distance_from_origin,
            begin,
            end,
        })
    }

    pub fn from_point_angle(
        start: Point<T>,
        angle: Angle<T>,
        length: Finite<T>,
    ) -> CurvyResult<Self> {
        let stop = start + Delta::magnitude_angle(length, angle);
        return Self::new(start, stop);
    }

    // Return a line that occupies the same space, but has opposite directionality.
    pub fn reversed(self) -> Self {
        Self {
            angle: self.angle + AngleDiff(Finite::<T>::PI),
            distance_from_origin: -self.distance_from_origin,
            begin: self.begin,
            end: self.end,
        }
    }

    pub fn point_along(self, signed_distance: Finite<T>) -> Point<T> {
        self.point_nearest_origin()
            + Delta::magnitude_angle(signed_distance, self.angle)
    }

    pub fn point_nearest_origin(self) -> Point<T> {
        Point::origin()
            + Delta::magnitude_angle(
                self.distance_from_origin,
                self.angle + AngleDiff(Finite::<T>::FRAC_PI_2),
            )
    }

    pub fn apply_bounded(self, t: Finite<T>) -> Option<Point<T>> {
        if t >= self.begin && t <= self.end {
            Some(self.apply(t))
        } else {
            None
        }
    }

    pub fn apply(self, t: Finite<T>) -> Point<T> {
        self.point_nearest_origin() + Delta::magnitude_angle(t, self.angle)
    }

    // Distance along line, from its point nearest the origin, for any point.
    pub fn signed_distance(self, point: Point<T>) -> Finite<T> {
        let delta = point - self.point_nearest_origin();
        return delta.rotate(-self.angle).dx;
    }

    pub fn length(self) -> Finite<T> {
        self.end - self.begin
    }

    pub fn begin(self) -> Finite<T> {
        self.begin
    }

    pub fn end(self) -> Finite<T> {
        self.end
    }

    pub fn start(self) -> Point<T> {
        self.apply(self.begin)
    }

    pub fn stop(self) -> Point<T> {
        self.apply(self.end)
    }

    pub fn herefrom(self, point: Point<T>) -> Self {
        Line {
            angle: self.angle,
            distance_from_origin: self.distance_from_origin,
            begin: self.signed_distance(point),
            end: self.end,
        }
    }

    pub fn until(self, point: Point<T>) -> Self {
        Line {
            angle: self.angle,
            distance_from_origin: self.distance_from_origin,
            begin: self.begin,
            end: self.signed_distance(point),
        }
    }
}

impl<T: Value> Offset<T> for Line<T> {
    type OffsetResult = Self;
    fn offset(self, offset: Finite<T>) -> Self::OffsetResult {
        Self {
            angle: self.angle,
            distance_from_origin: self.distance_from_origin + offset,
            begin: self.begin,
            end: self.end,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LineIntersection<T: Value> {
    None,
    OutOfBounds(Point<T>),
    OnePoint(Point<T>),
    Many,
    ManyOutOfBounds,
}

impl<T: Value> Intersects<Line<T>> for Line<T> {
    type Intersection = LineIntersection<T>;

    fn intersect(self, other: &Line<T>) -> Self::Intersection {
        if self.angle == other.angle {
            if self.distance_from_origin == other.distance_from_origin {
                if self.begin() > other.end() || other.begin() > self.end() {
                    return LineIntersection::ManyOutOfBounds;
                } else if self.begin() == other.end() {
                    return LineIntersection::OnePoint(self.point_along(self.begin()));
                } else if other.begin() == self.end() {
                    return LineIntersection::OnePoint(
                        other.point_along(other.begin()),
                    );
                } else {
                    // TODO: return line?
                    return LineIntersection::Many;
                }
            } else {
                // parallel lines that never intersect
                return LineIntersection::None;
            }
        }
        // Now we know there is at most one unique possible intersection.
        let self_point = self.point_nearest_origin();
        let other_point = other.point_nearest_origin();

        let origin = Point::origin();

        let self_delta = self_point - origin;
        let other_delta = other_point - origin;

        let A = self_delta.magnitude();
        let a = self_delta.angle().radians();
        let B = other_delta.magnitude();
        let b = other_delta.angle().radians();
        let sin_a = a.sin();
        let sin_b = b.sin();
        let cos_a = a.cos();
        let cos_b = b.cos();
        let denominator = cos_a * sin_b - sin_a * cos_b;
        let x = (A * sin_b - B * sin_a) / denominator;
        let y = (B * cos_a - A * cos_b) / denominator;
        let point = Point {x, y};

        let self_t = self.signed_distance(point);
        if self_t < self.begin() || self_t > self.end() {
            return LineIntersection::OutOfBounds(point);
        }

        let other_t = other.signed_distance(point);
        if other_t < other.begin() || other_t > other.end() {
            return LineIntersection::OutOfBounds(point);
        }

        LineIntersection::OnePoint(point)
    }
}
