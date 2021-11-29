use std::f64::consts::PI;


use crate::geometry::arc::Arc;
use crate::geometry::*;
use crate::geometry::line::Line;
use crate::to_svg::{to_document, CoordinateTransform, ToSvg};

#[test]
fn line_to_svg() {
    let start_point: Point<f64> = Point::new(1.0, 1.0);
    let end_point: Point<f64> = Point::new(5.0, 3.0);
    let line = Line::new(start_point, end_point).unwrap();
    let node = line.to_svg(None);
    let output_path = "test_line.svg";
    let transform = CoordinateTransform {
        upper_left: Point::<f64>::new(10.0, 10.0),
        scale: Delta::<f64>::new(1.0, 1.0),
        rotation: Angle::<f64>::new(0.0),
    };
    let document = to_document(node, transform);
    svg::save(&output_path, &document)
        .expect(&format!("Unable to write to file {}", &output_path));
}

#[test]
fn arc_to_svg() {
    let start_point: Point<f64> = Point::new(1.0, 1.0);
    let stop_point: Point<f64> = Point::new(5.0, 3.0);
    let angle: Angle<f64> = Angle::new(PI / 4.0);
    let arc = Arc::new(start_point, stop_point, angle).unwrap();

    let node = arc.to_svg(None);
    let output_path = "test_arc.svg";
    let transform = CoordinateTransform {
        upper_left: Point::<f64>::new(10.0, 10.0),
        scale: Delta::<f64>::new(1.0, 1.0),
        rotation: Angle::<f64>::new(0.0),
    };
    let document = to_document(node, transform);
    svg::save(&output_path, &document)
        .expect(&format!("Unable to write to file {}", &output_path));
}
