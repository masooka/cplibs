#![allow(clippy::type_complexity)]

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
pub fn convex_hull<K: Ord>(
    mut points: Vec<(i64, i64)>,
    key: fn(&(i64, i64)) -> K,
    turn_direction: fn(&(i64, i64), &(i64, i64), &(i64, i64)) -> bool,
) -> Vec<(i64, i64)> {
    points.sort_unstable_by_key(key);

    let mut hull = half_hull(&points, turn_direction);
    hull.pop();
    hull.extend(half_hull(points.iter().rev(), turn_direction));
    hull.pop();

    hull
}

/// Determines if three points make a clockwise turn.
pub fn clockwise(o: &(i64, i64), a: &(i64, i64), b: &(i64, i64)) -> bool {
    cross_product(o, a, b) < 0
}

/// Determines if three points make a clockwise turn or are collinear.
pub fn clockwise_or_collinear(o: &(i64, i64), a: &(i64, i64), b: &(i64, i64)) -> bool {
    cross_product(o, a, b) <= 0
}

/// Determines if three points make a counterclockwise turn.
pub fn counterclockwise(o: &(i64, i64), a: &(i64, i64), b: &(i64, i64)) -> bool {
    cross_product(o, a, b) > 0
}

/// Determines if three points make a counterclockwise turn or are collinear.
pub fn counterclockwise_or_collinear(o: &(i64, i64), a: &(i64, i64), b: &(i64, i64)) -> bool {
    cross_product(o, a, b) >= 0
}

fn cross_product(o: &(i64, i64), a: &(i64, i64), b: &(i64, i64)) -> i64 {
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
pub fn half_hull<'a, I>(
    points: I,
    turn_direction: fn(&(i64, i64), &(i64, i64), &(i64, i64)) -> bool,
) -> Vec<(i64, i64)>
where
    I: IntoIterator<Item = &'a (i64, i64)>,
{
    let mut hull: Vec<(i64, i64)> = Vec::new();

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
