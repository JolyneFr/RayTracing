pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: Point3 {
                x: origin.x,
                y: origin.y,
                z: origin.z,
            },
            dir: Vec3 {
                x: direction.x,
                y: direction.y,
                z: direction.z,
            },
        }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
