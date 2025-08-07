use crate::vec3::Vec3 as Vec3;
use crate::interval::Interval as Interval;

fn linear_to_gamma(linear_component: f64)->f64{
    if linear_component > 0.0{
        return linear_component.sqrt()
    }
    0.0
}

pub fn write_color(pixel_color: Vec3)->String{
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity: Interval = Interval::new(0.000, 0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r))as i32 ;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;
    
    format!("{rbyte} {gbyte} {bbyte}\n")
}