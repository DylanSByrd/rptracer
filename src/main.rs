extern crate image;

use std::io::Write;
use std::str::FromStr;

#[derive(Debug)]
struct Config {
    output_path: String,
    dimx: usize,
    dimy: usize,
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

    if args.len() != 3 {
        writeln!(std::io::stderr(), "Usage: rptracer FILE DIMENSIONX,DIMENSIONY")
            .unwrap();
        writeln!(std::io::stderr(), "Example: {} image.png 800,600", args[0])
            .unwrap();
        std::process::exit(1);
    }

    let output_path = args[1].to_string();
    let dimensions = parse_pair(&args[2], ',')
        .expect("Error parsing dimensions; format is DIMENSIONX,DIMENSIONY");

    Config {
        output_path,
        dimx: dimensions.0,
        dimy: dimensions.1,
    }
}

fn main() {
    let config = parse_args();

    let mut pixel_buf: Vec<u8> = vec![0; config.dimx * config.dimy * 3];

    let mut pixel_index = 0;
    for j in 0..config.dimy {
        for i in 0..config.dimx {
            let fr: f64 = i as f64 / config.dimx as f64;
            let fg: f64 = (config.dimy - j) as f64 / config.dimy as f64;
            let fb: f64 = 0.2;
            
            let ir = (255.99 * fr) as u8;
            let ig = (255.99 * fg) as u8;
            let ib = (255.99 * fb) as u8;
            
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
