use vecmat::vec::Vec3;

use ray::Ray;
use utils;

pub struct Camera {
    pub lower_left_corner: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub origin: Vec3<f64>,
    pub lens_radius: f64,
    pub back: Vec3<f64>,
    pub right: Vec3<f64>,
    pub up: Vec3<f64>,
}

impl Camera {
    pub fn new(look_from: Vec3<f64>, look_at: Vec3<f64>, view_up: Vec3<f64>, vfov_degrees: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let vfov_rad = vfov_degrees.to_radians();
        let half_height = (vfov_rad * 0.5).tan();
        let half_width = aspect * half_height;

        let back = (look_from - look_at).normalize();
        let right = view_up.cross(back).normalize();
        let up = back.cross(right);
        let lower_left_corner = look_from - (half_width * focus_dist * right + half_height * focus_dist * up + focus_dist * back);

        Camera {
            lower_left_corner,
            horizontal: 2.0 * half_width * focus_dist * right,
            vertical: 2.0 * half_height * focus_dist * up,
            origin: look_from,
            lens_radius: aperture * 0.5,
            back,
            right,
            up,
        }
    }

    pub fn get_ray_at_uv(&self, s: f64, t: f64) -> Ray {
        let random_dir = self.lens_radius * utils::get_point_in_unit_disk();
        let offset = self.right * random_dir[0] + self.up * random_dir[1];
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}