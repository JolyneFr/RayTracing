pub use crate::ray::Ray;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Point3,
    pub _lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        Self {
            origin: look_from,
            _lower_left_corner: look_from
                - u * viewport_width / 2.0
                - v * viewport_height / 2.0
                - w,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self._lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}
