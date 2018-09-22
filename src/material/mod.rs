use vecmat::vec::Vec3;

use ray::Ray;
use hitable::HitInfo;

pub mod lambert;
pub mod metal;
pub mod dielectric;

pub struct ScatterResult {
    pub attenuation: Vec3<f64>,
    pub ray: Ray,
}

pub trait Material {
    fn scatter_ray(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterResult>;
}