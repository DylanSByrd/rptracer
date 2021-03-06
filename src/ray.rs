use vecmat::vec::Vec3;

pub struct Ray {
    pub origin: Vec3<f64>,
    pub dir: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Vec3<f64>, dir: Vec3<f64>) -> Ray {
        Ray {
            origin,
            dir,
        }
    }

    pub fn get_point_at_time(&self, time: f64) -> Vec3<f64> {
        self.origin + (time * self.dir)
    }
}