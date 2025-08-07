use crate::hittable::Hittable as Hittable;
use crate::vec3::Vec3 as Vec3;
use crate::ray::Ray as Ray;
use crate::hittable::Hit_record as Hit_record;
use crate::interval::Interval as Interval;
use crate::color::write_color as write_color;
use crate::material::Lambertian as Lambertian;
use crate::material::Metal as Metal;
use crate::material::Material as Material;

use rand::Rng;
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Camera{
    aspect_ratio: f64,
    pixel_samples_scale: f64,
    vfov: f64,
    defocus_angle: f64,
    focus_dist: f64,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera{
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64,
    lookfrom: Vec3, lookat: Vec3, vup: Vec3, defocus_angle: f64, focus_dist: f64)->Camera{
        Camera{
            aspect_ratio: aspect_ratio,
            pixel_samples_scale: 0.0,
            vfov: vfov,
            defocus_angle: defocus_angle,
            focus_dist: focus_dist,
            image_width: image_width,
            image_height: 1,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            center: Vec3::enew(),
            pixel00_loc: Vec3::enew(),
            pixel_delta_u: Vec3::enew(),
            pixel_delta_v: Vec3::enew(),
            u: Vec3::enew(),
            v: Vec3::enew(),
            w: Vec3::enew(),
            lookfrom: lookfrom,
            lookat: lookat,
            vup: vup,
            defocus_disk_u: Vec3::enew(),
            defocus_disk_v: Vec3::enew(),
        }
    }
    fn degrees_to_radians(degrees: f64)->f64{
        let pi: f64 = 3.1415926535897932385;
        degrees*pi/180.0
    }
    fn initialize(&mut self){
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.center = self.lookfrom;

        self.pixel_samples_scale = 1.0/self.samples_per_pixel as f64;
        
        let theta: f64 = Self::degrees_to_radians(self.vfov);
        let h: f64 = (theta/2.0).tan();
        let viewport_height: f64 = 2.0*h*self.focus_dist;
        let viewport_width: f64 = viewport_height * (self.image_width / self.image_height) as f64;

        self.w = Vec3::unit_vector(&(self.lookfrom-self.lookat));
        self.u = Vec3::unit_vector(&self.vup.cross(self.w));
        self.v = self.w.cross(self.u);

        let viewport_u: Vec3 = viewport_width * self.u;
        let viewport_v: Vec3 = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left: Vec3 = self.center-(self.focus_dist*self.w)-viewport_u/2.0-viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left+0.5*(self.pixel_delta_u+self.pixel_delta_v);

        let defocus_radius: f64 = self.focus_dist * (Self::degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u*defocus_radius;
        self.defocus_disk_v = self.v*defocus_radius;
    } 
    fn ray_color(r: &Ray, depth: u32,world: &dyn Hittable)->Vec3{
        if depth <= 0{
            return Vec3::enew()
        }

        let dummy_mat = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let mut rec: Hit_record = Hit_record::new(dummy_mat);
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec){
            let mut scattered: Ray = Ray::new(Vec3::enew(),Vec3::enew());
            let mut attenuation: Vec3 = Vec3::enew();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered){
                return attenuation*Self::ray_color(&scattered, depth-1, world)
            }
            return Vec3::enew()
        }
        
        let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
        let a: f64 = 0.5*(unit_direction.y()+1.0);
        (1.0-a)*Vec3::new(1.0,1.0,1.0)+a*Vec3::new(0.5,0.7,1.0)
        // let dir: Vec3 = Vec3::unit_vector(&r.direction());
        // if dir.x()>=0.0 && dir.y()>=0.0{
        //     return Vec3::new(1.0,0.0,0.0)
        // }else if dir.x()>=0.0 && dir.y() < 0.0{
        //     return Vec3::new(0.0,1.0,0.0)
        // }else if dir.x() < 0.0 && dir.y() >= 0.0{
        //     return Vec3::new(0.0,0.0,1.0)
        // }else{
        //     return Vec3::new(1.0,1.0,0.0)
        // }
    }
    pub fn render(&mut self, world: &dyn Hittable){
        self.initialize();
        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);

        let mut pixels: Vec<String> = vec![String::new(); (self.image_width*self.image_height) as usize];

        pixels
            .par_chunks_mut(self.image_width as usize)
            .enumerate()
            .for_each(|(j,row)|{
                let mut local_camera: Camera = self.clone();
                for (i,pixel) in row.iter_mut().enumerate(){
                    let mut pixel_color: Vec3 = Vec3::enew();
                    for s in 0..local_camera.samples_per_pixel{
                        let r: Ray = local_camera.get_ray(i as u32, j as u32);
                        pixel_color += Camera::ray_color(&r, local_camera.max_depth, &*world);
                    }
                    let scaled: Vec3 = local_camera.pixel_samples_scale*pixel_color;
                    *pixel = write_color(scaled);
                }
            });

            for row in pixels.chunks(self.image_width as usize){
                for pixel in row{
                    print!("{}",pixel);
                }
            }

            eprintln!("Pixels rendered: {}", pixels.iter().filter(|s| !s.is_empty()).count());


        // for i in 0..self.image_height{
        //     eprint!("\rScanlines remaining: {} ", self.image_height - i);
        //     for j in 0..self.image_width{
        //         let mut pixel_color: Vec3 = Vec3::enew();
        //         for sample in 0..self.samples_per_pixel{
        //             let r: Ray = self.get_ray(j,i);
        //             pixel_color += Self::ray_color(&r, self.max_depth,world);
        //         }
                
        //         write_color(self.pixel_samples_scale*pixel_color);
        //     }
        // }
        eprintln!("Done.");
    }
    fn sample_square()->Vec3{
        Vec3::new(Self::random_double()-0.5,Self::random_double()-0.5,0.0)
    }
    fn get_ray(&self, i: u32, j: u32)->Ray{
        let offset: Vec3 = Self::sample_square();
        let pixel_sample: Vec3 = self.pixel00_loc +((i as f64 +offset.x())*self.pixel_delta_u)+((j as f64 +offset.y())*self.pixel_delta_v);
        let ray_origin: Vec3 = if self.defocus_angle <= 0.0{
            self.center 
        }else{
            self.defocus_disk_sample()
        };
        let ray_direction: Vec3 = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
    fn random_double()->f64{
        rand::thread_rng().gen_range(0.0..1.0)
    }
    fn defocus_disk_sample(&self)->Vec3{
        let p: Vec3 = Vec3::random_in_unit_disk();
        self.center+(p.x()*self.defocus_disk_u)+(p.y()*self.defocus_disk_v)
    }
}