use vecmat::vec::*;
use rand;

use material::*;
use utils;

pub struct Dielectric {
    pub refractive_index : f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric {
            refractive_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter_ray(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterResult> {
        let outward_normal: Vec3<f64>;
        let ni_over_nt: f64;
        let cosine: f64;

        if ray.dir.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * ray.dir.dot(hit.normal) / ray.dir.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -ray.dir.dot(hit.normal) / ray.dir.length();
        }

        let attenuation = Vec3::<f64>::from(1.0, 1.0, 1.0);
        match utils::refract(&ray.dir, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rand::random::<f64>() > utils::schlick(cosine, self.refractive_index) {
                    return Some(ScatterResult {
                        attenuation,
                        ray: Ray::new(hit.position, refracted),
                    });
                }
            }
            None => {}
        }

        Some(ScatterResult {
            attenuation,
            ray: Ray::new(hit.position, utils::reflect(&ray.dir, &hit.normal)),
        })
    }
}