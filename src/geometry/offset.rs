use decorum::Finite;

use crate::geometry::base::*;

// Positive offset means in the direction of -90deg, negative offset is in the direction of 90deg,
// relative to the direction of the line or curve. This means that counterclockwise polygons are
// outset when offset is positive, and inset when offset is negative.
pub trait Offset<T: Value> {
    type OffsetResult;
    fn offset(self, offset: Finite<T>) -> Self::OffsetResult;
}
