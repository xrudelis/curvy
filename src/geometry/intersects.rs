pub trait Intersects<Rhs> {
    type Intersection;
    fn intersect(self, other: &Rhs) -> Self::Intersection;
}
