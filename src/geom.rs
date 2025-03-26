use std::cmp::Ordering;

use nannou::geom::{IVec2, Vec2};

pub type Segment = (Vec2, Vec2);

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Collinear,
    Clockwise,
    Counterclockwise,
}

fn on_segment(p: IVec2, q: IVec2, r: IVec2) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

fn orientation(p: IVec2, q: IVec2, r: IVec2) -> Orientation {
    let o = (q.y - p.y) as i64 * (r.x - q.x) as i64 - (q.x - p.x) as i64 * (r.y - q.y) as i64;

    match o.cmp(&0) {
        Ordering::Greater => Orientation::Clockwise,
        Ordering::Less => Orientation::Counterclockwise,
        Ordering::Equal => Orientation::Collinear,
    }
}

/// ref: https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
fn point_intersects(p1: IVec2, q1: IVec2, p2: IVec2, q2: IVec2) -> bool {
    // Special case: we don't care about endpoint intersections, unless it's a total overlap
    if (p1 == p2) || (q1 == q2) || (p1 == q2) || (q1 == p2) {
        return (p1 == p2 || p1 == q2) && (q1 == p2 || q1 == q2);
    }

    let (o1, o2, o3, o4) = (
        orientation(p1, q1, p2),
        orientation(p1, q1, q2),
        orientation(p2, q2, p1),
        orientation(p2, q2, q1),
    );

    (o1 != o2 && o3 != o4)
        || (o1 == Orientation::Collinear && on_segment(p1, p2, q1))
        || (o2 == Orientation::Collinear && on_segment(p1, q2, q1))
        || (o3 == Orientation::Collinear && on_segment(p2, p1, q2))
        || (o4 == Orientation::Collinear && on_segment(p2, q1, q2))
}

// We really want approximate intersections, FPA begone
fn ivec(v: Vec2) -> IVec2 {
    IVec2::new(
        (10000.0 * v.x).round() as i32,
        (10000.0 * v.y).round() as i32,
    )
}

/// Test if two line segments intersect
pub fn intersects(a: &Segment, b: &Segment) -> bool {
    point_intersects(ivec(a.0), ivec(a.1), ivec(b.0), ivec(b.1))
}
