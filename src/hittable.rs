use crate::ray::Ray as Ray;
use crate::vec3::Vec3 as Vec3;
use crate::interval::Interval as Interval;
use crate::material::Material as Material;
use crate::material::Metal as Metal;

use std::sync::Arc;

pub trait Hittable: Send + Sync{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool;
}


pub struct Hit_record{
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl Hit_record{
    pub fn new(mat: Arc<dyn Material>)->Hit_record{
        Hit_record{
            p: Vec3::enew(),
            normal: Vec3::enew(),
            t: 0.0,
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

impl Clone for Hit_record{
    fn clone(&self) -> Self{
        Hit_record{
            p: self.p,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
            mat: Arc::clone(&self.mat),
        }
    }
}