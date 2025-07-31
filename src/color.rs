mod vec3;

use crate::vec3::Vec3;

pub fn write_color(pixel_color: &Vec3){
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    let rbyte: i32 = (255.999 * r) as i32;
    let gbyte: i32 = (255.999 * g) as i32;
    let bbyte: i32 = (255.999 * b) as i32;

    println!("{rbyte} {gbyte} {bbyte}")
}