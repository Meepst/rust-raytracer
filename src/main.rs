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
use material::Dielectric as Dielectric;

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

    let ground_material: Lambertian = Lambertian::new(Vec3::new(0.5,0.5,0.5));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,Arc::new(ground_material))));

    for a in -11..11{
        for b in -11..11{
            let choose_mat: f64 = Vec3::random_double();
            let center: Vec3 = Vec3::new(a as f64+0.9*Vec3::random_double(),0.2,b as f64+0.9*Vec3::random_double());
            if (center-Vec3::new(4.0,0.2,0.0)).length() > 0.9{
                if choose_mat < 0.8{
                    let albedo: Vec3 = Vec3::random()*Vec3::random();
                    let object_material: Lambertian = Lambertian::new(albedo);
                    world.push(Arc::new(Sphere::new(center, 0.2, Arc::new(object_material))));
                }else if choose_mat < 0.95{
                    let albedo: Vec3 = Vec3::random()*Vec3::random();
                    let fuzz: f64 = Vec3::random_between(0.0,0.5);
                    let object_material: Metal = Metal::new(albedo, fuzz);
                    world.push(Arc::new(Sphere::new(center, 0.2, Arc::new(object_material))));
                }else{
                    let object_material: Dielectric = Dielectric::new(1.5);
                    world.push(Arc::new(Sphere::new(center, 0.2, Arc::new(object_material))));
                }
            }
        }
    }

    let material_1: Dielectric = Dielectric::new(1.5);
    let material_2: Lambertian = Lambertian::new(Vec3::new(0.4,0.2,0.1));
    let material_3: Metal = Metal::new(Vec3::new(0.7,0.6,0.5),0.0);

    world.push(Arc::new(Sphere::new(Vec3::new(0.0,1.0,0.0),1.0,Arc::new(material_1))));
    world.push(Arc::new(Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0,Arc::new(material_2))));
    world.push(Arc::new(Sphere::new(Vec3::new(4.0,1.0,0.0),1.0,Arc::new(material_3))));

    let mut cam: Camera = Camera::new(16.0/9.0, 1200, 500, 50, 20.0, Vec3::new(13.0,2.0,3.0),
    Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,1.0,0.0),0.6,10.0);
    cam.render(&world);
}
