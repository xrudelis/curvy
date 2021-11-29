use std::cmp::min;

use decorum::Finite;
use num_traits::identities::Zero;

use crate::geometry::line::{Line, LineIntersection};
use crate::geometry::*;
use crate::geometry::{Intersects, Offset};

#[derive(Clone, Debug)]
pub struct Polyline<T: Value>(Vec<Point<T>>);

impl<'a, T: Value> Polyline<T> {
    pub fn points(&'a self) -> &'a Vec<Point<T>> {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct Polygon<T: Value>(Vec<Point<T>>);

impl<'a, T: Value> Polygon<T> {
    pub fn points(&'a self) -> &'a Vec<Point<T>> {
        &self.0
    }
}

// Generalization of polyline which includes the amount of each line to devote towards smoothing
// by circular arc. The first and last points have no smoothing info, so curve_size has two fewer
// entries than polyline.
#[derive(Clone, Debug)]
pub struct Polyarc<T: Value> {
    polyline: Polyline<T>,
    curve_sizes: Vec<Finite<T>>,
}

// Generalization of polygon which includes the amount of each line to devote towards smoothing
// by circular arc.
#[derive(Clone, Debug)]
pub struct Polycurve<T: Value> {
    polygon: Polygon<T>,
    curve_sizes: Vec<Finite<T>>,
}

pub trait Segmented<T: Value> {
    type SegmentIterator: Iterator;
    fn iter_segments(self) -> Self::SegmentIterator;
}

pub struct PolylineSegmentIterator<'a, T: Value> {
    index: usize,
    polyline: &'a Polyline<T>,
}

impl<'a, T: Value> Segmented<T> for &'a Polyline<T> {
    type SegmentIterator = PolylineSegmentIterator<'a, T>;
    fn iter_segments(self) -> Self::SegmentIterator {
        PolylineSegmentIterator {
            index: 0,
            polyline: self,
        }
    }
}

impl<'a, T: Value> Iterator for PolylineSegmentIterator<'a, T> {
    type Item = Line<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 >= self.polyline.0.len() {
            return None;
        }
        let start_point = self.polyline.0[self.index];
        let end_point = self.polyline.0[self.index + 1];
        self.index += 1;
        return Some(Line::new(start_point, end_point).unwrap());
    }
}

pub struct PolygonSegmentIterator<'a, T: Value> {
    index: usize,
    polygon: &'a Polygon<T>,
}

impl<'a, T: Value> Segmented<T> for &'a Polygon<T> {
    type SegmentIterator = PolygonSegmentIterator<'a, T>;
    fn iter_segments(self) -> Self::SegmentIterator {
        PolygonSegmentIterator {
            index: 0,
            polygon: self,
        }
    }
}

impl<'a, T: Value> Iterator for PolygonSegmentIterator<'a, T> {
    type Item = Line<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 == self.polygon.0.len() {
            // Polygon wraps around
            let start_point = self.polygon.0[self.index];
            let end_point = self.polygon.0[0];
            self.index += 1;
            return Some(Line::new(start_point, end_point).unwrap());
        } else if self.index >= self.polygon.0.len() {
            return None;
        }
        let start_point = self.polygon.0[self.index];
        let end_point = self.polygon.0[self.index + 1];
        self.index += 1;
        return Some(Line::new(start_point, end_point).unwrap());
    }
}

pub trait Curved<T: Value> {
    type CurvedResult;
    fn curve(&self, size: Finite<T>) -> Self::CurvedResult;
}

// Create a Polyarc from a Polyline by a constant curve size
impl<T: Value> Curved<T> for Polyline<T> {
    type CurvedResult = Polyarc<T>;

    fn curve(&self, size: Finite<T>) -> Self::CurvedResult {
        let n_points = self.0.len();
        // All polylines have at least two points
        assert!(n_points >= 2);
        let two: Finite<T> = Finite::<T>::from_inner(T::from_f64(2.0).unwrap());
        let mut curve_sizes = Vec::<Finite<T>>::with_capacity(n_points - 2);
        let mut prev_line_length: Option<Finite<T>> = None;
        for line in self.iter_segments() {
            let line_length = line.length();
            // Curve is limited by half the line length of either segment at this point.
            if let Some(prev_line_length) = prev_line_length {
                let curve_size = min(min(line_length, prev_line_length) / two, size);
                curve_sizes.push(curve_size);
            }
            prev_line_length = Some(line_length);
        }
        Polyarc {
            polyline: self.clone(),
            curve_sizes,
        }
    }
}

// Create a Polycurve from a Polygon by a constant curve size
impl<T: Value> Curved<T> for Polygon<T> {
    type CurvedResult = Polycurve<T>;

    fn curve(&self, size: Finite<T>) -> Self::CurvedResult {
        let n_points = self.0.len();
        // All polygons have at least three points
        assert!(n_points >= 3);
        let two = Finite::<T>::from_inner(T::from_f64(2.0).unwrap());
        let mut curve_sizes = Vec::<Finite<T>>::with_capacity(n_points);
        let mut prev_line_length: Option<Finite<T>> = None;
        let mut first_line_length: Option<Finite<T>> = None;
        // Placeholder curve size for the first point of the polygon, which will be replaced.
        curve_sizes.push(Finite::<T>::zero());
        for line in self.iter_segments() {
            let line_length = line.length();
            if first_line_length.is_none() {
                first_line_length = Some(line_length);
            }
            // Curve is limited by half the line length of either segment at this point.
            if let Some(prev_line_length) = prev_line_length {
                let curve_size = min(min(line_length, prev_line_length) / two, size);
                curve_sizes.push(curve_size);
            }
            prev_line_length = Some(line_length);
        }
        // Replace placeholder value
        let curve_size = min(
            min(first_line_length.unwrap(), prev_line_length.unwrap()) / two,
            size,
        );
        curve_sizes[0] = curve_size;

        Polycurve {
            polygon: self.clone(),
            curve_sizes,
        }
    }
}

impl<T: Value> Offset<T> for Polyline<T> {
    type OffsetResult = Self;
    fn offset(self, offset: Finite<T>) -> Self::OffsetResult {
        let n_points = self.0.len();
        assert!(n_points >= 2);
        // Build up a temporary list of previous lines which have tentatively correct starting
        // points, but ending points subject to change.
        let mut new_lines: Vec<Line<T>> = Vec::with_capacity(n_points);
        for line in self.iter_segments() {
            let new_line = line.offset(offset);
            loop {
                let prev_line = match new_lines.last() {
                    | Some(prev_line) => prev_line,
                    | None => {
                        new_lines.push(new_line);
                        break;
                    }
                };
                let intersection_point = match new_line.intersect(prev_line) {
                    | LineIntersection::OnePoint(point)
                    | LineIntersection::OutOfBounds(point) => point,
                    | _ => {
                        panic!();
                    }
                };
                // Clip previous line based on intersection to get new connection point
                let prev_line = prev_line.until(intersection_point);
                if prev_line.length() < Finite::<T>::zero() {
                    // Discard previous line, and go back to a previous one
                    new_lines.pop();
                    continue;
                }
                new_lines.push(new_line.herefrom(intersection_point));
                break;
            }
        }
        let mut new_points: Vec<Point<T>> = Vec::with_capacity(n_points);
        for line in &new_lines {
            new_points.push(line.start());
        }
        new_points.push(new_lines.last().unwrap().stop());
        Polyline(new_points)
    }
}

impl<T: Value> Offset<T> for Polygon<T> {
    type OffsetResult = Self;
    fn offset(self, offset: Finite<T>) -> Self::OffsetResult {
        let n_points = self.0.len();
        assert!(n_points >= 3);
        // Build up a temporary list of previous lines which have tentatively correct starting
        // points, but ending points subject to change.
        let mut new_lines: Vec<Line<T>> = Vec::with_capacity(n_points);
        for line in self.iter_segments() {
            let new_line = line.offset(offset);
            loop {
                let prev_line = match new_lines.last() {
                    | Some(prev_line) => prev_line,
                    | None => {
                        new_lines.push(new_line);
                        break;
                    }
                };
                let intersection_point = match new_line.intersect(prev_line) {
                    | LineIntersection::OnePoint(point)
                    | LineIntersection::OutOfBounds(point) => point,
                    | _ => {
                        panic!();
                    }
                };
                // Clip previous line based on intersection to get new connection point
                let prev_line = prev_line.until(intersection_point);
                if prev_line.length() < Finite::<T>::zero() {
                    // Discard previous line, and go back to a previous one
                    new_lines.pop();
                    continue;
                }
                new_lines.push(new_line.herefrom(intersection_point));
                break;
            }
        }
        // Close ends by revisiting the first line
        let new_line = new_lines[0].offset(offset);
        loop {
            let prev_line = match new_lines.last() {
                | Some(prev_line) => prev_line,
                | None => {
                    new_lines.push(new_line);
                    break;
                }
            };
            let intersection_point = match new_line.intersect(prev_line) {
                | LineIntersection::OnePoint(point)
                | LineIntersection::OutOfBounds(point) => point,
                | _ => {
                    panic!();
                }
            };
            // Clip previous line based on intersection to get new connection point
            let prev_line = prev_line.until(intersection_point);
            if prev_line.length() < Finite::<T>::zero() {
                // Discard previous line, and go back to a previous one
                new_lines.pop();
                continue;
            }
            new_lines.push(new_line.herefrom(intersection_point));
            break;
        }
        new_lines[0] = new_lines.pop().unwrap();

        let mut new_points: Vec<Point<T>> = Vec::with_capacity(n_points);
        for line in &new_lines {
            new_points.push(line.start());
        }
        Polygon(new_points)
    }
}

impl<T: Value> Offset<T> for Polyarc<T> {
    type OffsetResult = Self;
    fn offset(self, offset: Finite<T>) -> Self::OffsetResult {
        // note: need to turn all convex points into actual arcs, but not concave
        // note: need to calculate intersections between arcs and lines, probably?
        todo!()
    }
}

impl<T: Value> Offset<T> for Polycurve<T> {
    type OffsetResult = Self;
    fn offset(self, offset: Finite<T>) -> Self::OffsetResult {
        todo!()
    }
}
