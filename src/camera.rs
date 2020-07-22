pub use crate::ray::Ray;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Point3,
    pub _lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Self {
            origin: Point3::zero(),
            _lower_left_corner: Vec3::new(
                -viewport_width / 2.0,
                -viewport_height / 2.0,
                -focal_length,
            ),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            Point3::zero(),
            self._lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
