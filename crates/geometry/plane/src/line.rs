use std::{
    cmp::Ordering,
    ops::{Mul, Sub},
};

type Point<C> = (C, C);
type Segment<C> = (Point<C>, Point<C>);

/// Checks if two line segments have any point in common.
///
/// # Examples
///
/// ```
/// # use plane::line::do_intersect;
/// assert!(do_intersect(((0, 0), (1, 1)), ((0, 1), (1, 0))));
pub fn do_intersect<C>(line1: Segment<C>, line2: Segment<C>) -> bool
where
    C: Copy + Default + PartialOrd + Sub<Output = C> + Mul<Output = C>,
{
    let zero = C::default();
    let (p1, q1) = line1;
    let (p2, q2) = line2;

    let o1 = cross_product(p1, q1, p2);
    let o2 = cross_product(p1, q1, q2);
    let o3 = cross_product(p2, q2, p1);
    let o4 = cross_product(p2, q2, q1);

    (o1 > zero && o2 < zero || o1 < zero && o2 > zero)
        && (o3 > zero && o4 < zero || o3 < zero && o4 > zero)
        || o1 == zero && is_in_rectangle(p2, (p1, q1))
        || o2 == zero && is_in_rectangle(q2, (p1, q1))
        || o3 == zero && is_in_rectangle(p1, (p2, q2))
        || o4 == zero && is_in_rectangle(q1, (p2, q2))
}

/// Returns a positive value if `o`, `a`, and `b` make a counter-clockwise turn,
/// a negative value if they make a clockwise turn, and zero if they are
/// collinear.
fn cross_product<C>(o: (C, C), a: (C, C), b: (C, C)) -> C
where
    C: Copy + Sub<Output = C> + Mul<Output = C>,
{
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

/// Checks if a point `r` is on or in the rectangle parallel to the axes
/// defined by the diagonal line segment `diagonal`.
fn is_in_rectangle<C: Copy + PartialOrd>(p: Point<C>, diagonal: Segment<C>) -> bool {
    let (c1, c2) = diagonal;

    let x_min = match c1.0.partial_cmp(&c2.0) {
        Some(Ordering::Less) | Some(Ordering::Equal) => c1.0,
        Some(Ordering::Greater) => c2.0,
        None => return false,
    };

    let x_max = match c1.0.partial_cmp(&c2.0) {
        Some(Ordering::Greater) | Some(Ordering::Equal) => c1.0,
        Some(Ordering::Less) => c2.0,
        None => return false,
    };

    let y_min = match c1.1.partial_cmp(&c2.1) {
        Some(Ordering::Less) | Some(Ordering::Equal) => c1.1,
        Some(Ordering::Greater) => c2.1,
        None => return false,
    };

    let y_max = match c1.1.partial_cmp(&c2.1) {
        Some(Ordering::Greater) | Some(Ordering::Equal) => c1.1,
        Some(Ordering::Less) => c2.1,
        None => return false,
    };

    matches!(
        (
            p.0.partial_cmp(&x_min),
            p.0.partial_cmp(&x_max),
            p.1.partial_cmp(&y_min),
            p.1.partial_cmp(&y_max),
        ),
        (
            Some(Ordering::Greater) | Some(Ordering::Equal),
            Some(Ordering::Less) | Some(Ordering::Equal),
            Some(Ordering::Greater) | Some(Ordering::Equal),
            Some(Ordering::Less) | Some(Ordering::Equal),
        )
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn do_intersect() {
        // Overlapping
        assert!(super::do_intersect(((1, 1), (5, 5)), ((2, 2), (6, 6))));

        // Intersecting
        assert!(super::do_intersect(((1, 1), (5, 5)), ((3, 1), (1, 3))));

        // Sharing an endpoint
        assert!(super::do_intersect(((1, 1), (5, 5)), ((5, 5), (5, 8))));

        // Parallel
        assert!(!super::do_intersect(((1, 1), (5, 5)), ((2, 3), (6, 7))));

        // One includes the other
        assert!(super::do_intersect(((1, 1), (5, 5)), ((2, 2), (3, 3))));

        // collinear with no overlap
        assert!(!super::do_intersect(((1, 1), (5, 5)), ((6, 6), (10, 10))));

        // collinear sharing an endpoint
        assert!(super::do_intersect(((1, 1), (5, 5)), ((5, 5), (10, 10))));
    }
}
