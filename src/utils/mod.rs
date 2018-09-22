use vecmat::vec::*;
use rand::{self, Rng};

pub fn get_point_in_unit_sphere() -> Vec3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let point = 2.0 * Vec3::<f64>::from(rng.gen(), rng.gen(), rng.gen())
            - Vec3::<f64>::from(1.0, 1.0, 1.0);

        if point.sqrlen() < 1.0 {
            return point;
        }
    }
}

pub fn reflect(reflected_vec: &Vec3<f64>, normal: &Vec3<f64>) -> Vec3<f64> {
    *reflected_vec - 2.0 * reflected_vec.dot(*normal) * *normal
}

pub fn refract(refracted_vec: &Vec3<f64>, normal: &Vec3<f64>, ni_over_nt: f64) -> Option<Vec3<f64>> {
    let refracted_dir = refracted_vec.normalize();
    let refract_dot_normal = refracted_dir.dot(*normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - refract_dot_normal * refract_dot_normal);
    if discriminant > 0.0 {
        return Some(ni_over_nt * (refracted_dir - (*normal * refract_dot_normal)) - *normal * discriminant.sqrt());
    }

    None
}

pub fn schlick(cos: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}