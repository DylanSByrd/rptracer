use vecmat::vec::Vec3;

use ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub origin: Vec3<f64>,
}

impl Camera {
    pub fn new_default() -> Camera {
        Camera {
            lower_left_corner: Vec3::<f64>::from(-2.0, -1.0, -1.0),
            horizontal: Vec3::<f64>::from(4.0, 0.0, 0.0),
            vertical: Vec3::<f64>::from(0.0, 2.0, 0.0),
            origin: Vec3::<f64>::from(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray_at_uv(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }
}