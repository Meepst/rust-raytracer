use crate::vec3::Vec3 as Vec3;
use crate::onb::ONB as ONB;
use crate::hittable::Hittable as Hittable;

use std::sync::Arc;

pub trait PDF{
    fn value(&self, direction: Vec3)->f64;
    fn generate(&self)->Vec3;
}

pub struct CosinePDF{
    uvw: ONB,
}

pub struct HittablePDF{
    objects: Arc<dyn Hittable>,
    origin: Vec3,
}

pub struct MixturePDF{
    p: [Arc<dyn PDF>; 2],
}

pub struct SpherePDF{}

impl CosinePDF{
    pub fn new(n: Vec3)->Self{
        Self{
            uvw: ONB::new(n),
        }
    }
}

impl HittablePDF{
    pub fn new(objects: Arc<dyn Hittable>, origin: Vec3)->Self{
        Self{
            objects: objects,
            origin: origin,
        }
    }
}

impl MixturePDF{
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>)->Self{
        Self{
            p: [p0, p1],
        }
    }
}

impl SpherePDF{
    pub fn new()->Self{
        Self{}
    }
}


impl PDF for SpherePDF{
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

impl PDF for HittablePDF{
    fn value(&self, direction: Vec3)->f64{
        self.objects.pdf_value(self.origin, direction)
    }
    fn generate(&self)->Vec3{
        self.clone().objects.random(self.origin)
    }
}

impl PDF for MixturePDF{
    fn value(&self, direction: Vec3)->f64{
        0.5*self.p[0].value(direction)+0.5*self.p[1].value(direction)
    }
    fn generate(&self)->Vec3{
        if Vec3::random_double()<0.5{
            return self.p[0].generate()
        }
        self.p[1].generate()
    }
}