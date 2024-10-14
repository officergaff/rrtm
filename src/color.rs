use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn get_rgb(&self) -> [i32; 3] {
        let r = (255.999 * self.x()) as i32;
        let g = (255.999 * self.y()) as i32;
        let b = (255.999 * self.z()) as i32;
        return [r, g, b];
    }
}
