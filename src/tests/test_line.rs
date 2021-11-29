use std::f64::consts::PI;

use decorum::Finite;

use crate::geometry::line::{Line, LineIntersection};
use crate::geometry::*;

#[test]
fn line_definition() {
    let start_point: Point<f64> = Point::new(2.0, 4.0);
    let end_point: Point<f64> = Point::new(4.0, -2.0);
    let line = Line::new(start_point, end_point).unwrap();

    let angle = (-3.0_f64.atan2(1.0)).rem_euclid(2.0 * PI);
    assert_eq!(line.angle, Angle::new(angle));
    assert_abs_diff_eq!(
        line.point_nearest_origin(),
        Point::new(3.0, 1.0),
        epsilon = 1e-10
    );
    assert_abs_diff_eq!(line.distance_from_origin.into_inner(), 10.0_f64.sqrt(), epsilon = 1e-10);
}

#[test]
fn line_intersection() {
    let start_point: Point<f64> = Point::new(2.0, 4.0);
    let end_point: Point<f64> = Point::new(4.0, 0.0);
    let line1 = Line::new(start_point, end_point).unwrap();

    let start_point: Point<f64> = Point::new(1.0, 1.0);
    let end_point: Point<f64> = Point::new(2.0, 0.0);
    let line2 = Line::new(start_point, end_point).unwrap();

    let target: Point<f64> = Point::new(6.0, -4.0);

    match line1.intersect(&line2) {
        LineIntersection::OutOfBounds(point) => {
            assert_abs_diff_eq!(
                point,
                target,
                epsilon = 1e-10
            )
        },
        _ => unreachable!()
    }
}
