mod camera;
mod object;
mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
pub use std::{sync, vec};

pub use camera::Camera;
pub use object::{
    Dielectric, HitRecord, HittableList, Lambertian, Material, Metal, Object, Sphere,
};
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};

fn main() {
    let x_to_show = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x_to_show);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let cam = Camera::new(look_from, look_at, vup, 20.0, aspect_ratio);

    let mut world = HittableList::new();

    let material_ground = sync::Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = sync::Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left1 = sync::Arc::new(Dielectric::new(1.5));
    let material_left2 = sync::Arc::new(Dielectric::new(1.5));
    let material_right = sync::Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 0.0));

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left1,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left2,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

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
                cur_color += ray_color(&r, &world, max_depth);
            }
            cur_color *= 1.0 / (samples_per_pixel as f64);
            write_color(cur_color, &mut img, x, image_height - y - 1);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let rec = world.ray_hit(ray, 0.001, std::f64::INFINITY);

    if depth <= 0 {
        return Color::zero();
    }

    match rec {
        Some(cur_rec) => {
            let cur_data = cur_rec.mat_ptr.scatter(ray, &cur_rec);
            match cur_data {
                Some((attenuation, scattered)) => {
                    Vec3::elemul(attenuation, ray_color(&scattered, world, depth - 1))
                }
                None => Color::zero(),
            }
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
    let colorx = color.x.sqrt();
    let colory = color.y.sqrt();
    let colorz = color.z.sqrt();
    *pixel = image::Rgb([
        (within(0.0, 0.999, colorx) * 256.0) as u8,
        (within(0.0, 0.999, colory) * 256.0) as u8,
        (within(0.0, 0.999, colorz) * 256.0) as u8,
    ]);
}

fn within(min: f64, max: f64, value: f64) -> f64 {
    if value > max {
        return max;
    }
    if value < min {
        return min;
    }
    value
}
