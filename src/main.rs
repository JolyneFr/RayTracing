mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use ray::Ray;
pub use vec3::Color;
pub use vec3::Point3;
pub use vec3::Vec3;

fn ray_color(ray: &Ray) -> Color {
    let u = ray.dir.unit();
    let t = 0.5 * (u.y + 1.0);
    Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let aspect_ratio = 1.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    //let origin = Point3::zero();
    //let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    //let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let _lower_left_corner =
        Vec3::new(-viewport_width / 2.0, -viewport_height / 2.0, -focal_length);

    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_height as u64);

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel = img.get_pixel_mut(x, image_height - y - 1);
            let u: f64 = x as f64 / (image_width - 1) as f64;
            let v: f64 = y as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                Point3::zero(),
                Vec3::new(
                    (u - 0.5) * viewport_width,
                    (v - 0.5) * viewport_height,
                    -focal_length,
                ),
            );
            if (x - image_width / 2) * (x - image_width / 2)
                + (y - image_height / 2) * (y - image_height / 2)
                < (0.3125 * 0.3125 * image_width as f64 * image_width as f64) as u32
            {
                if (x - image_width / 2) * (x - image_width / 2)
                    + (y - image_height / 2) * (y - image_height / 2)
                    > (0.3 * 0.3 * image_width as f64 * image_width as f64) as u32
                {
                    *pixel = image::Rgb([1, 1, 1]);
                } else {
                    *pixel = image::Rgb([255, 255, 255]);
                    if (x - image_width / 2) * (x - image_width / 2)
                        + (y - image_height / 2) * (y - image_height / 2)
                        < (0.25 * 0.25 * image_width as f64 * image_width as f64) as u32
                        && (((x - image_width / 2) as f64) / ((y - image_height / 2) as f64) > 0.8
                            && ((x - image_width / 2) as f64) / ((y - image_height / 2) as f64)
                                < 0.9)
                    {
                        *pixel = image::Rgb([1, 1, 1]);
                    }
                    if (x - image_width / 2) * (x - image_width / 2)
                        + (y - image_height / 2) * (y - image_height / 2)
                        < (0.16 * 0.16 * image_width as f64 * image_width as f64) as u32
                        && ((x as i32 - (image_width / 2) as i32) as f64
                            / (y as i32 - (image_height / 2) as i32) as f64
                            > -6.5
                            && (x as i32 - (image_width / 2) as i32) as f64
                                / ((y as i32 - (image_height / 2) as i32) as f64)
                                < -4.0
                            && x < image_width / 2)
                    {
                        *pixel = image::Rgb([1, 1, 1]);
                    }
                }
            } else {
                let cur_color = ray_color(&r);
                //println!("{:?}", cur_color);
                let colorx = (cur_color.x * 255.999) as u8;
                let colory = (cur_color.y * 255.999) as u8;
                let colorz = (cur_color.z * 255.999) as u8;
                *pixel = image::Rgb([colorx, colory, colorz]);
            }
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
