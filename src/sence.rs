pub use crate::bvh::*;
pub use crate::camera::*;
pub use crate::object::*;
pub use crate::vec3::*;
use rand::Rng;
use std::sync::Arc;

pub fn init_sence(index: u32) -> (Arc<dyn Object>, Camera) {
    match index {
        1 => {
            let world_sence = random_scene();
            let world = BvhNode::new_boxed(world_sence, 0.0, 0.001);

            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let vfov = 20.0;
            let aspect_ratio = 16.0 / 9.0;
            let dist_to_focus = 10.0;
            let aperture = 0.1;
            let cam = Camera::new(
                look_from,
                look_at,
                vup,
                vfov,
                aspect_ratio,
                aperture,
                dist_to_focus,
            );

            (world, cam)
        }
        2 => {
            let world_sence = light_world();
            let world = BvhNode::new_boxed(world_sence, 0.0, 0.001);

            let look_from = Point3::new(9.0, 4.0, 4.0);
            let look_at = Point3::new(2.5, 1.0, 1.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let vfov = 27.0;
            let dist_to_focus = 10.0;
            let aperture = 0.06;
            let aspect_ratio = 16.0 / 9.0;
            let cam = Camera::new(
                look_from,
                look_at,
                vup,
                vfov,
                aspect_ratio,
                aperture,
                dist_to_focus,
            );

            (world, cam)
        }
        _ => panic!("index out of bound"),
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new(false);

    let checker_texture = Arc::new(CheckerTexture::new(
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_arc(checker_texture)),
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
                    let sphere_material = Arc::new(Lambertian::new(&albedo));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.9 {
                    //metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double_in(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}

fn light_world() -> HittableList {
    let mut world = HittableList::new(true);
    let mut sphere_pos = vec![] as Vec<(Vec3, f64)>;

    let checker_texture = Arc::new(CheckerTexture::new(
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_arc(checker_texture)),
    )));

    let center = Point3::new(0.0, 0.6, 0.0);
    let albedo1 = Vec3::elemul(Color::random_unit(), Color::random_unit()) * 1.7;
    let albedo2 = albedo1 * 1.4;
    let checker_texture2 = Arc::new(CheckerTexture::new(&albedo1, &albedo2));
    let tex = Arc::new(DiffuseLight::new(checker_texture2));
    let glass_material = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(center, 0.6, glass_material)));
    world.push(Arc::new(Sphere::new(center, 0.4, tex)));
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
                let check = sphere_pos.iter().all(|(cur_center, cur_radius)| {
                    (*cur_center - center).squared_length() > (cur_radius + radius).powf(2.0)
                });
                if check {
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

            if choose_mat < 0.25 {
                //light
                let albedo = Vec3::elemul(Color::random_unit(), Color::random_unit()) * 4.0;
                let difflight = Arc::new(DiffuseLight::new_color(&albedo));
                world.push(Arc::new(Sphere::new(center, radius, difflight)));
            } else if choose_mat < 0.5 {
                //diffuse
                let albedo = Vec3::elemul(Color::random_unit(), Color::random_unit());
                let sphere_material = Arc::new(Lambertian::new(&albedo));
                world.push(Arc::new(Sphere::new(center, radius, sphere_material)));
            } else if choose_mat < 0.75 {
                //metal
                let albedo = Color::random(0.5, 1.0);
                let fuzz = random_double_in(0.0, 0.5);
                let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                world.push(Arc::new(Sphere::new(center, radius, sphere_material)));
            } else {
                //glass
                let sphere_material = Arc::new(Dielectric::new(1.5));
                world.push(Arc::new(Sphere::new(center, radius, sphere_material)));
                if choose_mat < 0.85 {
                    let albedo_extra = Vec3::elemul(Color::random_unit(), Color::random_unit());
                    let sphere_material = Arc::new(Lambertian::new(&albedo_extra));
                    world.push(Arc::new(Sphere::new(center, radius * 0.9, sphere_material)));
                }
            }
        }
    }

    world
}

fn random_double() -> f64 {
    rand::thread_rng().gen()
}

fn random_double_in(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}
