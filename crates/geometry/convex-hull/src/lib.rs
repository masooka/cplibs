#![allow(clippy::type_complexity)]

use std::{
    cmp::Ordering,
    mem,
    ops::{Mul, Sub},
};

/// Finds the convex hull of a set of points.
///
/// This function takes a vector of points, a key function for sorting the
/// points, and a turn direction function. It returns a vector containing the
/// points that make up the convex hull in sorted order.
///
/// # Arguments
///
/// * `points` - A vector of points represented as tuples of i64 coordinates.
/// * `key` - A function that takes a point and returns a key for sorting the
///   points.
/// * `turn_direction` - A function that takes three points and returns a
///   boolean indicating the turn direction. This could be one of the turn
///   direction functions defined in this module, such as `clockwise`,
///   `clockwise_or_collinear`, `counterclockwise`, or
///   `counterclockwise_or_collinear`.
///
/// # Examples
///
/// ```
/// # use convex_hull::{convex_hull, counterclockwise};
/// let mut points = vec![(0, 0), (1, 1), (2, 2), (3, 2), (4, 0)];
/// let hull = convex_hull(points, |&(x, y)| (x, y), counterclockwise);
/// assert_eq!(hull, vec![(0, 0), (4, 0), (3, 2), (2, 2)]);
/// ```
pub fn convex_hull<C, K: PartialOrd>(
    mut points: Vec<(C, C)>,
    key: fn(&(C, C)) -> K,
    turn_direction: fn(&(C, C), &(C, C), &(C, C)) -> bool,
) -> Vec<(C, C)>
where
    C: Copy + 'static,
{
    points.sort_unstable_by(|a, b| key(a).partial_cmp(&key(b)).unwrap());

    let mut hull = half_hull(&points, turn_direction);
    hull.pop();
    hull.extend(half_hull(points.iter().rev(), turn_direction));
    hull.pop();

    hull
}

/// Finds the convex hull of a set of points. It returns the points in sorted
/// order along with the indices of the points that make up the hull.
pub fn convex_hull_counterclockwise<C>(
    mut points: Vec<(C, C)>,
    hull_include_midpoints: bool,
) -> (Vec<(C, C)>, Vec<usize>)
where
    C: Copy + Default + PartialOrd + Sub<Output = C> + Mul<Output = C>,
{
    points.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let turn_direction = if hull_include_midpoints {
        counterclockwise_or_collinear
    } else {
        counterclockwise
    };
    let hull = convex_hull_sorted(&points, turn_direction);
    (points, hull)
}

fn convex_hull_sorted<C: PartialOrd>(
    sorted_points: &[(C, C)],
    turn_direction: fn(&(C, C), &(C, C), &(C, C)) -> bool,
) -> Vec<usize> {
    let mut hull = Vec::new();

    let mut half: Vec<usize> = Vec::new();
    convex_hull_sorted_half(
        sorted_points,
        0..sorted_points.len(),
        &mut half,
        turn_direction,
    );

    half.pop();
    mem::swap(&mut hull, &mut half);
    convex_hull_sorted_half(
        sorted_points,
        (0..sorted_points.len()).rev(),
        &mut half,
        turn_direction,
    );
    half.pop();
    hull.extend(half);

    hull
}

pub fn convex_hull_sorted_half_counterclockwise<C>(
    sorted_points: &[(C, C)],
    iter: impl Iterator<Item = usize>,
    hull_include_midpoints: bool,
) -> Vec<usize>
where
    C: Copy + Default + PartialOrd + Sub<Output = C> + Mul<Output = C>,
{
    let turn_direction = if hull_include_midpoints {
        counterclockwise_or_collinear
    } else {
        counterclockwise
    };
    let mut half = Vec::new();
    convex_hull_sorted_half(sorted_points, iter, &mut half, turn_direction);
    half
}

fn convex_hull_sorted_half<C>(
    points: &[(C, C)],
    iter: impl Iterator<Item = usize>,
    half: &mut Vec<usize>,
    turn_direction: fn(&(C, C), &(C, C), &(C, C)) -> bool,
) {
    for i in iter {
        while half.len() >= 2 {
            let o = &points[*half.get(half.len() - 2).unwrap()];
            let a = &points[*half.get(half.len() - 1).unwrap()];
            if turn_direction(o, a, &points[i]) {
                break;
            }
            half.pop();
        }
        half.push(i);
    }
}

/// Determines if three points make a clockwise turn.
pub fn clockwise<C>(o: &(C, C), a: &(C, C), b: &(C, C)) -> bool
where
    C: Copy + Default + Sub<Output = C> + Mul<Output = C> + PartialOrd,
{
    cross_product(o, a, b) < C::default()
}

/// Determines if three points make a clockwise turn or are collinear.
pub fn clockwise_or_collinear<C>(o: &(C, C), a: &(C, C), b: &(C, C)) -> bool
where
    C: Copy + Default + Sub<Output = C> + Mul<Output = C> + PartialOrd,
{
    cross_product(o, a, b) <= C::default()
}

/// Determines if three points make a counterclockwise turn.
pub fn counterclockwise<C>(o: &(C, C), a: &(C, C), b: &(C, C)) -> bool
where
    C: Copy + Default + Sub<Output = C> + Mul<Output = C> + PartialOrd,
{
    cross_product(o, a, b) > C::default()
}

/// Determines if three points make a counterclockwise turn or are collinear.
pub fn counterclockwise_or_collinear<C>(o: &(C, C), a: &(C, C), b: &(C, C)) -> bool
where
    C: Copy + Default + Sub<Output = C> + Mul<Output = C> + PartialOrd,
{
    cross_product(o, a, b) >= C::default()
}

fn cross_product<C>(o: &(C, C), a: &(C, C), b: &(C, C)) -> C
where
    C: Copy + Sub<Output = C> + Mul<Output = C>,
{
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

/// Computes the half hull of a set of points, returning the points in sorted
/// order.
///
/// The `points` vector must be sorted prior to calling this function. For the
/// upper hull, sort by increasing x (and y if tied) and use `clockwise` as
/// `turn_direction`. For the lower hull, use `counterclockwise`.
///
/// # Examples
///
/// ```
/// # use convex_hull::{half_hull, counterclockwise};
/// let mut points = vec![(0, 0), (1, 1), (2, 2), (3, 2), (4, 0)];
/// points.sort_unstable_by_key(|&(x, y)| (x, y));
/// let lower_hull = half_hull(&points, counterclockwise);
/// assert_eq!(lower_hull, vec![(0, 0), (4, 0)]);
/// ```
pub fn half_hull<'a, C, I>(
    points: I,
    turn_direction: fn(&(C, C), &(C, C), &(C, C)) -> bool,
) -> Vec<(C, C)>
where
    C: Copy + 'static,
    I: IntoIterator<Item = &'a (C, C)>,
{
    let mut hull: Vec<(C, C)> = Vec::new();

    for &b in points {
        while hull.len() >= 2 {
            let a = *hull.last().unwrap();
            let o = *hull.get(hull.len() - 2).unwrap();
            if turn_direction(&o, &a, &b) {
                break;
            }
            hull.pop();
        }
        hull.push(b);
    }

    hull
}

pub fn antipodal_pairs(points: &Vec<(i64, i64)>) -> Vec<(usize, usize)> {
    let n = points.len();
    match n.cmp(&2) {
        Ordering::Less => return vec![],
        Ordering::Equal => return vec![(0, 1)],
        _ => {}
    }

    let mut pairs = Vec::new();
    let mut j = 1;

    for i in 0..n {
        let next_i = (i + 1) % n;

        while {
            let curr_area = cross_product(&points[i], &points[next_i], &points[j]);
            let next_j = (j + 1) % n;
            let next_area = cross_product(&points[i], &points[next_i], &points[next_j]);
            next_area > curr_area
        } {
            j = (j + 1) % n;
        }

        pairs.push((i, j));
    }

    pairs
}

#[cfg(test)]
mod tests {
    #[test]
    fn convex_hull() {
        let points = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 1),
            (4, 0),
            (3, -1),
            (2, -2),
            (1, -1),
        ];

        let hull = super::convex_hull(points, |&(x, y)| (x, y), super::counterclockwise);
        assert_eq!(hull, &[(0, 0), (2, -2), (4, 0), (2, 2)]);
    }

    #[test]
    fn partition() {
        let points = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 1),
            (4, 0),
            (3, -1),
            (2, -2),
            (1, -1),
        ];

        let (_, hull) = super::convex_hull_counterclockwise(points, false);
        assert_eq!(hull, &[0, 3, 7, 4]);
    }

    #[test]
    fn partition_last_pop() {
        let points = vec![
            (-5, 4),
            (-4, 2),
            (-3, 2),
            (-2, -5),
            (-1, 3),
            (2, -5),
            (2, -2),
            (2, -1),
        ];
        let (_, hull) = super::convex_hull_counterclockwise(points, false);
        assert_eq!(hull, &[0, 3, 5, 7, 4]);
    }

    #[test]
    fn partition_middle_pop() {
        let points = vec![
            (-4, -5),
            (-4, -3),
            (-3, 4),
            (1, -5),
            (3, -2),
            (3, -1),
            (3, 4),
            (4, -5),
        ];
        let (_sorted, hull) = super::convex_hull_counterclockwise(points, false);
        assert_eq!(hull, &[0, 7, 6, 2, 1]);
    }
}
