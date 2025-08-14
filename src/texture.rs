use crate::vec3::Vec3 as Vec3;
use crate::image_tex::RtwImage as RtwImage;
use crate::Interval;
use crate::perlin::Perlin as Perlin;

use std::sync::Arc;

pub trait Texture: Send + Sync{
    fn value(&self, u: f64,v: f64, p: Vec3)->Vec3;
}

pub struct Solid_Color{
    albedo: Vec3,
}

pub struct Checker_Texture{
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

pub struct Image_Texture{
    image: RtwImage,
}

#[derive(Clone)]
pub struct Noise_Texture{
    noise: Perlin,
    scale: f64,
}

impl Solid_Color{
    pub fn new(albedo: Vec3)->Self{
        Self{
            albedo: albedo,
        }
    }
    pub fn newi(red: f64, green: f64, blue: f64)->Self{
        Self{
            albedo: Vec3::new(red, green, blue),
        }
    }
}

impl Checker_Texture{
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>)->Self{
        Self{
            inv_scale: 1.0/scale,
            even: even,
            odd: odd,
        }
    }
    pub fn news(scale: f64, c1: Vec3, c2: Vec3)->Self{
        Self{
            inv_scale: 1.0/scale,
            even: Arc::new(Solid_Color::new(c1)),
            odd: Arc::new(Solid_Color::new(c2)),
        }
    }
}

impl Image_Texture{
    pub fn new(filename: &str)->Self{
        Self{
            image: RtwImage::from_file(filename),
        }
    }
}

impl Noise_Texture{
    pub fn new(scale: f64)->Self{
        Noise_Texture{
            noise: Perlin::new(),
            scale: scale,
        }
    }
}

impl Texture for Solid_Color{
    fn value(&self, u: f64, v: f64, p: Vec3)->Vec3{
        self.albedo
    }
}

impl Texture for Checker_Texture{
    fn value(&self, u: f64, v: f64, p: Vec3)->Vec3{
        let xInt: i32 = ((self.inv_scale*p.x()).floor()) as i32;
        let yInt: i32 = ((self.inv_scale*p.y()).floor()) as i32;
        let zInt: i32 = ((self.inv_scale*p.z()).floor()) as i32;

        let isEven: bool = (xInt + yInt + zInt) % 2 == 0;

        if isEven{
            return self.even.value(u, v, p)
        }

        self.odd.value(u, v, p)
    }
}

impl Texture for Image_Texture{
    fn value(&self, mut u: f64, mut v: f64, p: Vec3)->Vec3{
        if self.image.height() <= 0 {
            return Vec3::new(0.0,1.0,1.0)
        }
        //eprintln!("PREV u: {}, PREV v: {}", u, v);
        let u = Interval::new(0.0,1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0,1.0).clamp(v.abs());
        //eprintln!("u: {}, v: {}", u, v);
        let i: usize = (u*self.image.width() as f64) as usize;
        let j: usize = (v*self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i,j);
        
        let cscale = 1.0/255.0;
        Vec3::new(cscale*pixel[0] as f64, cscale*pixel[1] as f64, cscale*pixel[2] as f64)
    }
}

impl Texture for Noise_Texture{
    fn value(&self, mut u: f64, mut v: f64, p: Vec3)->Vec3{
        Vec3::new(0.5,0.5,0.5) * (1.0 + (self.scale * p.z() + 10.0 *self.noise.turb(p, 7)).sin())
    }
}

fn rgb_to_linear(c: u8)->f64{
    let x = c as f64 / 255.0;
    if x<=0.04045{
       return x / 12.92
    }
    ((x+0.055)/1.055).powf(2.4)
}