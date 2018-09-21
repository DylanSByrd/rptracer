use vecmat::vec::Vec3;

use material::*;
use utils;

pub struct Lambert {
    pub albedo: Vec3<f64>,
}

impl Lambert {
    pub fn new(albedo: Vec3<f64>) -> Lambert {
        Lambert {
            albedo,
        }
    }
}

impl Material for Lambert {
    fn scatter_ray(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterResult> {
        let target = hit.position + hit.normal + utils::get_point_in_unit_sphere();
        let result = ScatterResult {
            attenuation: self.albedo,
            ray: Ray::new(hit.position, target - hit.position),
        };

        Some(result)
    }
}