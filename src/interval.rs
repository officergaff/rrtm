pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
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

    pub fn empty() -> Self {
        Interval::new(f64::INFINITY, -f64::INFINITY)
    }

    pub fn universe() -> Self {
        Interval::new(-f64::INFINITY, f64::INFINITY)
    }
}
