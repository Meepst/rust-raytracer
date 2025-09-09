use crate::vec3::Vec3 as Vec3;
use crate::sphere::Sphere as Sphere;
use crate::onb::ONB as ONB;

pub trait PDF{
    fn value(&self, direction: Vec3)->f64;
    fn generate(&self)->Vec3;
}

pub struct CosinePDF{
    uvw: ONB,
}

impl CosinePDF{
    pub fn new(n: Vec3)->Self{
        Self{
            uvw: ONB::new(n),
        }
    }
}

impl PDF for Sphere{
    fn value(&self, direction: Vec3)->f64{
        1.0/(4.0*std::f64::consts::PI)
    }
    fn generate(&self)->Vec3{
        Vec3::random_unit_vector()
    }
}

impl PDF for CosinePDF{
     fn value(&self, direction: Vec3)->f64{
        let cosine_theta = Vec3::dot(&Vec3::unit_vector(&direction),self.uvw.w());
        f64::max(0.0,cosine_theta/std::f64::consts::PI)
    }
    fn generate(&self)->Vec3{
        self.uvw.transform(Vec3::random_cosine_direction())
    }
}