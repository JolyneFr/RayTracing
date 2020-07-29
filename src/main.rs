mod aabb;
mod aarect;
mod bvh;
mod camera;
mod object;
mod ray;
mod sence;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
pub use std::{sync, vec};

pub use aarect::*;
pub use camera::Camera;
pub use object::*;
pub use ray::Ray;
use rusttype::Font;
pub use sence::*;
use std::sync::mpsc::channel;
pub use texture::*;
use threadpool::ThreadPool;
pub use vec3::{Color, Point3, Vec3};

const AUTHOR: &str = "JolyneFr";

fn get_text() -> String {
    // GITHUB_SHA is the associated commit ID
    // only available on GitHub Action
    let github_sha = option_env!("b11daec439cec665eb4a312eecbba8857c3e1dd5")
        .map(|x| "@".to_owned() + &x[0..6])
        .unwrap_or_default();
    format!("{}{}", AUTHOR, github_sha)
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn _render_text(image: &mut RgbImage, msg: &str) {
    let font_file = if is_ci() {
        "EncodeSans-Regular.ttf"
    } else {
        "/System/Library/Fonts/Helvetica.ttc"
    };
    let font_path = std::env::current_dir().unwrap().join(font_file);
    let data = std::fs::read(&font_path).unwrap();
    let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
        panic!(format!(
            "error constructing a Font from data at {:?}",
            font_path
        ));
    });

    imageproc::drawing::draw_text_mut(
        image,
        Rgb([255, 255, 255]),
        10,
        10,
        rusttype::Scale::uniform(24.0),
        &font,
        msg,
    );
}

fn main() {
    let is_ci = is_ci();
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (8, 4) };
    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);

    let x_to_show = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x_to_show);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 400;
    let max_depth = 50;

    let (world, cam) = init_sence(2);

    /*
    let world_sence = random_scene();
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let background = Color::zero();
    */

    /*
    let look_from = Point3::new(9.0, 4.0, 4.0);
    let look_at = Point3::new(2.5, 1.0, 1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.01;
    */

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr_clone = world.clone();
        pool.execute(move || {
            let row_begin = image_height as usize * i / n_jobs;
            let row_end = image_height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let mut cur_color = Color::zero();
                    let mut rng = rand::thread_rng();
                    for _s in 0..samples_per_pixel {
                        let randa: f64 = rng.gen();
                        let randb: f64 = rng.gen();
                        let u: f64 = (x as f64 + randa) / (image_width - 1) as f64;
                        let v: f64 = (y as f64 + randb) / (image_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        cur_color += ray_color(&r, &*world_ptr_clone, max_depth);
                    }
                    cur_color *= 1.0 / (samples_per_pixel as f64);
                    write_color(cur_color, &mut img, x, img_y as u32);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("fail to send reslut");
        });
    }
    let bar = ProgressBar::new(n_jobs as u64);

    let mut result: RgbImage = ImageBuffer::new(image_width, image_height);

    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *result.get_pixel_mut(col, image_height - row - 1) = *data.get_pixel(col, idx);
            }
        }
        bar.inc(1);
    }
    bar.finish();

    let msg = get_text();
    println!("Extra Info: {}", msg);

    //render_text(&mut result, msg.as_str());

    result.save("output/test.png").unwrap();
}

fn ray_color(ray: &Ray, world: &dyn Object, depth: i32) -> Color {
    let rec = world.hit(ray, 0.001, std::f64::INFINITY);

    if depth <= 0 {
        return Color::zero();
    }

    match rec {
        Some(cur_rec) => {
            let emitted = cur_rec.mat_ptr.emitted(cur_rec.u, cur_rec.v, &cur_rec.p);
            let cur_data = cur_rec.mat_ptr.scatter(ray, &cur_rec);
            match cur_data {
                Some((attenuation, scattered)) => {
                    Vec3::elemul(attenuation, ray_color(&scattered, world, depth - 1)) + emitted
                }
                None => emitted,
            }
        }
        None => {
            let u = ray.dir.unit();
            let t = 0.5 * (u.y + 1.0);
            //Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            world.get_background(t)
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

/*



fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let checker_texture = sync::Arc::new(CheckerTexture::new(
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    let sphere_material1 = sync::Arc::new(Lambertian::new_arc(checker_texture));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        sphere_material1,
    )));

    let albedo2 = Vec3::elemul(Color::random_unit(), Color::random_unit());
    let sphere_material2 = sync::Arc::new(Lambertian::new(&albedo2));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        sphere_material2,
    )));

    let difflight = sync::Arc::new(DiffuseLight::new_color(&Color::new(4.0, 4.0, 4.0)));
    world.push(Box::new(XYrect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    let difflight = sync::Arc::new(DiffuseLight::new_color(&Color::new(4.0, 4.0, 4.0)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    world
}
*/
