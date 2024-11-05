#[derive(Debug, Default, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn with_intervals(a: &Interval, b: &Interval) -> Self {
        // Create the interval tightly enclosing the two input intervals
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }
    pub fn empty() -> Self {
        Interval::new(f64::INFINITY, -f64::INFINITY)
    }
    pub fn universe() -> Self {
        Interval::new(-f64::INFINITY, f64::INFINITY)
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return x >= self.min && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return x > self.min && x < self.max;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }
}
