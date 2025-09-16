pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;
pub mod interval;
pub mod camera;
pub mod material;
pub mod bvh;
pub mod aabb;
pub mod texture;
pub mod image_tex;
pub mod perlin;
pub mod quad;
pub mod constant_medium;
pub mod onb;
pub mod pdf;

use std::sync::Arc;
use vec3::Vec3 as Vec3;
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
use quad::Quad as Quad;
use material::Diffuse_Light as Diffuse_Light;
use hittable::RotateY as RotateY;
use hittable::Translate as Translate;
use quad::Cube as Cube;
use material::EmptyMat as EmptyMat;

fn cornell_box(){
    let mut world: Hittable_List = Hittable_List::new();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65,0.05,0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73,0.73,0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12,0.45,0.15)));
    let light = Arc::new(Diffuse_Light::newc(Vec3::new(15.0,15.0,15.0)));

    world.push(Arc::new(Quad::new(Vec3::new(555.0,0.0,0.0), Vec3::new(0.0,555.0,0.0),   Vec3::new(0.0,0.0,555.0), green)));
    world.push(Arc::new(Quad::new(Vec3::new(0.0,0.0,0.0),   Vec3::new(0.0,555.0,0.0),   Vec3::new(0.0,0.0,555.0), red)));
    world.push(Arc::new(Quad::new(Vec3::new(343.0,554.0,332.0), Vec3::new(-130.0,0.0,0.0),   Vec3::new(0.0,0.0,-105.0), light)));
    world.push(Arc::new(Quad::new(Vec3::new(0.0,0.0,0.0),   Vec3::new(555.0,0.0,0.0),   Vec3::new(0.0,0.0,555.0), white.clone())));
    world.push(Arc::new(Quad::new(Vec3::new(555.0,555.0,555.0), Vec3::new(-555.0,0.0,0.0),   Vec3::new(0.0,0.0,-555.0), white.clone())));
    world.push(Arc::new(Quad::new(Vec3::new(0.0,0.0,555.0), Vec3::new(555.0,0.0,0.0),   Vec3::new(0.0,555.0,0.0), white.clone())));

    let aluminum = Arc::new(Metal::new(Vec3::new(0.8,0.85,0.88),0.0));
    let mut box1: Arc<dyn Hittable> = Cube::new(Vec3::enew(),Vec3::new(165.0,330.0,165.0),aluminum);
    // box1 = Arc::new(RotateY::new(box1, 15.0));
    // box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    world.push(box1);
    

    let glass = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(Vec3::new(190.0,90.0,190.0),90.0,glass)));

    let empty_material: Arc<dyn Material> = Arc::new(EmptyMat::new());
    let mut lights = Hittable_List::new();
    lights.push(Arc::new(Quad::new(Vec3::new(343.0,554.0,332.0),Vec3::new(-130.0,0.0,0.0),Vec3::new(0.0,0.0,-105.0),empty_material.clone())));
    lights.push(Arc::new(Sphere::new(Vec3::new(190.0,90.0,190.0),90.0,empty_material)));

    let mut cam: Camera = Camera::new(1.0,600,10000,50,40.0,Vec3::new(278.0,278.0,-800.0),
    Vec3::new(278.0,278.0,0.0), Vec3::new(0.0,1.0,0.0), 0.0, 10.0, Vec3::enew());

    cam.render(&world, Arc::new(lights));
}

fn main() {
    cornell_box();
}
