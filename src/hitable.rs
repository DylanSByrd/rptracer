use vecmat::vec::Vec3;

use ray::Ray;

pub struct HitInfo {
    pub time: f64,
    pub position: Vec3<f64>,
    pub normal: Vec3<f64>,
}

impl HitInfo {
    pub fn new(time: f64, position: Vec3<f64>, normal: Vec3<f64>) -> HitInfo {
        HitInfo {
            time,
            position,
            normal,
        }
    }
}

pub trait Hitable {
    fn try_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}