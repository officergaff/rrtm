use crate::{
    color::Color,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::{Point3, Ray},
    sphere::hit_sphere,
    utils::{degrees_to_radians, random_double},
    vec3::{cross, unit_vector, Vec3},
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use std::sync::Arc;

pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32, // random sampling per pixel for antialiasing
    pixel_samples_scale: f64,
    pub max_depth: i32, // ray bounce depth
    pub vfov: f64,      // vertical view angle -> field of view
    lookfrom: Point3,   // point where camera is looking from
    lookat: Point3,     // point where camera is looking at
    vup: Vec3,          // rotation angle of camera

    u: Vec3, // camera frame basis vectors
    v: Vec3,
    w: Vec3,

    defocus_angle: f64,   // variation angle of rays through each pixel
    focus_dist: f64,      // perfect focus distance
    defocus_disk_u: Vec3, // defocus disk horizontal radius
    defocus_disk_v: Vec3, // defocus disk vertical radius
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
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1. / samples_per_pixel as f64;
        // Camera
        let camera_center = lookfrom;
        let w = unit_vector(&(lookfrom - lookat)); // z-axis, the directional vector that
                                                   // looks at the object
        let u = unit_vector(&cross(vup, w)); // the x axis of the camera looking
                                             // at object
        let v = cross(w, u); // y-axis

        // Viewport dimensions
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.);
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate location of the upper left pixel
        let viewport_upper_left =
            camera_center - (w * focus_dist) - (viewport_u / 2.) - (viewport_v / 2.);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u * pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Self {
            image_width,
            image_height,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
            samples_per_pixel,
            max_depth,
            vfov,
            pixel_samples_scale,
            defocus_angle,
            focus_dist,
            viewport_width,
            viewport_height,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00_loc,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    #[cfg(feature = "parallel")]
    pub fn render(&self, world: &Arc<dyn Hittable>) -> Vec<String> {
        return (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                let row: Vec<String> = (0..self.image_width)
                    .into_par_iter()
                    .map(|i| {
                        let pixel_color: Color = (0..self.samples_per_pixel)
                            .into_par_iter() // Make this parallel too
                            .map(|_| {
                                let r = self.get_ray(i, j);
                                self.ray_color(r, world, self.max_depth)
                            })
                            .reduce(|| Color::default(), |acc, color| acc + color);
                        let rgb = (pixel_color * self.pixel_samples_scale).get_rgb();
                        format!("{} {} {}", rgb[0], rgb[1], rgb[2])
                    })
                    .collect();
                row
            })
            .collect();
    }

    pub fn ray_color(&self, ray: Ray, world: &Arc<dyn Hittable>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        let mut rec: HitRecord = Default::default();

        // Fix for shadow acne, due to floating point rounding errors, the reflected ray might end
        // up being under surface of the object, we limit the minimum intersect distance
        if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            // let direction = Vec3::random_on_hemisphere(*rec.normal); --- Uniform Reflection
            // let direction = rec.normal + Vec3::random_unit_vector(); // Lambertian Reflection
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .material
                .as_ref()
                .unwrap()
                .scatter(&ray, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * self.ray_color(scattered, world, depth - 1);
            }
            return Color::default();
        }

        let unit_direction = unit_vector(&ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return Color::new(1., 1., 1.) * (1. - a) + Color::new(0.5, 0.7, 1.) * a;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk, and directed at a randomly
        // sampled point around the pixel location i, j
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x() + i as f64))
            + (self.pixel_delta_v * (offset.y() + j as f64));
        let ray_origin = if self.defocus_angle <= 0. {
            self.lookfrom
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        return Ray::new_tm(ray_origin, ray_direction, ray_time);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk
        let p = Vec3::random_in_unit_disk();
        self.lookfrom + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.)
}

#[cfg(test)]
mod camera {
    //Test viewport calculations
    //Test pixel00 calculation
    //Test focal length calculation
}
