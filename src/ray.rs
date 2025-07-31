use crate::vec3::Vec3 as Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(org: Vec3, dir: Vec3)->Ray{
        Ray{
            orig: org,
            dir: dir,
        }
    }
    pub fn origin(&self)->Vec3{
        self.orig
    }
    pub fn direction(&self)->Vec3{
        self.dir
    }
    pub fn at(&self, t: f64)->Vec3{
        self.orig + t*self.dir
    }
}