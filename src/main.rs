mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod material;

use std::sync::Arc;
use vec3::Vec3 as Vec3;
use color::write_color as write_color;
use ray::Ray as Ray;
use sphere::Sphere as Sphere;
use hittable::Hittable as Hittable;
use hittable::Hit_record as Hit_record;
use hittable_list::Hittable_List as Hittable_List;
use interval::Interval as Interval;
use camera::Camera as Camera;
use material::Lambertian as Lambertian;
use material::Material as Material;
use material::Metal as Metal;

// fn hit_sphere(center: Vec3, radius: f64, r: &Ray)->f64{
//     let oc: Vec3 = center-r.origin();
//     let a: f64 = r.direction().length_squared();
//     let h: f64 = Vec3::dot(&r.direction(), oc);
//     let c: f64 = oc.length_squared()-radius*radius;
//     let discriminant: f64 = h*h-a*c;
//     if discriminant < 0.0{
//         return -1.0
//     }
//     (h-discriminant.sqrt())/(a)
// }


// fn ray_color(r: &Ray, world: &dyn Hittable)->Vec3{
//     let mut rec: Hit_record = Hit_record::new();
//     if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec){
//         return 0.5*(rec.normal()+Vec3::new(1.0,1.0,1.0))
//     }
    
//     let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
//     let a: f64 = 0.5*(unit_direction.y()+1.0);
//     (1.0-a)*Vec3::new(1.0,1.0,1.0)+a*Vec3::new(0.5,0.7,1.0)
// }

fn main() {
    let mut world: Hittable_List = Hittable_List::new();

    let material_ground: Lambertian = Lambertian::new(Vec3::new(0.8,0.8,0.0));
    let material_center: Lambertian = Lambertian::new(Vec3::new(0.1,0.2,0.5));
    let material_left: Metal = Metal::new(Vec3::new(0.8,0.8,0.8), 0.3);
    let material_right: Metal = Metal::new(Vec3::new(0.8,0.6,0.2), 1.0);



    world.push(Arc::new(Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5, Arc::new(material_center))));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0, Arc::new(material_ground))));
    world.push(Arc::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0),0.5,Arc::new(material_left))));
    world.push(Arc::new(Sphere::new(Vec3::new(1.0,0.0,-1.0),0.5,Arc::new(material_right))));

    let mut cam: Camera = Camera::new(16.0/9.0, 400, 100, 50);
    cam.render(&world);
}
