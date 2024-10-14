use crate::{
    color::Color,
    ray::{Point3, Ray},
    sphere::hit_sphere,
    vec3::{unit_vector, Vec3},
};
use rayon::prelude::*;

pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    camera_center: Point3,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64, focal_length: f64) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };

        // Camera
        let viewport_height = 2.;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3::new(0., 0., 0.);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate location of the upper left pixel
        let viewport_upper_left =
            camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u * pixel_delta_v) * 0.5;
        Self {
            image_width,
            image_height,
            camera_center,
            focal_length,
            viewport_width,
            viewport_height,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00_loc,
        }
    }

    pub fn render(&self) -> Vec<String> {
        return (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                let row: Vec<String> = (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let pixel_center = self.pixel00_loc
                            + (self.pixel_delta_u * i as f64)
                            + (self.pixel_delta_v * j as f64);
                        let ray_direction = pixel_center - self.camera_center;
                        let ray = Ray::new(self.camera_center, ray_direction);

                        let pixel_color = ray_color(ray);
                        let rgb = pixel_color.get_rgb();
                        format!("{} {} {}", rgb[0], rgb[1], rgb[2])
                    })
                    .collect();
                row
            })
            .collect();
    }
}

pub fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point3::new(0., 0., -1.), 0.5, &ray) {
        return Color::new(1., 0., 0.);
    }
    let unit_direction = unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Color::new(1., 1., 1.) * (1. - a) + Color::new(0.5, 0.7, 1.) * a;
}

#[cfg(test)]
mod camera {
    //Test viewport calculations
    //Test pixel00 calculation
    //Test focal length calculation
}
