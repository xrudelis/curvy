use std::fmt;

use decorum::{Finite, Real};
use derive_more::{Add, Div, Mul, Neg, Sub};

use crate::geometry::*;

#[derive(Add, Clone, Copy, Debug, Div, Eq, Mul, Neg, PartialEq, Sub)]
pub struct Delta<T: Value> {
    pub dx: Finite<T>,
    pub dy: Finite<T>,
}

impl<T: Value> Delta<T> {
    // Delta::new() will panic if dx, dy are not finite.
    pub fn new(dx: T, dy: T) -> Self {
        let dx = Finite::<T>::from_inner(dx);
        let dy = Finite::<T>::from_inner(dy);
        Delta { dx, dy }
    }

    pub fn magnitude_angle(magnitude: Finite<T>, angle: Angle<T>) -> Self {
        Delta {
            dx: magnitude * angle.0.cos(),
            dy: magnitude * angle.0.sin(),
        }
    }

    pub fn angle(self) -> Angle<T> {
        Angle(self.dy.atan2(self.dx))
    }

    pub fn magnitude(self) -> Finite<T> {
        return (self.dx * self.dx + self.dy * self.dy).sqrt();
    }

    // If this Delta represents a point on a circle drawn from its center, how far
    // along the circle from (1, 0) the point is.
    pub fn arc_length(self) -> Finite<T> {
        self.magnitude() * self.angle().0
    }

    pub fn rotate(self, angle: Angle<T>) -> Self {
        let sin = angle.radians().sin();
        let cos = angle.radians().cos();
        Delta {
            dx: self.dx * cos - self.dy * sin,
            dy: self.dx * sin + self.dy * cos,
        }
    }
}

impl<T: Value> fmt::Display for Delta<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.dx, self.dy)
    }
}
