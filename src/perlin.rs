use crate::utils::random_int;

const POINT_COUNT: usize = 256;
struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

// WIP
impl Perlin {
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
}
