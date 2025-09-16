use crate::ray::Ray as Ray;
use crate::hittable::Hit_record as Hit_record;
use crate::vec3::Vec3 as Vec3;
use crate::texture::Solid_Color as Solid_Color;
use crate::texture::Texture as Texture;
use crate::pdf::PDF as PDF;
use crate::pdf::SpherePDF as SpherePDF;
use crate::pdf::CosinePDF as CosinePDF;

use std::sync::Arc;



pub trait Material: Send + Sync{
    fn scatter(&self, r_in: Ray, rec: Hit_record, srec: &mut ScatterRecord)->bool{
        false
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3{
        Vec3::enew()
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64{
        0.0
    }
}

pub struct ScatterRecord{
    pub attenuation: Vec3,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
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

pub struct EmptyMat{

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
        r0+(1.0-r0)*f64::powi(1.0-cosine,5)
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

impl EmptyMat{
    pub fn new()->Self{
        Self{}
    }
}

impl ScatterRecord{
    pub fn new()->Self{
        Self{
            attenuation: Vec3::enew(),
            pdf_ptr: None,
            skip_pdf: false,
            skip_pdf_ray: Ray::new(Vec3::enew(),Vec3::enew()),
        }
    }
}

impl Material for Lambertian{
    fn scatter(&self, r_in: Ray, rec: Hit_record, srec: &mut ScatterRecord)->bool{
        srec.attenuation = self.tex.value(rec.u(),rec.v(),rec.p());
        srec.pdf_ptr = Some(Arc::new(CosinePDF::new(rec.normal)));
        srec.skip_pdf = false;

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
    fn scatter(&self, r_in: Ray, rec: Hit_record, srec: &mut ScatterRecord)->bool{
        let mut reflected = r_in.direction().reflect(&rec.normal());
        reflected = reflected.unit_vector() + (self.fuzz*Vec3::random_unit_vector());

        srec.attenuation = self.albedo;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::newt(rec.p(),reflected,r_in.time());

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
    fn scatter(&self, r_in: Ray, rec: Hit_record, srec: &mut ScatterRecord)->bool{
        srec.attenuation = Vec3::new(1.0,1.0,1.0);
        srec.skip_pdf = true;
        let ri = if rec.front_face{
            1.0/self.refraction_index
        }else{
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(rec.normal()), 1.0);
        let sin_theta = (1.0-cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ri*sin_theta > 1.0;
        let mut direction = Vec3::enew();

        if cannot_refract || Self::reflectance(cos_theta,ri) > Vec3::random_double(){
            direction = Vec3::reflect(&unit_direction, &rec.normal());
        }else{
            direction = Vec3::refract(&unit_direction, rec.normal(),ri)
        }

        srec.skip_pdf_ray = Ray::newt(rec.p(),direction,r_in.time());
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
    fn scatter(&self, r_in: Ray, rec: Hit_record, srec: &mut ScatterRecord)->bool{
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
    fn scatter(&self, r_in: Ray, rec: Hit_record, srec: &mut ScatterRecord)->bool{
        srec.attenuation = self.tex.value(rec.u(),rec.v(),rec.p());
        srec.pdf_ptr = Some(Arc::new(SpherePDF::new()));
        srec.skip_pdf = false;
        true
    }
    fn emitted(&self, r_in: Ray, rec: Hit_record, u: f64, v: f64, p: Vec3)->Vec3{
        Vec3::enew()
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: Hit_record, scattered: Ray)->f64{
        1.0/(4.0*std::f64::consts::PI)
    }
}

impl Material for EmptyMat{
}