use crate::hittable::Hittable as Hittable;
use crate::vec3::Vec3 as Vec3;
use crate::ray::Ray as Ray;
use crate::hittable::Hit_record as Hit_record;
use crate::interval::Interval as Interval;
use crate::color::write_color as write_color;
use crate::material::Lambertian as Lambertian;
use crate::material::Metal as Metal;
use crate::material::Material as Material;
use crate::pdf::CosinePDF as CosinePDF;
use crate::pdf::PDF as PDF;
use crate::pdf::HittablePDF as HittablePDF;
use crate::pdf::MixturePDF as MixturePDF;
use crate::material::ScatterRecord as ScatterRecord;

use rand::Rng;
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Camera{
    aspect_ratio: f64,
    pixel_samples_scale: f64,
    recip_sqrt_spp: f64,
    vfov: f64,
    defocus_angle: f64,
    focus_dist: f64,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    sqrt_spp: i32,
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
    background: Vec3,
}

impl Camera{
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64,
    lookfrom: Vec3, lookat: Vec3, vup: Vec3, defocus_angle: f64, focus_dist: f64, background: Vec3)->Camera{
        Camera{
            aspect_ratio: aspect_ratio,
            pixel_samples_scale: 0.0,
            recip_sqrt_spp: 0.0,
            vfov: vfov,
            defocus_angle: defocus_angle,
            focus_dist: focus_dist,
            image_width: image_width,
            image_height: 1,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            sqrt_spp: 1,
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
            background: background,
        }
    }
    fn degrees_to_radians(degrees: f64)->f64{
        let pi: f64 = 3.1415926535897932385;
        degrees*pi/180.0
    }
    fn initialize(&mut self){
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.center = self.lookfrom;
        
        self.sqrt_spp = u32::isqrt(self.samples_per_pixel) as i32;
        self.pixel_samples_scale = 1.0/(self.sqrt_spp*self.sqrt_spp) as f64;
        self.recip_sqrt_spp = 1.0/self.sqrt_spp as f64;
        
        let theta: f64 = Self::degrees_to_radians(self.vfov);
        let h: f64 = (theta/2.0).tan();
        let viewport_height: f64 = 2.0*h*self.focus_dist;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

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
    fn ray_color(&self,r: &Ray, depth: u32,world: &dyn Hittable, lights: Arc<dyn Hittable>)->Vec3{
        if depth <= 0{
            return Vec3::enew()
        }

        let dummy_mat = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let mut rec: Hit_record = Hit_record::new(dummy_mat);
        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec){
            return self.background;
        }

        let mut srec = ScatterRecord::new();
        let color_from_emission = rec.mat.clone().emitted(*r, rec.clone(),rec.u(),rec.v(),rec.p());

        if !rec.clone().mat.scatter(*r, rec.clone(), &mut srec){
            return color_from_emission
        }

        if srec.skip_pdf{
            return srec.attenuation*self.ray_color(&srec.skip_pdf_ray,depth-1,world,lights)
        }

        let light_ptr = Arc::new(HittablePDF::new(lights.clone(), rec.p()));
        let mixture_pdf = MixturePDF::new(light_ptr,srec.pdf_ptr.expect("REASON"));


        let scattered = Ray::newt(rec.p(),mixture_pdf.generate(),r.time());
        let pdf_value = mixture_pdf.value(scattered.direction());

        let scattering_pdf = rec.clone().mat.scattering_pdf(r,rec,scattered);

        let sample_color = self.ray_color(&scattered, depth-1, world, lights);
        let color_from_scatter = (srec.attenuation*scattering_pdf*sample_color)/pdf_value;
        
        color_from_emission + color_from_scatter
    }
    pub fn render(&mut self, world: &dyn Hittable, lights: Arc<dyn Hittable>){
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
                    for s_i in 0..self.sqrt_spp{
                        for s_j in 0..self.sqrt_spp{
                            let r: Ray = local_camera.get_ray(i as u32, j as u32, s_j as u32, s_i as u32);
                            pixel_color += local_camera.ray_color(&r, local_camera.max_depth, &*world, lights.clone());
                        }
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
    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32)->Ray{
        let offset: Vec3 = self.sample_square_stratified(s_i, s_j);
        let pixel_sample: Vec3 = self.pixel00_loc +((i as f64 +offset.x())*self.pixel_delta_u)+((j as f64 +offset.y())*self.pixel_delta_v);
        let ray_origin: Vec3 = if self.defocus_angle <= 0.0{
            self.center 
        }else{
            self.defocus_disk_sample()
        };
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        let ray_time: f64 = Vec3::random_double();

        Ray::newt(ray_origin, ray_direction, ray_time)
    }
    fn random_double()->f64{
        rand::thread_rng().gen_range(0.0..1.0)
    }
    fn defocus_disk_sample(&self)->Vec3{
        let p: Vec3 = Vec3::random_in_unit_disk();
        self.center+(p.x()*self.defocus_disk_u)+(p.y()*self.defocus_disk_v)
    }
    fn sample_square_stratified(&self, s_i: u32, s_j: u32)->Vec3{
        let px = ((s_i as f64 + Self::random_double())*self.recip_sqrt_spp)-0.5;
        let py = ((s_j as f64 + Self::random_double())*self.recip_sqrt_spp)-0.5;

        Vec3::new(px,py,0.0)
    }
}