use crate::ray::Ray as Ray;
use crate::vec3::Vec3 as Vec3;

pub trait Hittable: Send + Sync{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hit_record)->bool;
}

#[derive(Copy, Clone)]
pub struct Hit_record{
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl Hit_record{
    pub fn new()->Hit_record{
        Hit_record{
            p: Vec3::enew(),
            normal: Vec3::enew(),
            t: 0.0,
            front_face: false,
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
