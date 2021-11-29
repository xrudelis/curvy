use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Neg, Sub};

use decorum::{Finite, Real};
use derive_more::{Add};
use num_traits::Zero;

use crate::geometry::*;

// Angle of value 0 to 2PI. Use this unless you need to know the difference between
// +180deg and -180deg for instance.
#[derive(Clone, Copy, Debug)]
pub struct Angle<T: Value>(pub Finite<T>);

// Angular difference of value -2PI to 2PI
#[derive(Add, Clone, Copy, Debug)]
pub struct AngleDiff<T: Value>(pub Finite<T>);

pub trait Angular<T: Value> {
    fn radians(self) -> Finite<T>;
}

impl<T: Value> Neg for Angle<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        Angle(two_pi - self.0)
    }
}

impl<T: Value> Neg for AngleDiff<T> {
    type Output = Self;
    fn neg(self) -> Self {
        AngleDiff(-self.0)
    }
}

impl<T: Value> Angular<T> for Angle<T> {
    fn radians(self) -> Finite<T> {
        return self.0;
    }
}

impl<T: Value> Angular<T> for AngleDiff<T> {
    fn radians(self) -> Finite<T> {
        return self.0;
    }
}

impl<T: Value> PartialEq for Angle<T> {
    fn eq(&self, other: &Self) -> bool {
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        return self.0 % two_pi == other.0 % two_pi;
    }
}
impl<T: Value> Eq for Angle<T> {}

impl<T: Value> Add<AngleDiff<T>> for Angle<T> {
    type Output = Angle<T>;
    fn add(self, diff: AngleDiff<T>) -> Self::Output {
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        Angle((diff.0 + self.0) % two_pi)
    }
}

impl<T: Value> Sub for Angle<T> {
    type Output = AngleDiff<T>;
    // Angular difference based on shortest direction. Thus the result is always
    // between -PI and PI (-180deg and 180deg).
    fn sub(self, other: Self) -> Self::Output {
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        let pi = Finite::<T>::from_inner(T::from_f64(f64::PI).unwrap());
        AngleDiff(((self.0 - other.0 + pi) % two_pi) - pi)
    }
}

impl<T: Value> From<Delta<T>> for Angle<T> {
    fn from(item: Delta<T>) -> Self {
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        Angle((item.dy.atan2(item.dx) + two_pi) % two_pi)
    }
}

impl<T: Value> std::ops::Mul<Finite<T>> for Angle<T> {
    type Output = Angle<T>;
    fn mul(self, value: Finite<T>) -> Self::Output {
        Angle(self.0 * value)
    }
}

impl<T: Value> fmt::Display for Angle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.radians();
        let frac_180_pi = Finite::<T>::from_inner(T::from_f64(180.0 / f64::PI).unwrap());
        write!(f, "{} ({}deg)", value, value * frac_180_pi)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    None,
    Clockwise,
    Counterclockwise,
}

impl<T: Value> Angle<T> {
    // Angle::new() will panic if theta is not finite
    pub fn new(theta: T) -> Self {
        let theta = Finite::<T>::from_inner(theta);
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        assert_ge!(theta, Finite::<T>::zero());
        assert_lt!(theta, two_pi);
        Angle(theta)
    }

    pub fn direction(self, other: Angle<T>) -> Direction {
        // Direction of shortest rotation from this angle to another.
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        match ((self.0 - other.0) % two_pi).cmp(&Finite::<T>::PI) {
            | Ordering::Equal => Direction::None,
            | Ordering::Greater => Direction::Counterclockwise,
            | Ordering::Less => Direction::Clockwise,
        }
    }

    // Returns true if self is between start and stop by the shortest path.
    pub fn between(self, start: Angle<T>, stop: Angle<T>) -> bool {
        start.direction(self) == start.direction(stop)
    }

    // Returns an angle representing the angle from other to self, counter-clockwise,
    // between -2PI and 2PI (-360deg and 360deg).
    fn ccw(self, other: Self) -> AngleDiff<T> {
        AngleDiff(other.0 - self.0)
    }

    // Returns an angle representing the angle from other to self, clockwise,
    // between -2PI and 2PI (-360deg and 360deg).
    fn cw(self, other: Self) -> AngleDiff<T> {
        AngleDiff(self.0 - other.0)
    }
}

impl<T: Value> From<AngleDiff<T>> for Angle<T> {
    fn from(diff: AngleDiff<T>) -> Self {
        let two_pi = Finite::<T>::from_inner(T::from_f64(2.0 * f64::PI).unwrap());
        // Add two_pi first because modulus doesn't work as expected for negative
        // numbers.
        Angle((diff.0 + two_pi) % two_pi)
    }
}

impl<T: Value> From<Angle<T>> for AngleDiff<T> {
    fn from(angle: Angle<T>) -> Self {
        AngleDiff(angle.0)
    }
}
