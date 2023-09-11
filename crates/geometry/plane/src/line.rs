use std::{
    cmp::Ordering,
    collections::BTreeSet,
    mem,
    ops::{Mul, Sub},
};

use crate::{maxf64, minf64};

type Point<C> = (C, C);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Segment<C>(pub Point<C>, pub Point<C>);

impl Eq for Segment<f64> {}

impl Ord for Segment<f64> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0 .0.partial_cmp(&other.0 .0).unwrap()
    }
}

impl Segment<f64> {
    fn partial_cmp_at_start(&self, other: &Self) -> Option<Ordering> {
        let x = maxf64(minf64(self.0 .0, self.1 .0), minf64(other.0 .0, other.1 .0));
        self.y(x).partial_cmp(&other.y(x))
    }

    fn y(&self, x: f64) -> f64 {
        if (self.0 .0 - self.1 .0).abs() < f64::EPSILON {
            self.0 .1
        } else {
            self.0 .1 + (x - self.0 .0) * (self.1 .1 - self.0 .1) / (self.1 .0 - self.0 .0)
        }
    }
}

/// Checks if two line segments have any point in common.
///
/// # Examples
///
/// ```
/// # use plane::line::{Segment, do_intersect};
/// assert!(do_intersect(Segment((0, 0), (1, 1)), Segment((0, 1), (1, 0))));
pub fn do_intersect<C>(line1: Segment<C>, line2: Segment<C>) -> bool
where
    C: Copy + Default + PartialOrd + Sub<Output = C> + Mul<Output = C>,
{
    let zero = C::default();
    let Segment(p1, q1) = line1;
    let Segment(p2, q2) = line2;

    let o1 = cross_product(p1, q1, p2);
    let o2 = cross_product(p1, q1, q2);
    let o3 = cross_product(p2, q2, p1);
    let o4 = cross_product(p2, q2, q1);

    (o1 > zero && o2 < zero || o1 < zero && o2 > zero)
        && (o3 > zero && o4 < zero || o3 < zero && o4 > zero)
        || o1 == zero && is_in_rectangle(p2, line1)
        || o2 == zero && is_in_rectangle(q2, line1)
        || o3 == zero && is_in_rectangle(p1, line2)
        || o4 == zero && is_in_rectangle(q1, line2)
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
    let Segment(c1, c2) = diagonal;

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

#[derive(Clone, Copy, Debug)]
struct Event {
    x: f64,
    is_start: bool,
    id: usize,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.x - other.x).abs() > f64::EPSILON {
            self.x.partial_cmp(&other.x)
        } else {
            other.is_start.partial_cmp(&self.is_start)
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.is_start == other.is_start
    }
}

impl Eq for Event {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ActiveSegment {
    segment: Segment<f64>,
    id: usize,
}

impl PartialOrd for ActiveSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.segment.partial_cmp_at_start(&other.segment)
    }
}

impl Ord for ActiveSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn find_intersecting_segments(segments: &[Segment<f64>]) -> Option<(usize, usize)> {
    let mut events: Vec<Event> = Vec::new();
    for (i, &segment) in segments.iter().enumerate() {
        let Segment(mut p, mut q) = segment;
        if p.0 > q.0 {
            mem::swap(&mut p, &mut q);
        }
        events.push(Event {
            x: p.0,
            is_start: true,
            id: i,
        });
        events.push(Event {
            x: q.0,
            is_start: false,
            id: i,
        });
    }
    events.sort_unstable();

    let mut active_segments = BTreeSet::<ActiveSegment>::new();
    for event in events {
        let segment = ActiveSegment {
            segment: segments[event.id],
            id: event.id,
        };
        if event.is_start {
            if let Some(&next) = active_segments.range(segment..).next() {
                if do_intersect(segments[event.id], next.segment) {
                    return Some((event.id, next.id));
                }
            }
            if let Some(prev) = active_segments.range(..segment).next_back() {
                if do_intersect(segments[event.id], prev.segment) {
                    return Some((event.id, prev.id));
                }
            }
            active_segments.insert(segment);
        } else {
            let mut iter = active_segments.range(segment..);
            iter.next().unwrap();
            if let (Some(next), Some(prev)) =
                (iter.next(), active_segments.range(..segment).next_back())
            {
                if do_intersect(next.segment, prev.segment) {
                    return Some((next.id, prev.id));
                }
            }
            active_segments.remove(&segment);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::Segment;

    #[test]
    fn do_intersect() {
        // Overlapping
        assert!(super::do_intersect(
            Segment((1, 1), (5, 5)),
            Segment((2, 2), (6, 6))
        ));

        // Intersecting
        assert!(super::do_intersect(
            Segment((1, 1), (5, 5)),
            Segment((3, 1), (1, 3))
        ));

        // Sharing an endpoint
        assert!(super::do_intersect(
            Segment((1, 1), (5, 5)),
            Segment((5, 5), (5, 8))
        ));

        // Parallel
        assert!(!super::do_intersect(
            Segment((1, 1), (5, 5)),
            Segment((2, 3), (6, 7))
        ));

        // One includes the other
        assert!(super::do_intersect(
            Segment((1, 1), (5, 5)),
            Segment((2, 2), (3, 3))
        ));

        // collinear with no overlap
        assert!(!super::do_intersect(
            Segment((1, 1), (5, 5)),
            Segment((6, 6), (10, 10))
        ));

        // collinear sharing an endpoint
        assert!(super::do_intersect(
            Segment((1, 1), (5, 5)),
            Segment((5, 5), (10, 10))
        ));
    }

    #[test]
    fn find_intersecting_segments() {
        let segments = vec![
            Segment((0.0, 0.0), (1.0, 1.0)),
            Segment((0.0, 1.0), (1.0, 0.0)),
        ];
        let (segment1, segment2) = super::find_intersecting_segments(&segments).unwrap();
        assert!(segment1 == 0 && segment2 == 1 || segment1 == 1 && segment2 == 0);

        let segments = vec![
            Segment((0.0, 0.0), (1.0, 1.0)),
            Segment((0.0, 1.0), (1.0, 0.0)),
            Segment((0.1, 0.0), (0.9, 0.0)),
            Segment((0.1, 1.0), (0.9, 1.0)),
            Segment((0.0, 0.1), (0.0, 0.9)),
            Segment((1.0, 0.1), (1.0, 0.9)),
        ];
        let (segment1, segment2) = super::find_intersecting_segments(&segments).unwrap();
        assert!(segment1 == 0 && segment2 == 1 || segment1 == 1 && segment2 == 0);

        let segments = vec![
            Segment((0.1, 0.0), (0.9, 0.0)),
            Segment((0.1, 1.0), (0.9, 1.0)),
            Segment((0.0, 0.1), (0.0, 0.9)),
            Segment((1.0, 0.1), (1.0, 0.9)),
            Segment((0.0, 0.0), (1.0, 1.0)),
            Segment((1.0, 1.0), (2.0, 2.0)),
        ];
        let (segment1, segment2) = super::find_intersecting_segments(&segments).unwrap();
        assert!(segment1 == 4 && segment2 == 5 || segment1 == 5 && segment2 == 4);
    }

    #[test]
    fn debug() {
        let segments = vec![
            Segment((2.0, 501.0), (6.0, 501.0)),
            Segment((5.0, 1000.0), (995.0, 1.0)),
            Segment((1.0, 1.0), (1000.0, 1000.0)),
            Segment((994.0, 500.0), (1001.0, 500.0)),
        ];
        let result = super::find_intersecting_segments(&segments);
        assert!(result.is_some());
    }
}
