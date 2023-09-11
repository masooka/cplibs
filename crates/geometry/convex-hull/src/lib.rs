#![allow(clippy::type_complexity)]

use std::ops::{Mul, Sub};

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
}
