pub use crate::ray::*;
pub use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min_p: Point3,
    pub max_p: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            min_p: Point3 {
                x: a.x,
                y: a.y,
                z: a.z,
            },
            max_p: Point3 {
                x: b.x,
                y: b.y,
                z: b.z,
            },
        }
    }

    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        let mut t_min = tmin;
        let mut t_max = tmax;

        let invd = 1.0 / r.dir.x;
        let mut t0 = (self.min_p.x - r.orig.x) * invd;
        let mut t1 = (self.max_p.x - r.orig.x) * invd;
        if invd < 0.0 {
            std::mem::swap(&mut t1, &mut t0);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        let invd = 1.0 / r.dir.y;
        let mut t0 = (self.min_p.y - r.orig.y) * invd;
        let mut t1 = (self.max_p.y - r.orig.y) * invd;
        if invd < 0.0 {
            std::mem::swap(&mut t1, &mut t0);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        let invd = 1.0 / r.dir.z;
        let mut t0 = (self.min_p.z - r.orig.z) * invd;
        let mut t1 = (self.max_p.z - r.orig.z) * invd;
        if invd < 0.0 {
            std::mem::swap(&mut t1, &mut t0);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point3::new(
        box0.min_p.x.min(box1.min_p.x),
        box0.min_p.y.min(box1.min_p.y),
        box0.min_p.z.min(box1.min_p.z),
    );
    let big = Point3::new(
        box0.max_p.x.max(box1.max_p.x),
        box0.max_p.y.max(box1.max_p.y),
        box0.max_p.z.max(box1.max_p.z),
    );

    AABB::new(small, big)
}
