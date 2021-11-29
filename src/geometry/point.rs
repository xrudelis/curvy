use std::fmt;
use std::ops::{Add, Sub};

use approx::AbsDiffEq;
use decorum::Finite;
use num_traits::identities::Zero;

use crate::geometry::*;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point<T: Value> {
    pub x: Finite<T>,
    pub y: Finite<T>,
}

impl<T: Value> Point<T> {
    // Point::new() will panic if x, y are not finite.
    pub fn new(x: T, y: T) -> Self {
        let x = Finite::<T>::from_inner(x);
        let y = Finite::<T>::from_inner(y);
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point {
            x: Finite::<T>::zero(),
            y: Finite::<T>::zero(),
        }
    }

    pub fn midpoint(self: Self, other: Self) -> Self {
        let two = Finite::<T>::from_inner(T::from_f64(2.0).unwrap());
        Point {
            x: (self.x + other.x) / two,
            y: (self.y + other.y) / two,
        }
    }

    pub fn distance(self: Self, other: Point<T>) -> Finite<T> {
        (self - other).magnitude()
    }

    pub fn rotate_about(self: Self, other: Point<T>, angle: Angle<T>) -> Point<T> {
        let delta = self - other;
        let new_delta = delta.rotate(angle);
        other + new_delta
    }
}

impl<T: Value> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl<T: Value> AbsDiffEq<Point<T>> for Point<T> where T::Epsilon: Copy {
    type Epsilon = T::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Point<T>, epsilon: Self::Epsilon) -> bool {
        let x = self.x.into_inner();
        let y = self.y.into_inner();
        let other_x = other.x.into_inner();
        let other_y = other.y.into_inner();
        return x.abs_diff_eq(&other_x, epsilon)
            && y.abs_diff_eq(&other_y, epsilon);
    }
}

impl<T: Value> Add<Delta<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Delta<T>) -> Self::Output {
        Point {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

impl<T: Value> Sub for Point<T> {
    type Output = Delta<T>;

    fn sub(self, other: Self) -> Self::Output {
        Delta {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}
