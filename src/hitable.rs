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

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            list: Vec::new(),
        }
    }
}

impl Hitable for HitableList {
    fn try_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let mut closest_so_far = t_max;
        let mut current_result : Option<HitInfo> = None;
        
        for hitable in &self.list {
            match hitable.try_hit(ray, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.time;
                    current_result = Some(hit)
                }
                None => {}
            }
        }

        current_result
    }
}