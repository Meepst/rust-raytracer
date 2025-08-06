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

impl Material for Lambertian{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray)->bool{
        let mut scatter_direction: Vec3 = rec.normal()+Vec3::random_unit_vector();
        
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal();
        }
       
        *scattered = Ray::new(rec.p(), scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &Hit_record, attenuation: &mut Vec3, scattered: &mut Ray)->bool{
        let reflected: Vec3 = Vec3::reflect(&r_in.direction(), &rec.normal());
        *scattered = Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        true
    }
}