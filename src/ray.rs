use crate::vec3::Vec3 as Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(org: Vec3, dir: Vec3)->Ray{
        Ray{
            orig: org,
            dir: dir,
            tm: 0.0,
        }
    }
    pub fn newt(org: Vec3, dir: Vec3, time: f64)->Ray{
        Ray{
            orig: org,
            dir: dir,
            tm: time,
        }
    }
    pub fn origin(&self)->Vec3{
        self.orig
    }
    pub fn direction(&self)->Vec3{
        self.dir
    }
    pub fn time(&self)->f64{
        self.tm
    }
    pub fn at(&self, t: f64)->Vec3{
        self.orig + t*self.dir
    }
}