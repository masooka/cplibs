/// Finds the minimum distance between two points in `points` by divide and
/// conquer.
pub fn min_distance2(points: &mut [(i64, i64)]) -> i64 {
    if points.len() <= 3 {
        let mut dist_min = i64::MAX;
        for (i, &p) in points.iter().enumerate() {
            for &q in points.iter().skip(i + 1) {
                let dist = (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2);
                dist_min = dist_min.min(dist);
            }
        }
        points.sort_by_key(|&p| p.1);
        return dist_min;
    }

    let mid = points.len() / 2;
    let x_mid = points[mid].0;
    let min_left = min_distance2(&mut points[..mid]);
    let min_right = min_distance2(&mut points[mid..]);
    let mut dist_min = min_left.min(min_right);
    let mut tmp = merge_by_y(points[..mid].iter().copied(), points[mid..].iter().copied());
    points.copy_from_slice(&tmp);

    tmp.clear();
    for p in points {
        if (p.0 - x_mid).pow(2) >= dist_min {
            continue;
        }
        for q in tmp.iter().rev() {
            if (p.1 - q.1).pow(2) >= dist_min {
                break;
            }
            let dist = (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2);
            dist_min = dist_min.min(dist);
        }
        tmp.push(*p);
    }
    dist_min
}

fn merge_by_y(
    left: impl Iterator<Item = (i64, i64)>,
    right: impl Iterator<Item = (i64, i64)>,
) -> Vec<(i64, i64)> {
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
