pub use crate::ray::Ray;
pub use crate::vec3::Point3;
pub use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub _lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
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
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let sw = (look_from - look_at).unit();
        let su = Vec3::cross(vup, sw).unit();
        let sv = Vec3::cross(sw, su);

        Self {
            origin: look_from,
            _lower_left_corner: look_from
                - su * viewport_width / 2.0 * focus_dist
                - sv * viewport_height / 2.0 * focus_dist
                - sw * focus_dist,
            horizontal: su * viewport_width * focus_dist,
            vertical: sv * viewport_height * focus_dist,
            w: sw,
            u: su,
            v: sv,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = crate::vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self._lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        )
    }
}
