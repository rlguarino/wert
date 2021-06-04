mod math;
mod ray;

use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use std::f64;

fn vec_color(v: &math::Vec3) -> Rgb<u8> {
    let r = v.x;
    let g = v.y;
    let b = v.z;
    Rgb([(256.0 * r) as u8, (256.0 * g) as u8, (256.0 * b) as u8])
}

fn ray_color(ray: &ray::Ray) -> Rgb<u8> {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    // TODO Why do I need to do so many borrows?
    // TODO Allow scalars in LHS of vector multiplication.
    let c = &(&math::Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * &(1.0 - t))
        + &(&math::Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * &t);
    vec_color(&c)
}

fn main() -> Result<(), image::error::ImageError> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = math::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = math::Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = math::Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner: math::Vec3 = &origin
        - &horizontal / &2.0
        - &vertical / &2.0
        - math::Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    let mut img = RgbImage::new(image_width, image_height);
    let pb = ProgressBar::new(image_height as u64);
    dbg!(image_height, image_width);
    for j in 0..image_height {
        pb.set_position(j as u64);
        for i in 0..image_width {
            let u: f64 = i as f64 / (image_width - 1) as f64;
            // Image vertical coordinate system (x,y) is inverted from spacial coordinates (u,v)
            // See: https://github.com/rust-lang/cargo/issues/1634
            let v: f64 = (image_height - j) as f64 / (image_height - 1) as f64;

            // TODO Understand why putting "- &origin" at the end of this line conplained about expecting a &Vec3
            let ray = ray::Ray {
                origin: origin.clone(),
                direction: &lower_left_corner - &origin + u * &horizontal + v * &vertical,
            };

            let pixel = ray_color(&ray);
            img.put_pixel(i, j, pixel);
        }
    }
    pb.finish_and_clear();
    img.save("image.png")
}
