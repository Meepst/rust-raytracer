use crate::ray::Ray as Ray;
use crate::hittable::Hit_record as Hit_record;
use crate::vec3::Vec3 as Vec3;


pub trait Material: Send + Sync{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray)->bool{
        false
    }
}

pub struct Lambertian{
    albedo: Vec3,
}

pub struct Metal{
    albedo: Vec3,
    fuzz: f64,
}

pub struct Dielectric{
    refraction_index: f64,
}

impl Lambertian{
    pub fn new(albedo: Vec3)->Lambertian{
        Lambertian{
            albedo: albedo,
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

impl Material for Lambertian{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray)->bool{
        let mut scatter_direction: Vec3 = rec.normal()+Vec3::random_unit_vector();
        
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal();
        }
       
        *scattered = Ray::newt(rec.p(), scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray)->bool{
        let reflected: Vec3 = Vec3::reflect(&r_in.direction(), &rec.normal());
        *scattered = Ray::newt(rec.p(), reflected, r_in.time());
        *attenuation = self.albedo;
        true
    }
}

impl Material for Dielectric{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray)->bool{
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
}