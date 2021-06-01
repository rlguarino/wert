use image::{Rgb, RgbImage};
use indicatif::ProgressBar;

fn main() -> Result<(), image::error::ImageError> {
    let image_width = 256;
    let image_height = 256;

    let mut img = RgbImage::new(image_width, image_height);
    let pb = ProgressBar::new(256);
    for j in 0..image_height {
        pb.set_position(j as u64);
        for i in 0..image_width {
            let r = i as f64 / ((image_width - 1) as f64);
            let g = j as f64 / ((image_height - 1) as f64);
            let b = 0.25;

            let pixel = Rgb([(256.0 * r) as u8, (256.0 * g) as u8, (256.0 * b) as u8]);
            img.put_pixel(j, i, pixel);
        }
    }
    pb.finish_and_clear();
    img.save("image.png")
}
