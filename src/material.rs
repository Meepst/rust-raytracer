use crate::ray::Ray as Ray;
use crate::hittable::Hit_record as Hit_record;
use crate::vec3::Vec3 as Vec3;
use crate::texture::Solid_Color as Solid_Color;
use crate::texture::Checker_Texture as Checker_Texture;
use crate::texture::Texture as Texture;
use crate::onb::ONB as ONB;

use std::sync::Arc;



pub trait Material: Send + Sync{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray, pdf: &mut f64)->bool{
        false
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3;
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64;
}

pub struct Lambertian{
    tex: Arc<dyn Texture>,
}

pub struct Metal{
    albedo: Vec3,
    fuzz: f64,
}

pub struct Dielectric{
    refraction_index: f64,
}

pub struct Diffuse_Light{
    tex: Arc<dyn Texture>,
}

pub struct Isotropic{
    tex: Arc<dyn Texture>,
}

impl Lambertian{
    pub fn new(albedo: Vec3)->Lambertian{
        Lambertian{
            tex: Arc::new(Solid_Color::new(albedo)),
        }
    }
    pub fn newt(tex: Arc<dyn Texture>)->Lambertian{
        Lambertian{
            tex: tex,
        }
    }
}

impl Metal{
    pub fn new(albedo: Vec3, fuzz: f64)->Metal{
        Metal{
            albedo: albedo,
            fuzz: if fuzz > 1.0 {
                fuzz
            }else{
                1.0
            },
        }
    }
}

impl Dielectric{
    pub fn new(refraction_index: f64)->Dielectric{
        Dielectric{
            refraction_index: refraction_index,
        }
    }
    fn reflectance(cosine: f64, refraction_index: f64)->f64{
        let mut r0: f64 = (1.0-refraction_index)/(1.0+refraction_index);
        r0=r0*r0;
        r0+(1.0-r0)*f64::powi((1.0-cosine),5)
    }
}

impl Diffuse_Light{
    pub fn new(tex: Arc<dyn Texture>)->Self{
        Self{
            tex: tex,
        }
    }
    pub fn newc(emit: Vec3)->Self{
        Self{
            tex: Arc::new(Solid_Color::new(emit)),
        }
    }
}

impl Isotropic{
    pub fn new(tex: Arc<dyn Texture>)->Self{
        Self{tex}
    }
    pub fn newc(albedo: Vec3)->Self{
        Self{tex: Arc::new(Solid_Color::new(albedo)),}
    }
}

impl Material for Lambertian{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray, pdf: &mut f64)->bool{
        let uvw: ONB = ONB::new(rec.normal);
        let scatter_direction: Vec3 = uvw.transform(Vec3::random_cosine_direction());

        *scattered = Ray::newt(rec.p(), scatter_direction.unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u(),rec.v(), rec.p());
        //eprintln!("Atten: {} {} {}", attenuation.x(), attenuation.y(), attenuation.z());
        *pdf = Vec3::dot(&uvw.w(), scattered.direction()) / std::f64::consts::PI;
        true
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3{
        Vec3::enew()
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64{
        let cos_theta = Vec3::dot(&rec.normal, Vec3::unit_vector(&scattered.direction()));
        if cos_theta < 0.0{
            return 0.0
        }else{
            cos_theta / std::f64::consts::PI
        }
    }
}

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray, pdf: &mut f64)->bool{
        let reflected: Vec3 = Vec3::reflect(&r_in.direction(), &rec.normal());
        *scattered = Ray::newt(rec.p(), reflected, r_in.time());
        *attenuation = self.albedo;
        true
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3{
        Vec3::enew()
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64{
        0.0
    }
}

impl Material for Dielectric{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray, pdf: &mut f64)->bool{
        *attenuation = Vec3::new(1.0,1.0,1.0);
        let ri: f64 = if rec.front_face() {
            1.0/self.refraction_index
        }else{
            self.refraction_index
        };
        
        let unit_direction: Vec3 = Vec3::unit_vector(&r_in.direction());
        let cos_theta: f64 = f64::min(-unit_direction.dot(rec.normal()), 1.0);
        let sin_theta: f64 = (1.0-cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ri*sin_theta > 1.0;

        let direction: Vec3 = if cannot_refract || Self::reflectance(cos_theta, ri) > Vec3::random_double(){
            unit_direction.reflect(&rec.normal())
        }else{
            unit_direction.refract(rec.normal(), ri)
        };

        *scattered = Ray::newt(rec.p(), direction, r_in.time());
        true
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3{
        Vec3::enew()
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64{
        0.0
    }
}

impl Material for Diffuse_Light{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray, pdf: &mut f64)->bool{
        false
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3{
        if !rec.front_face{
            return Vec3::enew()
        }
        self.tex.value(u,v,p)
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64{
        0.0
    }
}

impl Material for Isotropic{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray, pdf: &mut f64)->bool{
        *scattered = Ray::newt(rec.p(), Vec3::random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u(), rec.v(), rec.p());
        *pdf = 1.0/(4.0*std::f64::consts::PI);
        true
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3{
        Vec3::enew()
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64{
        1.0/(4.0*std::f64::consts::PI)
    }
}