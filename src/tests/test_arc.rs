use std::f64::consts::PI;

use decorum::Finite;

use crate::geometry::*;
use crate::geometry::arc::Arc;


#[test]
fn arc_new_clockwise() {
    let start_point: Point<f64> = Point::new(1.0, 1.0);
    let stop_point: Point<f64> = Point::new(5.0, 3.0);
    let angle: Angle<f64> = Angle::new(PI / 4.0);
    let arc = Arc::new(start_point, stop_point, angle).unwrap();
    assert_abs_diff_eq!(start_point.midpoint(stop_point), Point::new(3.0, 2.0), epsilon = 1e-10);
    assert_abs_diff_eq!(arc.center, Point::new(6.0, -4.0), epsilon = 1e-10);
    assert_abs_diff_eq!(arc.radius.into_inner(), 50.0_f64.sqrt(), epsilon = 1e-10);
    let start_angle = angle.radians().into_inner() + PI / 2.0;
    assert_abs_diff_eq!(arc.start_angle().radians().into_inner(), start_angle, epsilon = 1e-10);
    let stop_angle = 7.0_f64.atan2(-1.0);
    assert_abs_diff_eq!(arc.stop_angle().radians().into_inner(), stop_angle, epsilon = 1e-10);
    assert_abs_diff_eq!(arc.start(), start_point, epsilon = 1e-10);
    assert_abs_diff_eq!(arc.stop(), stop_point, epsilon = 1e-10);
}

#[test]
fn arc_length() {
    let start_point: Point<f64> = Point::new(1.0, 1.0);
    let end_point: Point<f64> = Point::new(-1.0, 1.0);
    let angle: Angle<f64> = Angle::new(3.0 * PI / 4.0);
    let arc = Arc::new(start_point, end_point, angle).unwrap();
    assert_abs_diff_eq!(arc.length().into_inner(), 2.0_f64.sqrt() * PI / 2.0, epsilon = 1e-10)
}

#[test]
fn arc_negative_offset_length() {
    // When radius is negative, for instance from too much an inset, then we expect
    // that the length will also be negative.
    let start_point: Point<f64> = Point::new(1.0, 1.0);
    let end_point: Point<f64> = Point::new(-1.0, 1.0);
    let angle: Angle<f64> = Angle::new(3.0 * PI / 4.0);
    let arc = Arc::new(start_point, end_point, angle).unwrap();
    assert_abs_diff_eq!(arc.radius.into_inner(), 2.0_f64.sqrt(), epsilon = 1e-10);
    assert_lt!(arc.begin(), arc.end());
    let arc = arc.offset(Finite::from_inner(-2.0 * 2.0_f64.sqrt()));
    assert_abs_diff_eq!(arc.radius.into_inner(), -2.0_f64.sqrt(), epsilon = 1e-10);
    assert_lt!(arc.end(), arc.begin());
    assert_abs_diff_eq!(arc.length().into_inner(), -2.0_f64.sqrt() * PI / 2.0, epsilon = 1e-10)
}
