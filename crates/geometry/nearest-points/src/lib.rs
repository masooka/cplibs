use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

pub trait UpperBounded {
    fn max_value() -> Self;
}

impl UpperBounded for i64 {
    fn max_value() -> Self {
        i64::MAX
    }
}

impl UpperBounded for f64 {
    fn max_value() -> Self {
        f64::MAX
    }
}

/// Finds the minimum distance between two points in `points` by divide and
/// conquer.
///
/// # Panics
///
/// Panics if `T::partial_cmp` returns `None`.
pub fn min_distance2<T>(points: Vec<(T, T)>) -> T
where
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + UpperBounded,
{
    let mut points = points;
    points.sort_by(|p, q| match p.0.partial_cmp(&q.0).unwrap() {
        Ordering::Equal => p.1.partial_cmp(&q.1).unwrap(),
        ord => ord,
    });
    min_distance2_inner(&mut points)
}

fn min_distance2_inner<T>(points: &mut [(T, T)]) -> T
where
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + UpperBounded,
{
    if points.len() <= 3 {
        let mut dist_min = T::max_value();
        for (i, &p) in points.iter().enumerate() {
            for &q in points.iter().skip(i + 1) {
                let dist = (p.0 - q.0) * (p.0 - q.0) + (p.1 - q.1) * (p.1 - q.1);
                if dist < dist_min {
                    dist_min = dist;
                }
            }
        }
        points.sort_by(|&p, &q| p.1.partial_cmp(&q.1).unwrap());
        return dist_min;
    }

    let mid = points.len() / 2;
    let x_mid = points[mid].0;
    let min_left = min_distance2_inner(&mut points[..mid]);
    let min_right = min_distance2_inner(&mut points[mid..]);
    let mut dist_min = if min_left < min_right {
        min_left
    } else {
        min_right
    };
    let mut tmp = merge_by_y(points[..mid].iter().copied(), points[mid..].iter().copied());
    points.copy_from_slice(&tmp);

    tmp.clear();
    for p in points {
        if (p.0 - x_mid) * (p.0 - x_mid) >= dist_min {
            continue;
        }
        for q in tmp.iter().rev() {
            if (p.1 - q.1) * (p.1 - q.1) >= dist_min {
                break;
            }
            let dist = (p.0 - q.0) * (p.0 - q.0) + (p.1 - q.1) * (p.1 - q.1);
            if dist < dist_min {
                dist_min = dist;
            }
        }
        tmp.push(*p);
    }
    dist_min
}

fn merge_by_y<T>(
    left: impl Iterator<Item = (T, T)>,
    right: impl Iterator<Item = (T, T)>,
) -> Vec<(T, T)>
where
    T: Copy + PartialOrd,
{
    let mut left = left.peekable();
    let mut right = right.peekable();
    let mut merged = Vec::new();
    while let (Some(&l), Some(&r)) = (left.peek(), right.peek()) {
        if l.1 < r.1 {
            merged.push(left.next().unwrap());
        } else {
            merged.push(right.next().unwrap());
        }
    }
    merged.extend(left);
    merged.extend(right);
    merged
}
