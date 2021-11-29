use std::fmt::{Debug, Display};
use std::ops::Rem;

use approx::RelativeEq;
use decorum::{Float, Primitive};
use num_traits::cast::FromPrimitive;

pub trait Value:
    Float + Primitive + Debug + Display + FromPrimitive + RelativeEq + Rem
{
}

// Value is blanket-implemented for types like f32 and f64.
impl<T> Value for T where
    T: Float + Primitive + Debug + Display + FromPrimitive + RelativeEq + Rem
{
}
