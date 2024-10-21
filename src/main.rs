mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::{io::Write, sync::Arc};

use crate::{
    camera::Camera, color::Color, hittable::HittableList, material::*, ray::Point3, sphere::Sphere,
};

fn basic_world() -> HittableList {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.))),
    )));
    return world;
}
fn air_bubble() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    let ground = Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Box::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        material_center.clone(),
    ));
    let left = Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    let bubble = Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble.clone(),
    ));
    let right = Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    ));
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(bubble);
    world.add(right);
    return world;
}
fn dielectric_metal_lambertian_world() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    let ground = Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Box::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        material_center.clone(),
    ));
    let left = Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    let right = Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    ));
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    return world;
}

fn metal_lambertian_world() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    let ground = Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Box::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        material_center.clone(),
    ));
    let left = Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    let right = Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    ));
    world.add(ground);
    world.add(center);
    world.add(left);
    world.add(right);
    return world;
}

fn main() {
    let out = std::io::stdout();

    let camera = Camera::new(400, 16. / 9., 1., 50, 50, 90.);

    let _ = writeln!(
        &out,
        "P3\n{} {}\n255\n",
        camera.image_width, camera.image_height
    );
    let world = air_bubble();
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
