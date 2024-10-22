use crate::vec3::Vec3;

pub type Point3 = Vec3;

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir, tm: 0. }
    }
    pub fn new_tm(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod ray {
    use super::*;
    #[test]
    fn position_at_t() {
        let t = 5.;
        let start_pos = Point3::new(0., 0., 0.);
        let dir = Vec3::new(0., 0., 1.);
        let ray = Ray::new(start_pos, dir);
        let end_pos = ray.at(t);
        assert_eq!(end_pos, Point3::new(0., 0., 5.));
    }
}
