mod vec3;

use crate::vec3::Vec3;

fn main() {
    const image_width: u32 = 256;
    const image_height: u32 = 256;
    
    println!("P3\n{image_width} {image_height}\n255");

    for i in 0..image_height{
        eprint!("\rScanlines remaining: {} ", image_height - i);
        for j in 0..image_width{
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }
    eprintln!("Done.");
}
