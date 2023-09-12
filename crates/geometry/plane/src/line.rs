use std::{
    cmp::Ordering,
    collections::BTreeSet,
    mem,
    ops::{Mul, Sub},
};

use crate::{cmpf64, maxf64, minf64};

type Point<C> = (C, C);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Segment<C>(pub Point<C>, pub Point<C>);

impl Eq for Segment<f64> {}

impl Segment<f64> {
    fn cmp_at_start(&self, other: &Self) -> Ordering {
        let x = maxf64(minf64(self.0 .0, self.1 .0), minf64(other.0 .0, other.1 .0));
        cmpf64(self.y(x), other.y(x))
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntersectionType {
    None,           // Segments don't meet
    Proper,         // Segments meet at one point other than endpoints
    Collinear,      // Segments overlap and share more than one point
    OneSided,       // One endpoint lies on the middle of the other segment
    MutualEndpoint, // Segments meet at a mutual endpoint
}

/// Identifies the relationship between two line segments.
#[must_use]
pub fn relationship_between_segments<C>(seg1: Segment<C>, seg2: Segment<C>) -> IntersectionType
where
    C: Copy + Default + PartialEq + PartialOrd + Sub<Output = C> + Mul<Output = C>,
{
    let zero = C::default();
    let (mut p1, mut q1) = (seg1.0, seg1.1);
    let (mut p2, mut q2) = (seg2.0, seg2.1);

    let o1 = cross_product(p1, q1, p2);
    let o2 = cross_product(p1, q1, q2);
    let o3 = cross_product(p2, q2, p1);
    let o4 = cross_product(p2, q2, q1);

    if o1 != zero && o2 != zero && o3 != zero && o4 != zero {
        if (o1 > zero && o2 > zero)
            || (o1 < zero && o2 < zero)
            || (o3 > zero && o4 > zero)
            || (o3 < zero && o4 < zero)
        {
            IntersectionType::None
        } else {
            IntersectionType::Proper
        }
    } else if o1 == zero && o2 == zero {
        if p1 > q1 {
            mem::swap(&mut p1, &mut q1);
        }
        if p2 > q2 {
            mem::swap(&mut p2, &mut q2);
        }
        if p1 > p2 {
            mem::swap(&mut p1, &mut p2);
            mem::swap(&mut q1, &mut q2);
        }
        if p2 < q1 {
            IntersectionType::Collinear
        } else if p2 == q1 {
            IntersectionType::MutualEndpoint
        } else {
            IntersectionType::None
        }
    } else {
        if o1 == zero || o2 == zero {
            if o2 == zero {
                mem::swap(&mut p2, &mut q2);
            }
        } else {
            mem::swap(&mut p1, &mut p2);
            mem::swap(&mut q1, &mut q2);
            if o4 == zero {
                mem::swap(&mut p2, &mut q2);
            }
        }
        if is_in_rectangle(p2, Segment(p1, q1)) {
            if p2 == p1 || p2 == q1 {
                IntersectionType::MutualEndpoint
            } else {
                IntersectionType::OneSided
            }
        } else {
            IntersectionType::None
        }
    }
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
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.x - other.x).abs() > f64::EPSILON {
            cmpf64(self.x, other.x)
        } else {
            other.is_start.cmp(&self.is_start)
        }
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
        Some(self.cmp(other))
    }
}

impl Ord for ActiveSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.segment.cmp_at_start(&other.segment) {
            Ordering::Equal => self.id.cmp(&other.id),
            other => other,
        }
    }
}

/// Finds the first pair of segments that have any point in common.
pub fn find_intersecting_segments(segments: &[Segment<f64>]) -> Option<(usize, usize)> {
    find_intersecting_segments_by(segments, do_intersect)
}

/// Finds the first pair of segments that share a point according to the given
/// predicate.
pub fn find_intersecting_segments_by(
    segments: &[Segment<f64>],
    do_intersect: fn(Segment<f64>, Segment<f64>) -> bool,
) -> Option<(usize, usize)> {
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
    fn three_lines() {
        let segments = vec![
            Segment((2.0, 5.0), (3.1, 5.0)),
            Segment((3.0, 10.0), (9.0, 1.0)),
            Segment((1.0, 1.0), (10.0, 10.0)),
            Segment((8.0, 5.0), (10.1, 5.0)),
        ];
        let result = super::find_intersecting_segments(&segments);
        assert!(result.is_some());

        let segments = vec![
            Segment((1.0, 4.0), (9.0, 0.0)),
            Segment((0.0, 2.0), (10.0, 2.0)),
            Segment((1.0, 0.0), (9.0, 4.0)),
        ];
        let result = super::find_intersecting_segments(&segments);
        assert!(result.is_some());
    }

    #[test]
    fn relationship_between_segments() {
        // Segments not parallel
        let seg1 = Segment((1, 1), (5, 5));
        let seg2 = Segment((0, 2), (0, 4));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::None
        );

        // Segments on the same line not overlapping
        let seg1 = Segment((1, 1), (5, 5));
        let seg2 = Segment((6, 6), (10, 10));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::None
        );

        // Segments on the same line meet at exactly one endpoint
        let seg1 = Segment((1, 1), (5, 5));
        let seg2 = Segment((5, 5), (6, 6));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::MutualEndpoint
        );

        // Segments meet at one point other than endpoints
        let seg1 = Segment((1, 1), (5, 5));
        let seg2 = Segment((2, 5), (4, 1));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::Proper
        );

        // The first segment is contained in the second
        let seg1 = Segment((1, 1), (2, 2));
        let seg2 = Segment((0, 0), (3, 3));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::Collinear
        );

        // The second segment is contained in the first
        let seg1 = Segment((1, 1), (5, 5));
        let seg2 = Segment((2, 2), (4, 4));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::Collinear
        );

        // Collinear segments overlapping
        let seg1 = Segment((1, 1), (5, 5));
        let seg2 = Segment((2, 2), (6, 6));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::Collinear
        );

        // Overlapping segments sharing an endpoint
        let seg1 = Segment((0, 0), (1, 1));
        let seg2 = Segment((0, 0), (2, 2));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::Collinear
        );

        // Sharing one endpoint and not parallel
        let seg1 = Segment((0, 0), (1, 1));
        let seg2 = Segment((1, 1), (0, 1));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::MutualEndpoint
        );

        // First one's endpoint is on the same line as the secondd
        let seg1 = Segment((0, 0), (4, 0));
        let seg2 = Segment((4, 5), (4, 1));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::None
        );

        // p1 and q2 are the same point
        let seg1 = Segment((0, 0), (0, 1));
        let seg2 = Segment((1, 0), (0, 0));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::MutualEndpoint
        );

        // q2 is on the same line as p1 q1
        let seg1 = Segment((0, 1), (0, 0));
        let seg2 = Segment((-1, 0), (2, 0));
        assert_eq!(
            super::relationship_between_segments(seg1, seg2),
            super::IntersectionType::OneSided
        );
    }

    #[test]
    fn find_intersecting_segments_by() {
        let segments = vec![
            Segment((0.0, 0.0), (1.0, 1.0)),
            Segment((1.0, 1.0), (2.0, 2.0)),
        ];
        let result = super::find_intersecting_segments_by(&segments, |a, b| {
            matches!(
                super::relationship_between_segments(a, b),
                super::IntersectionType::Proper
                    | super::IntersectionType::Collinear
                    | super::IntersectionType::OneSided
            )
        });
        assert!(result.is_none());

        let segments = vec![
            Segment((0.0, 0.0), (1.0, 1.0)),
            Segment((0.0, 0.0), (1.0, 0.0)),
        ];
        let result = super::find_intersecting_segments_by(&segments, |a, b| {
            matches!(
                super::relationship_between_segments(a, b),
                super::IntersectionType::Proper
                    | super::IntersectionType::Collinear
                    | super::IntersectionType::OneSided
            )
        });
        assert!(result.is_none());
    }
}
