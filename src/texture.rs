use crate::vec3::Vec3 as Vec3;

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
