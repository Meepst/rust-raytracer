use crate::ray::Ray as Ray;
use crate::vec3::Vec3 as Vec3;
use crate::interval::Interval as Interval;
use crate::material::Material as Material;
use crate::aabb::AABB as AABB;

use std::sync::Arc;

//#[derive(Copy)]
pub trait Hittable: Send + Sync{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool;
    fn bounding_box(&self)->AABB;
    fn pdf_value(&self, origin: Vec3, direction: Vec3)->f64{
        0.0
    }
    fn random(&self, origin: Vec3)->Vec3{
        Vec3::new(1.0,0.0,0.0)
    }
}

pub struct Hit_record{
    p: Vec3,
    pub normal: Vec3,
    t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

pub struct Translate{
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

pub struct RotateY{
    sin_theta: f64,
    cos_theta: f64,
    object: Arc<dyn Hittable>,
    bbox: AABB,
}

impl Hit_record{
    pub fn new(mat: Arc<dyn Material>)->Hit_record{
        Hit_record{
            p: Vec3::enew(),
            normal: Vec3::enew(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat: mat,
        }
    }
    pub fn p(&self)->Vec3{
        self.p
    }
    pub fn normal(&self)->Vec3{
        self.normal
    }
    pub fn t(&self)->f64{
        self.t
    }
    pub fn u(&self)->f64{
        self.u
    }
    pub fn v(&self)->f64{
        self.v
    }
    pub fn front_face(&self)->bool{
        self.front_face
    }
    pub fn setP(&mut self, p: Vec3){
        self.p = p
    }
    pub fn setT(&mut self, t: f64){
        self.t = t
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.front_face = r.direction().dot(*outward_normal) < 0.0;
        
        self.normal = if self.front_face{
            *outward_normal
        }else{
            -*outward_normal
        };
    }
}

impl Translate{
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3)->Translate{
        Self{
            object: object.clone(),
            offset: offset,
            bbox: object.bounding_box()+offset,
        }
    }
}

impl RotateY{
    pub fn new(object: Arc<dyn Hittable>, angle: f64)->Self{
        let mut ret = Self{
            sin_theta: 0.0,
            cos_theta: 0.0,
            object: object,
            bbox: AABB::newi(Vec3::enew(),Vec3::enew()),
        };
        let radians = (angle * std::f64::consts::PI)/180.0;
        ret.sin_theta = radians.sin();
        ret.cos_theta = radians.cos();
        ret.bbox = ret.object.bounding_box();

        let mut min = Vec3::new(std::f64::INFINITY,std::f64::INFINITY,std::f64::INFINITY);
        let mut max = Vec3::new(std::f64::NEG_INFINITY,std::f64::NEG_INFINITY,std::f64::NEG_INFINITY);

        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    let x = i as f64*ret.bbox.x().max+(1.0-i as f64)*ret.bbox.x().min;
                    let y = j as f64*ret.bbox.y().max+(1.0-j as f64)*ret.bbox.y().min;
                    let z = k as f64*ret.bbox.z().max+(1.0-k as f64)*ret.bbox.z().min;

                    let newx = ret.cos_theta*x+ret.sin_theta*z;
                    let newz = -ret.sin_theta*x + ret.cos_theta*z;

                    let tester = Vec3::new(newx,y,newz);

                    for c in 0..3{
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        ret.bbox = AABB::newi(min, max);
        ret
    }
}

impl Hittable for Translate{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool{
        let offset_r: Ray = Ray::newt(r.origin()-self.offset,r.direction(),r.time());
        if !self.object.hit(&offset_r, ray_t, rec){
            return false
        }

        rec.p += self.offset;

        true
    }
    fn bounding_box(&self)->AABB{
        self.bbox
    }
}

impl Hittable for RotateY{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool{
        let origin = Vec3::new((self.cos_theta*r.origin().x())-(self.sin_theta*r.origin().z()),
        r.origin().y(), (self.sin_theta*r.origin().x())+(self.cos_theta*r.origin().z()));
        let direction = Vec3::new((self.cos_theta*r.direction().x())-(self.sin_theta*r.direction().z()),
        r.direction().y(),(self.sin_theta*r.direction().x())+(self.cos_theta*r.direction().z()));

        let rotated_r = Ray::newt(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec){
            return false
        }

        rec.setP(Vec3::new((self.cos_theta*rec.p().x())+(self.sin_theta*rec.p().z()),
        rec.p().y(), (-self.sin_theta*rec.p().x())+(self.cos_theta*rec.p.z())));

        rec.normal = Vec3::new((self.cos_theta*rec.normal().x())+(self.sin_theta*rec.normal().z()),
                    rec.normal().y(), (-self.sin_theta*rec.normal().x())+(self.cos_theta*rec.normal().z()));

        true
    }
    fn bounding_box(&self)->AABB{
        self.bbox
    }
}

impl Clone for Hit_record{
    fn clone(&self) -> Self{
        Hit_record{
            p: self.p,
            normal: self.normal,
            t: self.t,
            u: self.u,
            v: self.v,
            front_face: self.front_face,
            mat: Arc::clone(&self.mat),
        }
    }
}