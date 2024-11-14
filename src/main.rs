mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod utils;
mod vec3;

use std::{f64::consts, fs::File, io::Write, sync::Arc};

use bvh::BVHNode;
use hittable::Hittable;
use texture::{CheckerTexture, ImageTexture, NoiseTexture};

use crate::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::*,
    ray::Point3,
    sphere::Sphere,
    utils::{random_double, random_double_range},
    vec3::Vec3,
};

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let out = std::io::stdout();

    let (camera, world) = perlin();
    let pixels = camera.render(&world);
    let _ = writeln!(
        &out,
        "P3\n{} {}\n255\n",
        camera.image_width, camera.image_height
    );
    for p in pixels {
        let _ = writeln!(&out, "{}", p);
    }
    let elapsed = now.elapsed();
    dbg!(elapsed);
}

fn perlin() -> (Camera, Arc<dyn Hittable>) {
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(400, 16. / 9., 100, 50, 20., lookfrom, lookat, vup, 0., 12.);

    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new());
    let ground = Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::with_texture(pertext.clone())),
    ));
    let sphere = Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::with_texture(pertext.clone())),
    ));
    world.add(ground);
    world.add(sphere);
    (camera, BVHNode::new(&mut world) as Arc<dyn Hittable>)
}
fn mike() -> (Camera, Arc<dyn Hittable>) {
    let lookfrom = Point3::new(0., 0., 12.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(400, 16. / 9., 100, 50, 20., lookfrom, lookat, vup, 0., 12.);

    let mut world = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("cat.jpg"));
    let earth_surface = Arc::new(Lambertian::with_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));
    world.add(globe);

    (camera, BVHNode::new(&mut world) as Arc<dyn Hittable>)
}
fn earth() -> (Camera, Arc<dyn Hittable>) {
    let lookfrom = Point3::new(0., 0., 12.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(400, 16. / 9., 100, 50, 20., lookfrom, lookat, vup, 0., 12.);

    let mut world = HittableList::new();
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::with_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));
    world.add(globe);

    (camera, BVHNode::new(&mut world) as Arc<dyn Hittable>)
}

fn checkered_sphere() -> (Camera, Arc<dyn Hittable>) {
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(400, 16. / 9., 100, 50, 20., lookfrom, lookat, vup, 0.6, 10.);
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::with_color(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Arc::new(Lambertian::with_texture(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Arc::new(Lambertian::with_texture(checker)),
    )));
    (camera, BVHNode::new(&mut world) as Arc<dyn Hittable>)
}

fn render_much_sphere() -> (Camera, Arc<dyn Hittable>) {
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let camera = Camera::new(400, 16. / 9., 100, 50, 20., lookfrom, lookat, vup, 0.6, 10.);
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::with_color(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    let ground_mat = Arc::new(Lambertian::with_texture(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_mat,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let mat: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    mat = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_double_range(0., 0.5);
                    mat = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    mat = Arc::new(Dielectric::new(1.5));
                }
                let center2 = center + Vec3::new(0., random_double_range(0., 0.5), 0.);
                world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, mat)));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1., mat1)));

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., mat2)));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., mat3)));
    return (camera, BVHNode::new(&mut world) as Arc<dyn Hittable>);
}

fn wide_angle_test() -> HittableList {
    let mut world = HittableList::new();

    let R = f64::cos(consts::PI / 4.);

    let material_left = Arc::new(Lambertian::new(Color::new(0., 0., 1.)));
    let material_right = Arc::new(Lambertian::new(Color::new(1., 0., 0.)));

    let left = Arc::new(Sphere::new(Point3::new(-R, 0., -1.), R, material_left));
    let right = Arc::new(Sphere::new(Point3::new(R, 0., -1.), R, material_right));

    world.add(left);
    world.add(right);

    world
}
fn basic_world() -> HittableList {
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.))),
    )));
    world.add(Arc::new(Sphere::new(
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

    let ground = Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Arc::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        material_center.clone(),
    ));
    let left = Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    let bubble = Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble.clone(),
    ));
    let right = Arc::new(Sphere::new(
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

    let ground = Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Arc::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        material_center.clone(),
    ));
    let left = Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    let right = Arc::new(Sphere::new(
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

    let ground = Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    ));
    let center = Arc::new(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        material_center.clone(),
    ));
    let left = Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));
    let right = Arc::new(Sphere::new(
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

#[cfg(test)]
mod render {
    use super::*;

    #[test]
    fn test_render() {}
}
