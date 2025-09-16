use std::{f64::INFINITY, ops};

#[macro_export]
macro_rules! int {
    ($from:expr, $to:expr) => {
        Interval::new($from, $to)
    };
    ($from:expr) => {
        Interval::from($from)
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn from(min: f64) -> Self {
        Interval { min, max: INFINITY }
    }

    pub fn universe() -> Self {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn pad(&mut self, x: f64) -> &Self {
        let padding = x / 2.0;
        self.min -= padding;
        self.max += padding;
        self
    }

    pub fn padded(&self, x: f64) -> Self {
        let padding = x / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl ops::Add for Interval {
    type Output = Interval;

    fn add(self, rhs: Self) -> Self::Output {
        Interval {
            min: if self.min <= rhs.min {
                self.min
            } else {
                rhs.min
            },
            max: if self.max >= rhs.max {
                self.max
            } else {
                rhs.max
            },
        }
    }
}

impl ops::AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        self.min = self.min.min(rhs.min);
        self.max = self.max.max(rhs.max);
    }
}

impl ops::Add<f64> for Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl ops::AddAssign<f64> for Interval {
    fn add_assign(&mut self, rhs: f64) {
        self.min -= rhs;
        self.max += rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        let i = int!(3.0, 7.0);
        assert_eq!(i.size(), 4.0);
    }

    #[test]
    fn test_contains() {
        let i = int!(1.0, 2.0);
        assert!(i.contains(1.5));
        assert!(i.contains(1.0));
        assert!(i.contains(2.0));

        assert!(!i.contains(0.5));
        assert!(!i.contains(2.5));
    }

    #[test]
    fn test_surrounds() {
        let i = int!(1.0, 2.0);
        assert!(i.surrounds(1.5));
        assert!(!i.surrounds(1.0));
        assert!(!i.surrounds(2.0));

        assert!(!i.surrounds(0.5));
        assert!(!i.surrounds(2.5));
    }

    #[test]
    fn test_clamp() {
        let i = int!(1.0, 2.0);
        assert_eq!(i.clamp(0.5), 1.0);
        assert_eq!(i.clamp(2.5), 2.0);
        assert_eq!(i.clamp(1.5), 1.5);
    }

    #[test]
    fn test_add() {
        assert_eq!(int!(1.0, 2.0) + int!(3.0, 4.0), int!(1.0, 4.0));
        assert_eq!(int!(2.0, 3.0) + int!(1.0, 4.0), int!(1.0, 4.0));
    }

    #[test]
    fn test_add_assign() {
        let mut i = int!(1.0, 2.0);
        i += int!(3.0, 4.0);
        assert_eq!(i, int!(1.0, 4.0));

        let mut i = int!(2.0, 3.0);
        i += int!(1.0, 4.0);
        assert_eq!(i, int!(1.0, 4.0));
    }

    #[test]
    fn test_add_scalar() {
        let i = int!(1.0, 2.0);
        assert_eq!(i + 0.5, int!(0.75, 2.25));
    }

    #[test]
    fn test_add_assign_scalar() {
        let mut i = int!(1.0, 2.0);
        i += 0.5;
        assert_eq!(i, int!(0.75, 2.25));
    }
}
