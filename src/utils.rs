use vecmat::vec::Vec3;
use rand::{self, Rng};

pub fn get_point_in_unit_sphere() -> Vec3<f64> {
    // Rejection method
    // #TODO - I don't care for it; should replace
    let mut rng = rand::thread_rng();

    loop {
        let point = 2.0 * Vec3::<f64>::from(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vec3::<f64>::from(1.0, 1.0, 1.0);

        if point.sqrlen() >= 1.0 {
            return point;
        }
    }
}