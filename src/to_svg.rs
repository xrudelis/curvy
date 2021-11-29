use svg::node::element::{Group, Path};
use svg::node::Node;
use svg::Document;

use crate::geometry::arc::Arc;
use crate::geometry::line::Line;
use crate::geometry::poly::{Polyarc, Polycurve, Polygon, Polyline};
use crate::geometry::{Angle, Delta, Point, Value};

#[derive(Clone, Copy, Debug)]
pub struct CoordinateTransform<T: Value> {
    pub upper_left: Point<T>,
    pub scale: Delta<T>,
    pub rotation: Angle<T>,
}

pub trait ToSvg<T: Value> {
    type ElementStyling;
    fn to_svg(self: &Self, style: Self::ElementStyling) -> Group;
}

pub struct LineStyling {/* todo */}
pub struct FillStyling {/* todo */}
//pub struct MarkerStyling {/* todo */}

impl<T: Value> ToSvg<T> for Line<T> {
    type ElementStyling = Option<LineStyling>;

    fn to_svg(self: &Self, style: Self::ElementStyling) -> Group {
        let d_string = format!("M{} L{}", self.start(), self.stop()).to_string();
        let mut path = Path::new().set("d", d_string).set("fill", "none");
        if cfg!(debug_assertions) {
            // debug color
            path.assign("stroke", "#FF00FF");
        } else {
            path.assign("display", "none");
        }
        let group = Group::new().add(path);
        return group;
    }
}

impl<T: Value> ToSvg<T> for Polyline<T> {
    type ElementStyling = Option<LineStyling>;

    fn to_svg(self: &Self, style: Self::ElementStyling) -> Group {
        let points = self.points();
        let n_points = points.len();
        let mut d_string = String::with_capacity(32 * n_points);
        let first_point = points[0];
        d_string.push_str(&format!("M{} ", first_point));
        for point in points {
            d_string.push_str(&format!("L{} ", point));
        }
        let mut path = Path::new().set("d", d_string).set("fill", "none");
        if cfg!(debug_assertions) {
            // debug color
            path.assign("stroke", "#FF00FF");
        } else {
            path.assign("display", "none");
        }
        let group = Group::new().add(path);
        return group;
    }
}

impl<T: Value> ToSvg<T> for Polygon<T> {
    // TODO: styling also has fill styling?
    type ElementStyling = (Option<LineStyling>, Option<FillStyling>);

    fn to_svg(self: &Self, style: Self::ElementStyling) -> Group {
        let points = self.points();
        let n_points = points.len();
        let mut d_string = String::with_capacity(32 * n_points);
        let first_point = points[0];
        d_string.push_str(&format!("M{} ", first_point));
        for point in points {
            d_string.push_str(&format!("L{} ", point));
        }
        d_string.push_str("Z");
        let mut path = Path::new().set("d", d_string).set("fill", "none");
        if cfg!(debug_assertions) {
            // debug color
            path.assign("stroke", "#FF00FF");
        } else {
            path.assign("display", "none");
        }
        let group = Group::new().add(path);
        return group;
    }
}

impl<T: Value> ToSvg<T> for Arc<T> {
    type ElementStyling = Option<LineStyling>;

    fn to_svg(self: &Self, style: Self::ElementStyling) -> Group {
        let large_arc_flag = false;
        let d_string = format!(
            "M{} A{},{} 0 {},{} {} ",
            self.start(),
            self.radius,
            self.radius,
            large_arc_flag as usize,
            self.sweep_flag() as usize,
            self.stop()
        );
        let mut path = Path::new().set("d", d_string).set("fill", "none");
        if cfg!(debug_assertions) {
            // debug color
            path.assign("stroke", "#FF00FF");
        } else {
            path.assign("display", "none");
        }
        let group = Group::new().add(path);
        return group;
    }
}

impl<T: Value> ToSvg<T> for Polyarc<T> {
    type ElementStyling = LineStyling;

    fn to_svg(self: &Self, style: Self::ElementStyling) -> Group {
        todo!()
    }
}

impl<T: Value> ToSvg<T> for Polycurve<T> {
    // TODO: styling also has fill styling?
    type ElementStyling = (Option<LineStyling>, Option<FillStyling>);

    fn to_svg(self: &Self, style: Self::ElementStyling) -> Group {
        todo!()
    }
}

pub fn to_document<T: Value>(
    group: Group,
    transform: CoordinateTransform<T>,
) -> Document {
    let viewbox = (0.0, 0.0, 10.0, 10.0);
    let document = Document::new().set("viewBox", viewbox).add(group);
    return document;
}
