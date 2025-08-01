mod vec3;
mod color;
mod ray;

use vec3::Vec3 as Vec3;
use color::write_color as write_color;
use ray::Ray as Ray;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray)->f64{
    let oc: Vec3 = center-r.origin();
    let a: f64 = Vec3::dot(&r.direction(), r.direction());
    let b: f64 = -2.0*Vec3::dot(&r.direction(),oc);
    let c: f64 = Vec3::dot(&oc,oc)-radius*radius;
    let discriminant: f64 = b*b-4.0*a*c;
    if discriminant < 0.0{
        return -1.0
    }
    (-b-discriminant.sqrt())/(2.0*a)
}

fn ray_color(r: &Ray)->Vec3{
    let t: f64 = hit_sphere(Vec3::new(0.0,0.0,-1.0),0.5,r);
    if t>0.0{
        let N: Vec3 = Vec3::unit_vector(&(r.at(t)-Vec3::new(0.0,0.0,-1.0)));
        return 0.5*Vec3::new(N.x()+1.0,N.y()+1.0,N.z()+1.0)
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let a: f64 = 0.5*(unit_direction.y()+1.0);
    (1.0-a)*Vec3::new(1.0,1.0,1.0)+a*Vec3::new(0.5,0.7,1.0)
}

fn main() {
    let image_width: u32 = 400;
    let aspect_ratio: f64 = 16.0/9.0;

    let image_height: u32 = image_width / aspect_ratio as u32;

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64/image_height as f64);
    let camera_center: Vec3 = Vec3::enew();

    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    let viewport_upper_left: Vec3 = camera_center-Vec3::new(0.0,0.0,focal_length)-viewport_u/2.0-viewport_v/2.0;
    let pixel00_loc: Vec3 = viewport_upper_left+0.5*(pixel_delta_u+pixel_delta_v);
    
    println!("P3\n{image_width} {image_height}\n255");

    for i in 0..image_height{
        eprint!("\rScanlines remaining: {} ", image_height - i);
        for j in 0..image_width{
            let pixel_center: Vec3 = pixel00_loc+(j as f64*pixel_delta_u)+(i as f64*pixel_delta_v);
            let ray_direction: Vec3 = pixel_center - camera_center;

            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Vec3 = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done.");
}
