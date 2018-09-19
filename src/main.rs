extern crate image;
extern crate vecmat;
extern crate rand;

mod ray;
mod hitable;
mod sphere;
mod camera;

use ray::Ray;
use hitable::*;
use sphere::Sphere;
use camera::Camera;

use vecmat::vec::*;
use rand::Rng;

use std::io::Write;
use std::str::FromStr;
use std::f64;

#[derive(Debug)]
struct Config {
    output_path: String,
    dimx: usize,
    dimy: usize,
    num_samples: usize,
}

fn save_image(buf: &[u8], config: &Config) {
    image::save_buffer(&config.output_path, buf, config.dimx as u32, config.dimy as u32, image::RGB(8))
        .expect("Error saving out image to file");
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T,T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn parse_args() -> Config {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        writeln!(std::io::stderr(), "Usage: rptracer FILE DIMENSIONX,DIMENSIONY NUMBER_OF_SAMPLES")
            .unwrap();
        writeln!(std::io::stderr(), "Example: {} image.png 800,600 100", args[0])
            .unwrap();
        std::process::exit(1);
    }

    let output_path = args[1].to_string();
    let dimensions = parse_pair(&args[2], ',')
        .expect("Error parsing dimensions; format is DIMENSIONX,DIMENSIONY");
    let num_samples = usize::from_str(&args[3])
        .expect("Error parsing sample count");

    Config {
        output_path,
        dimx: dimensions.0,
        dimy: dimensions.1,
        num_samples,
    }
}

fn get_pixel_color(ray: &Ray, hitable: &Hitable) -> Vec3<f64> {
    match hitable.try_hit(ray, 0.0, f64::MAX) {
        Some(hit) => {
            0.5 * (hit.normal + Vec3::<f64>::from(1.0, 1.0, 1.0))
        }
        None => {
            // background
            let unit_dir = ray.dir.normalize();
            let time = 0.5 * (unit_dir[1]) + 1.0;
            (1.0 - time) * Vec3::<f64>::from(1.0, 1.0, 1.0) + time * Vec3::<f64>::from(0.5, 0.7, 1.0)            
        }
    }
}

fn main() {
    let config = parse_args();

    let mut pixel_buf: Vec<u8> = vec![0; config.dimx * config.dimy * 3];

    let camera = Camera::new_default();

    let mut world = HitableList::new();
    world.list.push(Box::new(Sphere::new(Vec3::<f64>::from(0.0,0.0,-1.0), 0.5)));
    world.list.push(Box::new(Sphere::new(Vec3::<f64>::from(0.0,-100.5,-1.0), 100.0)));

    let mut rng = rand::thread_rng();

    let mut pixel_index = 0;
    for j in 0..config.dimy {
        for i in 0..config.dimx {
            let mut color = Vec3::<f64>::new();
            
            for _ in 0..config.num_samples {
                let u = (i as f64 + rng.gen::<f64>())/ config.dimx as f64;
                let v = ((config.dimy - j) as f64 + rng.gen::<f64>()) / config.dimy as f64;

                let ray = camera.get_ray_at_uv(u, v);
                color += get_pixel_color(&ray, &world);                
            }

            color /= config.num_samples as f64;
            
            let ir = (255.99 * color[0]) as u8;
            let ig = (255.99 * color[1]) as u8;
            let ib = (255.99 * color[2]) as u8;
            
            pixel_buf[pixel_index] = ir;
            pixel_index += 1;
            pixel_buf[pixel_index] = ig;
            pixel_index += 1;
            pixel_buf[pixel_index] = ib;
            pixel_index += 1;
        }
    }

    save_image(&pixel_buf, &config);
}
