use vecmat::vec::Vec3;

use ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub origin: Vec3<f64>,
}

impl Camera {
    pub fn new(look_from: Vec3<f64>, look_at: Vec3<f64>, view_up: Vec3<f64>, vfov_degrees: f64, aspect: f64) -> Camera {
        let vfov_rad = vfov_degrees.to_radians();
        let half_height = (vfov_rad * 0.5).tan();
        let half_width = aspect * half_height;

        let back = (look_from - look_at).normalize();
        let right = view_up.cross(back).normalize();
        let up = back.cross(right);
        let lower_left_corner = look_from - (half_width * right + half_height * up + back);

        Camera {
            lower_left_corner,
            horizontal: 2.0 * half_width * right,
            vertical: 2.0 * half_height * up,
            origin: look_from,
        }
    }

    pub fn get_ray_at_uv(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}