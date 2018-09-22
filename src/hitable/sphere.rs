use std::rc::Rc;

use vecmat::vec::*;

use ray::Ray;
use hitable::*;
use material::Material;

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
    pub material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn try_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let ray_to_center = ray.origin - self.center;
        let a = ray.dir.dot(ray.dir);
        let b = ray.dir.dot(ray_to_center);
        let c = ray_to_center.dot(ray_to_center) - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let near_solution = (-b - discriminant.sqrt()) / a;
            if near_solution < t_max && near_solution > t_min {
                let time = near_solution;
                let position = ray.get_point_at_time(near_solution);
                let normal = (position - self.center) / self.radius;

                return Some(HitInfo::new(time, position, normal, self.material.clone()));
            }

            let far_solution = (-b + discriminant.sqrt()) / a;
            if far_solution < t_max && far_solution > t_min {
                let time = far_solution;
                let position = ray.get_point_at_time(far_solution);
                let normal = (position - self.center) / self.radius;

                return Some(HitInfo::new(time, position, normal, self.material.clone()));
            }
        }
        
        None
    }
}