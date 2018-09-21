use vecmat::vec::*;

use material::*;
use utils;

pub struct Metal {
    pub albedo: Vec3<f64>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3<f64>, mut fuzz: f64) -> Metal {
        if fuzz > 1.0 {
            fuzz = 1.0;
        }

        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter_ray(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterResult> {
        let reflected = utils::reflect(&ray.dir.normalize(), &hit.normal);
        if reflected.dot(hit.normal) > 0.0 {
            return Some(ScatterResult {
                attenuation: self.albedo,
                ray: Ray::new(hit.position, reflected + self.fuzz*utils::get_point_in_unit_sphere()),
            });
        }
        
        None
    }
}