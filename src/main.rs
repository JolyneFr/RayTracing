mod aabb;
mod aarect;
mod bvh;
mod camera;
mod object;
mod ray;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
pub use std::{sync, vec};

pub use aarect::*;
pub use camera::Camera;
pub use object::*;
pub use ray::Ray;
pub use texture::*;
pub use vec3::{Color, Point3, Vec3};

fn main() {
    let x_to_show = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x_to_show);

    let aspect_ratio = 3.0 / 2.0;
    let image_width = 900;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 400;
    let max_depth = 50;

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

    let world_sence = light_world();
    let look_from = Point3::new(9.0, 4.0, 4.0);
    let look_at = Point3::new(2.5, 1.0, 1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.01;
    let background = Color::zero();

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world = bvh::BvhNode::new_boxed(world_sence, 0.0, 0.001);

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
                cur_color += ray_color(&r, background, &*world, max_depth);
            }
            cur_color *= 1.0 / (samples_per_pixel as f64);
            write_color(cur_color, &mut img, x, image_height - y - 1);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}

fn ray_color(ray: &Ray, background: Color, world: &dyn Object, depth: i32) -> Color {
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
                    Vec3::elemul(
                        attenuation,
                        ray_color(&scattered, background, world, depth - 1),
                    ) + emitted
                }
                None => emitted,
            }
        }
        None => {
            //let u = ray.dir.unit();
            //let t = 0.5 * (u.y + 1.0);
            //Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            background
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

fn random_double() -> f64 {
    rand::thread_rng().gen()
}

fn random_double_in(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}

/*
fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker_texture = sync::Arc::new(CheckerTexture::new(
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        sync::Arc::new(Lambertian::new_arc(checker_texture)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.65 {
                    //diffuse
                    let albedo = Vec3::elemul(Color::random_unit(), Color::random_unit());
                    let sphere_material = sync::Arc::new(Lambertian::new(&albedo));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.9 {
                    //metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double_in(0.0, 0.5);
                    let sphere_material = sync::Arc::new(Metal::new(&albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    let sphere_material = sync::Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = sync::Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = sync::Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = sync::Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}


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

fn light_world() -> HittableList {
    let mut world = HittableList::new();
    let mut sphere_pos = vec![] as Vec<(Vec3, f64)>;

    let checker_texture = sync::Arc::new(CheckerTexture::new(
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        sync::Arc::new(Lambertian::new_arc(checker_texture)),
    )));

    let center = Point3::new(0.0, 0.6, 0.0);
    let albedo1 = Vec3::elemul(Color::random_unit(), Color::random_unit()) * 1.7;
    let albedo2 = albedo1 * 1.4;
    let checker_texture2 = sync::Arc::new(CheckerTexture::new(&albedo1, &albedo2));
    let tex = sync::Arc::new(DiffuseLight::new(checker_texture2));
    world.push(Box::new(Sphere::new(center, 0.6, tex)));
    sphere_pos.push((center, 0.6));

    for a in -11..11 {
        for b in -11..11 {
            let mut radius = 0.05 + 0.25 * random_double();
            let choose_mat = random_double();
            let mut center = Point3::new(
                (a as f64 + 0.9 * random_double()) * 0.5,
                radius,
                (b as f64 + 0.9 * random_double()) * 0.5,
            );

            let mut flag = true;
            loop {
                if sphere_pos.iter().all(|(cur_center, cur_radius)| {
                    (*cur_center - center).squared_length() > (cur_radius + radius).powf(2.0)
                }) {
                    sphere_pos.push((center, radius));
                    break;
                } else {
                    radius *= 0.7;
                    center.y = radius;
                    if radius < 0.02 {
                        flag = false;
                        break;
                    }
                }
            }
            if !flag {
                continue;
            }

            if choose_mat < 0.3 {
                //light
                let albedo = Vec3::elemul(Color::random_unit(), Color::random_unit()) * 4.0;
                let difflight = sync::Arc::new(DiffuseLight::new_color(&albedo));
                world.push(Box::new(Sphere::new(center, radius, difflight)));
            } else if choose_mat < 0.65 {
                //diffuse
                let albedo = Vec3::elemul(Color::random_unit(), Color::random_unit());
                let sphere_material = sync::Arc::new(Lambertian::new(&albedo));
                world.push(Box::new(Sphere::new(center, radius, sphere_material)));
            } else if choose_mat < 0.9 {
                //metal
                let albedo = Color::random(0.5, 1.0);
                let fuzz = random_double_in(0.0, 0.5);
                let sphere_material = sync::Arc::new(Metal::new(&albedo, fuzz));
                world.push(Box::new(Sphere::new(center, radius, sphere_material)));
            } else {
                //glass
                let sphere_material = sync::Arc::new(Dielectric::new(1.5));
                world.push(Box::new(Sphere::new(center, radius, sphere_material)));
            }
        }
    }

    world
}
