mod camera;
mod color;
mod ray;
mod sphere;
mod vec3;

use std::io::Write;

use crate::camera::Camera;

fn main() {
    let out = std::io::stdout();

    let camera = Camera::new(400, 16. / 9., 1.);

    let _ = writeln!(
        &out,
        "P3\n{} {}\n255\n",
        camera.image_width, camera.image_height
    );

    let pixels = camera.render();

    for p in pixels {
        let _ = writeln!(&out, "{}", p);
    }
}

#[cfg(test)]
mod render {
    use super::*;

    #[test]
    fn test_render() {}
}
