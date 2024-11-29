use crate::{interval::Interval, vec3::Vec3};
pub type Color = Vec3;

impl Color {
    pub fn get_rgb(&self) -> [u8; 3] {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let rg = linear_to_gamma(r);
        let gg = linear_to_gamma(g);
        let bg = linear_to_gamma(b);

        let intensity = Interval::new(0.000, 0.999);
        let rbyte = (256. * intensity.clamp(rg)) as u8;
        let gbyte = (256. * intensity.clamp(gg)) as u8;
        let bbyte = (256. * intensity.clamp(bg)) as u8;
        return [rbyte, gbyte, bbyte];
    }
}

// Linear correction for more consistent ramp from darkness to lightness
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return f64::sqrt(linear_component);
    }
    return 0.;
}
