pub use crate::aabb::*;
pub use crate::ray::Ray;
pub use crate::texture::*;
pub use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;
pub use std::{sync::Arc, vec};

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
    fn get_background(&self, t: f64) -> Color;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: Point3 {
                x: c.x,
                y: c.y,
                z: c.z,
            },
            radius: r,
            mat_ptr: m,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.squared_length();
        let half_b = oc * ray.dir;
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let n = (ray.at(temp) - self.center).unit();
                let outward_normal = (ray.at(temp) - self.center) / self.radius;
                let mut rec = HitRecord::new(ray.at(temp), n, temp, self.mat_ptr.clone());
                rec.set_face_normal(ray, &outward_normal);
                let res = get_sphere_uv(&rec.p);
                rec.set_uv(res);
                return Some(rec);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let n = (ray.at(temp) - self.center).unit();
                let outward_normal = (ray.at(temp) - self.center) / self.radius;
                let mut rec = HitRecord::new(ray.at(temp), n, temp, self.mat_ptr.clone());
                rec.set_face_normal(ray, &outward_normal);
                let res = get_sphere_uv(&rec.p);
                rec.set_uv(res);
                return Some(rec);
            }
        }
        Option::None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let output_box = AABB::new(
            self.center - Vec3::ones() * self.radius,
            self.center + Vec3::ones() * self.radius,
        );
        Some(output_box)
    }

    fn get_background(&self, _t: f64) -> Color {
        Color::zero()
    }
}

fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
    let v = (theta + std::f64::consts::PI / 2.0) / std::f64::consts::PI;
    (u, v)
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, n: Vec3, tin: f64, m: Arc<dyn Material>) -> Self {
        Self {
            p: Point3 {
                x: point.x,
                y: point.y,
                z: point.z,
            },
            normal: Vec3 {
                x: n.x,
                y: n.y,
                z: n.z,
            },
            mat_ptr: m,
            t: tin,
            u: 0.0,
            v: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = (r.dir * *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn set_uv(&mut self, res: (f64, f64)) {
        self.u = res.0;
        self.v = res.1;
    }
}

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Object>>,
    pub if_dark: bool,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new(true)
    }
}

impl HittableList {
    pub fn new(if_dark: bool) -> Self {
        Self {
            objects: vec::Vec::new(),
            if_dark,
        }
    }

    pub fn push(&mut self, ob: Arc<dyn Object>) {
        self.objects.push(ob);
    }
}

impl Object for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut cur_rec = Option::<HitRecord>::None;

        for object in &self.objects {
            let this_rec = object.hit(ray, t_min, closest_so_far);
            if let Some(cur) = this_rec {
                closest_so_far = cur.t;
                cur_rec = Some(cur).clone();
            }
        }

        cur_rec
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box = AABB::new(Point3::zero(), Point3::zero());
        let mut first_box = true;

        for object in &self.objects {
            let res = object.bounding_box(t0, t1);
            match res {
                Some(temp_box) => {
                    output_box = if first_box {
                        temp_box
                    } else {
                        surrounding_box(output_box, temp_box)
                    };
                    first_box = false;
                }
                None => {
                    return None;
                }
            }
        }
        Some(output_box)
    }

    fn get_background(&self, t: f64) -> Color {
        if self.if_dark {
            Color::zero()
        } else {
            Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        let tex = SolidColor::new(a);
        Self {
            albedo: Arc::new(tex),
        }
    }

    pub fn new_arc(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + crate::vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        Self {
            albedo: Color {
                x: a.x,
                y: a.y,
                z: a.z,
            },
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = crate::vec3::reflect(&r_in.dir.unit(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + crate::vec3::random_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self.albedo;
        if scattered.dir * rec.normal > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self { ref_idx: ri }
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0_sqrt = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0_sqrt * r0_sqrt;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::ones();
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.dir.unit();

        let cos = -unit_direction * rec.normal;
        let cos_theta = if cos < 1.0 { cos } else { 1.0 };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = crate::vec3::reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some((attenuation, scattered));
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        let flag: f64 = rand::thread_rng().gen();
        if flag < reflect_prob {
            let reflected = crate::vec3::reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some((attenuation, scattered));
        }
        let refracted = crate::vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted);
        Some((attenuation, scattered))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::zero()
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        Self { emit: a }
    }

    pub fn new_color(c: &Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
