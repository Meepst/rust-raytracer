mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod material;
mod bvh;
mod aabb;
mod texture;
mod image_tex;
mod perlin;
mod quad;
mod constant_medium;
mod onb;

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
use bvh::BVH as BVH;
use texture::Checker_Texture as Checker_Texture;
use texture::Solid_Color as Solid_Color;
use texture::Image_Texture as Image_Texture;
use texture::Noise_Texture as Noise_Texture;
use quad::Quad as Quad;
use material::Diffuse_Light as Diffuse_Light;
use hittable::RotateY as RotateY;
use hittable::Translate as Translate;
use quad::Cube as Cube;
use constant_medium::ConstantMedium as ConstantMedium;

use image::io::Reader as ImageReader;
use image::{ImageFormat, ImageError};
use std::path::Path;
use std::fs;
use std::io::Cursor;

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

fn bouncing_spheres(){
    let mut world: Hittable_List = Hittable_List::new();

    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5,0.5,0.5)));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,ground_material)));

    for a in -11..11{
        for b in -11..11{
            let choose_mat: f64 = Vec3::random_double();
            let center: Vec3 = Vec3::new(a as f64+0.9*Vec3::random_double(),0.2,b as f64+0.9*Vec3::random_double());
            if (center-Vec3::new(4.0,0.2,0.0)).length() > 0.9{
                if choose_mat < 0.8{
                    let albedo: Vec3 = Vec3::random()*Vec3::random();
                    let object_material: Lambertian = Lambertian::new(albedo);
                    let center2: Vec3 = center+Vec3::new(0.0,Vec3::random_between(0.0,0.5),0.0);
                    world.push(Arc::new(Sphere::newt(center,center2, 0.2, Arc::new(object_material))));
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
    
    let mut objects = world.objects().clone();
    let bvh_root = Arc::new(BVH::new(objects));
    let mut world = Hittable_List::new();
    world.push(bvh_root);

    let mut cam: Camera = Camera::new(16.0/9.0, 800, 100, 50, 30.0, Vec3::new(13.0,2.0,3.0),
    Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,1.0,0.0),0.6,10.0,Vec3::new(0.7,0.8,1.0));
    cam.render(&world);
}

fn checkered_spheres(){
    let mut world: Hittable_List = Hittable_List::new();

    let checker = Arc::new(Checker_Texture::news(0.32, Vec3::new(0.2,0.3,0.1),Vec3::new(0.9,0.9,0.9)));

    world.push(Arc::new(Sphere::new(Vec3::new(0.0,-10.0,0.0),10.0,Arc::new(Lambertian::newt(checker.clone())))));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,10.0,0.0),10.0,Arc::new(Lambertian::newt(checker)))));

    let mut objects = world.objects().clone();
    let bvh_root = Arc::new(BVH::new(objects));
    let mut world = Hittable_List::new();
    world.push(bvh_root);

    let mut cam: Camera = Camera::new(16.0/9.0, 400, 100, 50, 20.0, Vec3::new(13.0,2.0,3.0),
    Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,1.0,0.0),0.6,10.0,Vec3::new(0.7,0.8,1.0));
    cam.render(&world);
}

fn earth(){
    let mut world: Hittable_List = Hittable_List::new();
    let earth_texture = Arc::new(Image_Texture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::newt(earth_texture));
    let globe = Arc::new(Sphere::new(Vec3::enew(), 2.0, earth_surface));

    world.push(globe);

    let mut cam: Camera = Camera::new(16.0/9.0,400,100,50,20.0,Vec3::new(0.0,0.0,12.0),
    Vec3::enew(), Vec3::new(0.0,1.0,0.0), 0.0, 10.0,Vec3::new(0.7,0.8,1.0));

    cam.render(&world);
}

fn perlin_spheres(){
    let mut world: Hittable_List = Hittable_List::new();

    let perlin_texture = Arc::new(Noise_Texture::new(4.0));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,Arc::new(Lambertian::newt(perlin_texture.clone())))));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,2.0,0.0),2.0,Arc::new(Lambertian::newt(perlin_texture)))));

    let mut cam: Camera = Camera::new(16.0/9.0,400,500,50,20.0,Vec3::new(13.0,2.0,3.0),
    Vec3::enew(), Vec3::new(0.0,1.0,0.0), 0.0, 10.0,Vec3::new(0.7,0.8,1.0));

    cam.render(&world);
}

fn quads(){
    let mut world: Hittable_List = Hittable_List::new();

    let left_red = Arc::new(Lambertian::new(Vec3::new(1.0,0.2,0.2)));
    let back_green = Arc::new(Lambertian::new(Vec3::new(0.2,1.0,0.2)));
    let right_blue = Arc::new(Lambertian::new(Vec3::new(0.2,0.2,1.0)));
    let upper_orange = Arc::new(Lambertian::new(Vec3::new(1.0,0.5,0.0)));
    let lower_teal = Arc::new(Lambertian::new(Vec3::new(0.2,0.8,0.8)));

    world.push(Arc::new(Quad::new(Vec3::new(-3.0,-2.0,5.0),Vec3::new(0.0,0.0,-4.0),Vec3::new(0.0,4.0,0.0),left_red)));
    world.push(Arc::new(Quad::new(Vec3::new(-2.0,-2.0,0.0),Vec3::new(4.0,0.0,0.0),Vec3::new(0.0,4.0,0.0),back_green)));
    world.push(Arc::new(Quad::new(Vec3::new(3.0,-2.0,1.0),Vec3::new(0.0,0.0,4.0),Vec3::new(0.0,4.0,0.0),right_blue)));
    world.push(Arc::new(Quad::new(Vec3::new(-2.0,3.0,1.0),Vec3::new(4.0,0.0,0.0),Vec3::new(0.0,0.0,4.0),upper_orange)));
    world.push(Arc::new(Quad::new(Vec3::new(-2.0,-3.0,5.0),Vec3::new(4.0,0.0,0.0),Vec3::new(0.0,0.0,-4.0),lower_teal)));

    
    let mut cam: Camera = Camera::new(1.0,400,100,50,80.0,Vec3::new(0.0,0.0,9.0),
    Vec3::enew(), Vec3::new(0.0,1.0,0.0), 0.0, 10.0, Vec3::new(0.7,0.8,1.0));

    cam.render(&world);
}

fn simple_light(){
    let mut world: Hittable_List = Hittable_List::new();

    let perl_tex = Arc::new(Noise_Texture::new(4.0));
    let diff_light = Arc::new(Diffuse_Light::newc(Vec3::new(4.0,4.0,4.0)));

    world.push(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,Arc::new(Lambertian::newt(perl_tex.clone())))));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,2.0,0.0),2.0,Arc::new(Lambertian::newt(perl_tex)))));
    world.push(Arc::new(Quad::new(Vec3::new(3.0,1.0,-2.0),Vec3::new(2.0,0.0,0.0),Vec3::new(0.0,2.0,0.0),diff_light)));

    let mut cam: Camera = Camera::new(16.0/9.0,1024,1000,100,20.0,Vec3::new(26.0,3.0,6.0),
    Vec3::new(0.0,2.0,0.0), Vec3::new(0.0,1.0,0.0), 0.0, 10.0, Vec3::enew());

    cam.render(&world);
}

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

    let mut box1: Arc<dyn Hittable> = Cube::new(Vec3::enew(),Vec3::new(165.0,330.0,165.0),white.clone());
    // box1 = Arc::new(RotateY::new(box1, 15.0));
    // box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    world.push(box1);
    

    let mut box2: Arc<dyn Hittable>  = Cube::new(Vec3::enew(),Vec3::new(165.0,165.0,165.0), white);
   
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    world.push(box2);

    let mut cam: Camera = Camera::new(1.0,600,1000,50,40.0,Vec3::new(278.0,278.0,-800.0),
    Vec3::new(278.0,278.0,0.0), Vec3::new(0.0,1.0,0.0), 0.0, 10.0, Vec3::enew());

    cam.render(&world);
}

fn cornell_smoke(){
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

    let mut box1: Arc<dyn Hittable> = Cube::new(Vec3::enew(),Vec3::new(165.0,330.0,165.0),white.clone());
    // box1 = Arc::new(RotateY::new(box1, 15.0));
    // box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    box1 = Arc::new(ConstantMedium::newc(box1, 0.01, Vec3::enew()));
    world.push(box1);
    

    let mut box2: Arc<dyn Hittable>  = Cube::new(Vec3::enew(),Vec3::new(165.0,165.0,165.0), white);
   
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    box2 = Arc::new(ConstantMedium::newc(box2, 0.01, Vec3::new(1.0,1.0,1.0)));
    world.push(box2);

    let mut cam: Camera = Camera::new(1.0,600,200,50,40.0,Vec3::new(278.0,278.0,-800.0),
    Vec3::new(278.0,278.0,0.0), Vec3::new(0.0,1.0,0.0), 0.0, 10.0, Vec3::enew());

    cam.render(&world);
}

fn final_scene(){
    let mut boxes1: Hittable_List = Hittable_List::new();
    let ground = Arc::new(Lambertian::new(Vec3::new(0.48,0.83,0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side{
        for j in 0..boxes_per_side{
            let w = 100.0;
            let x0 = -1000.0+(i as f64 * w);
            let z0 = -1000.0+(j as f64 * w);
            let y0 = 0.0;
            let x1 = x0+w;
            let y1 = Vec3::random_between(1.0,101.0);
            let z1 = z0+w;

            boxes1.push(Cube::new(Vec3::new(x0,y0,z0),Vec3::new(x1,y1,z1),ground.clone()));
        }
    }
    
    let mut world = Hittable_List::new();
    world.push(Arc::new(BVH::new(boxes1.objects)));

    let light = Arc::new(Diffuse_Light::newc(Vec3::new(7.0,7.0,7.0)));
    world.push(Arc::new(Quad::new(Vec3::new(123.0,554.0,147.0),Vec3::new(300.0,0.0,0.0),Vec3::new(0.0,0.0,265.0),light)));

    let center1 = Vec3::new(400.0,400.0,200.0);
    let center2 = center1+Vec3::new(30.0,0.0,0.0);
    let sphere_mat = Arc::new(Lambertian::new(Vec3::new(0.7,0.3,0.1)));
    world.push(Arc::new(Sphere::newt(center1, center2, 50.0, sphere_mat)));

    world.push(Arc::new(Sphere::new(Vec3::new(260.0,150.0,45.0), 50.0,Arc::new(Dielectric::new(1.5)))));
    world.push(Arc::new(Sphere::new(Vec3::new(0.0,150.0,145.0),50.0,Arc::new(Metal::new(Vec3::new(0.8,0.8,0.9), 1.0)))));

    let boundary = Arc::new(Sphere::new(Vec3::new(360.0,150.0,145.0),70.0,Arc::new(Dielectric::new(1.5))));
    world.push(boundary.clone());
    world.push(Arc::new(ConstantMedium::newc(boundary.clone(), 0.2, Vec3::new(0.2,0.4,0.0))));

    let boundary = Arc::new(Sphere::new(Vec3::enew(),5000.0,Arc::new(Dielectric::new(1.5))));
    world.push(Arc::new(ConstantMedium::newc(boundary, 0.0001, Vec3::new(1.0,1.0,1.0))));

    let earth_mat = Arc::new(Lambertian::newt(Arc::new(Image_Texture::new("earthmap.jpg"))));
    world.push(Arc::new(Sphere::new(Vec3::new(400.0,200.0,400.0),100.0,earth_mat)));

    let perlin_mat = Arc::new(Noise_Texture::new(0.2));
    world.push(Arc::new(Sphere::new(Vec3::new(220.0,280.0,300.0), 80.0, Arc::new(Lambertian::newt(perlin_mat)))));

    let mut boxes2: Hittable_List = Hittable_List::new();
    let white = Arc::new(Lambertian::new(Vec3::new(0.73,0.73,0.73)));
    let ns = 1000;
    for k in 0..ns{
        boxes2.push(Arc::new(Sphere::new(Vec3::random_vars(0.0,165.0),10.0,white.clone())));
    }

    world.push(Arc::new(Translate::new(Arc::new(RotateY::new(Arc::new(BVH::new(boxes2.objects)), 15.0)), Vec3::new(-100.0,270.0,395.0))));

    let mut cam: Camera = Camera::new(1.0,1080,10000,40,40.0,Vec3::new(478.0,278.0,-600.0),
    Vec3::new(278.0,278.0,0.0), Vec3::new(0.0,1.0,0.0), 0.0, 10.0, Vec3::enew());

    cam.render(&world);
}

fn ppm_to_png(ppm_path: &Path, png_path: &Path) -> Result<(), ImageError> {
       let bytes = fs::read(ppm_path).expect("Failed to read file");

    // Detect UTF-16LE BOM (0xFF 0xFE) or implicit UTF-16LE (P\0 3\0)
    let is_utf16le = bytes.starts_with(&[0xFF, 0xFE]) || bytes.get(1) == Some(&0x00);

    // Convert UTF-16LE to plain UTF-8 string
    let ascii_bytes = if is_utf16le {
        let utf16: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        let content = String::from_utf16(&utf16).expect("Bad UTF-16LE");
        content.into_bytes() // now it's valid ASCII-like bytes
    } else {
        bytes
    };

    // Now feed valid PNM bytes to image crate
    let img = ImageReader::with_format(Cursor::new(ascii_bytes), ImageFormat::Pnm)
        .decode()?;

    img.save(png_path)?;
    Ok(())
}

fn main() {
    cornell_box();
}
