use crate::{
    ray::Point3,
    utils::{random_double, random_int},
};

const POINT_COUNT: usize = 256;

#[derive(Debug)]
pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = random_double()
        }
        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];
        Self::generate_perm(&mut perm_x);
        Self::generate_perm(&mut perm_y);
        Self::generate_perm(&mut perm_z);
        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    fn generate_perm(p: &mut [i32]) {
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        Self::permute(p, POINT_COUNT);
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in (0..n).rev() {
            let target = random_int(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        // Don't fully understand the mechanics here with the &
        let i = ((4. * p.x()) as i32 & 255) as usize;
        let j = ((4. * p.y()) as i32 & 255) as usize;
        let k = ((4. * p.z()) as i32 & 255) as usize;

        return self.randfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize];
    }
}
