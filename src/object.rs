pub use crate::ray::Ray;
pub use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;
pub use std::{sync, vec};

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: sync::Arc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: sync::Arc<dyn Material>) -> Self {
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
                return Some(rec);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let n = (ray.at(temp) - self.center).unit();
                let outward_normal = (ray.at(temp) - self.center) / self.radius;
                let mut rec = HitRecord::new(ray.at(temp), n, temp, self.mat_ptr.clone());
                rec.set_face_normal(ray, &outward_normal);
                return Some(rec);
            }
        }
        Option::None
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: sync::Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, n: Vec3, tin: f64, m: sync::Arc<dyn Material>) -> Self {
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
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Object>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec::Vec::new(),
        }
    }

    pub fn push(&mut self, ob: Box<dyn Object>) {
        self.objects.push(ob);
    }

    pub fn ray_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self {
            albedo: Color {
                x: a.x,
                y: a.y,
                z: a.z,
            },
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + crate::vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
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
}
