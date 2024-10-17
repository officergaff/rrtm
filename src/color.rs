use crate::{interval::Interval, vec3::Vec3};
pub type Color = Vec3;

impl Color {
    pub fn get_rgb(&self) -> [i32; 3] {
        let intensity = Interval::new(0.000, 0.999);
        let r = (256. * intensity.clamp(self.x())) as i32;
        let g = (256. * intensity.clamp(self.y())) as i32;
        let b = (256. * intensity.clamp(self.z())) as i32;
        return [r, g, b];
    }
}
