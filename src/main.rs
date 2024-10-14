use std::io::Write;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let out = std::io::stdout();

    let _ = writeln!(&out, "P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            let _ = writeln!(&out, "{} {} {}\n", ir, ig, ib);
        }
    }
}
