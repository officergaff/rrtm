mod color;
mod ray;
mod vec3;

use rayon::prelude::*;
use std::io::Write;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let out = std::io::stdout();

    let _ = writeln!(&out, "P3\n{} {}\n255\n", image_width, image_height);

    let pixels: Vec<String> = render_pixels(image_width, image_height);
    for p in pixels {
        let _ = writeln!(&out, "{}", p);
    }
}

fn render_pixels(image_width: i32, image_height: i32) -> Vec<String> {
    return (0..image_height)
        .into_par_iter()
        .flat_map(|j| {
            let row: Vec<String> = (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let r = i as f64 / (image_width - 1) as f64;
                    let g = j as f64 / (image_height - 1) as f64;
                    let b = 0.;

                    let ir = (255.999 * r) as u32;
                    let ig = (255.999 * g) as u32;
                    let ib = (255.999 * b) as u32;

                    format!("{} {} {}\n", ir, ig, ib)
                })
                .collect();
            row
        })
        .collect();
}

#[cfg(test)]
mod render {
    use super::*;

    #[test]
    fn test_render() {
        let image_width = 256;
        let image_height = 256;
        let pixels = render_pixels(image_width, image_height);
        assert_eq!(image_height * image_width, pixels.len() as i32);
    }
}
