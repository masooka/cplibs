use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Direction {}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x == 0 && self.y == 0 {
            if other.x == 0 && other.y == 0 {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        }
        if other.x == 0 && other.y == 0 {
            return Ordering::Greater;
        }

        let det = self.x * other.y - self.y * other.x;
        if self.y >= 0 {
            match det.cmp(&0) {
                Ordering::Less => {
                    if other.y >= 0 {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
                Ordering::Equal => {
                    if self.x.signum() == other.x.signum() && self.y.signum() == other.y.signum() {
                        Ordering::Equal
                    } else if self.y == 0 {
                        if self.x >= 0 {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        Ordering::Less
                    }
                }
                Ordering::Greater => Ordering::Less,
            }
        } else {
            match det.cmp(&0) {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => {
                    if self.x.signum() == other.x.signum() && self.y.signum() == other.y.signum() {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                }
                Ordering::Greater => {
                    if other.y >= 0 {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn partial_cmp() {
        let a = Direction::new(2, 1);
        let b = Direction::new(1, 2);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));

        let a = Direction::new(2, 1);
        let b = Direction::new(-1, 2);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));

        let a = Direction::new(2, 1);
        let b = Direction::new(-1, -2);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));

        let a = Direction::new(2, 1);
        let b = Direction::new(1, -2);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));

        let a = Direction::new(-3, 1);
        let b = Direction::new(-1, 0);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));

        let a = Direction::new(-1, 0);
        let b = Direction::new(3, 0);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater));

        let a = Direction::new(-3, 3);
        let b = Direction::new(1, -1);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    }
}
