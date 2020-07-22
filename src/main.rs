mod camera;
mod object;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;

pub use camera::Camera;
pub use object::{HitRecord, HittableList, Object, Sphere};
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};

fn main() {
    let x_to_show = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x_to_show);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    let cam = Camera::new(aspect_ratio);

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_height as u64);

    for y in 0..image_height {
        for x in 0..image_width {
            let mut cur_color = Color::zero();
            let mut rng = rand::thread_rng();
            for _s in 0..samples_per_pixel {
                let randa: f64 = rng.gen();
                let randb: f64 = rng.gen();
                let u: f64 = (x as f64 + randa) / (image_width - 1) as f64;
                let v: f64 = (y as f64 + randb) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                cur_color += ray_color(&r, &world);
            }
            cur_color *= 1.0 / (samples_per_pixel as f64);
            write_color(cur_color, &mut img, x, image_height - y - 1);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let rec = world.ray_hit(ray, 0.0, std::f64::INFINITY);
    match rec {
        Some(cur) => {
            let n = cur.normal;
            Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
        }
        None => {
            let u = ray.dir.unit();
            let t = 0.5 * (u.y + 1.0);
            Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn write_color(color: Color, img: &mut RgbImage, x: u32, y: u32) {
    let pixel = img.get_pixel_mut(x, y);
    *pixel = image::Rgb([
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8,
    ]);
}
