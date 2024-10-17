mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::io::Write;

use crate::{camera::Camera, hittable::HittableList, ray::Point3, sphere::Sphere};

fn create_world() -> HittableList {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));
    return world;
}

fn main() {
    let out = std::io::stdout();

    let camera = Camera::new(400, 16. / 9., 1.);

    let _ = writeln!(
        &out,
        "P3\n{} {}\n255\n",
        camera.image_width, camera.image_height
    );
    let mut world = create_world();
    let pixels = camera.render(&world);

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
