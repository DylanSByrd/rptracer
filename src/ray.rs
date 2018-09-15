extern crate vecmat;

use vecmat::vec::*;

struct Ray {
    origin: Vec3<f64>,
    dir: Vec3<f64>,
}

impl Ray {
    fn new() -> Ray {
        Ray {
            origin: Vec3::<f64>::new(),
            dir: Vec3::<f64>::new(),
        }
    }

    fn from_point_and_dir(origin: Vec3<f64>, dir: Vec3<f64>) -> Ray {
        Ray {
            orign,
            dir,
        }
    }

    fn get_point_at_time(&self, time: f64) -> Vec3<f64> {
        origin + (time * dir)
    }
}