use crate::hittable::Hittable as Hittable;
use crate::material::Material as Material;
use crate::aabb::AABB as AABB;
use crate::vec3::Vec3 as Vec3;
use crate::texture::Texture as Texture;
use crate::material::Isotropic as Isotropic;
use crate::hittable::Hit_record as Hit_record;
use crate::ray::Ray as Ray;
use crate::interval::Interval as Interval;

use std::sync::Arc;

pub struct ConstantMedium{
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}


impl ConstantMedium{
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>)->Self{
        Self{
            boundary: boundary,
            neg_inv_density: -1.0/density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }
    pub fn newc(boundary: Arc<dyn Hittable>, density: f64, albedo: Vec3)->Self{
        Self{
            boundary: boundary,
            neg_inv_density: -1.0/density,
            phase_function: Arc::new(Isotropic::newc(albedo)),
        }
    }
}

impl Hittable for ConstantMedium{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool{
        let mut rec1 = Hit_record::new(self.phase_function.clone());
        let mut rec2 = Hit_record::new(self.phase_function.clone());

        if !self.boundary.hit(r, Interval::universe(), &mut rec1){
            return false
        }
        //eprintln!("{}", rec1.t());
        if !self.boundary.hit(r, Interval::new(rec1.t()+0.0001, f64::INFINITY), &mut rec2){
            //eprintln!("dog!");
            return false
        }

        if rec1.t() < ray_t.min(){
            rec1.setT(ray_t.min());
        }
        if rec2.t() > ray_t.max(){
            rec2.setT(ray_t.max());
        }

        //eprintln!("mmmmm");
        if rec1.t() >= rec2.t(){
            return false 
        }
        if rec1.t() < 0.0{
            rec1.setT(0.0);
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t()-rec1.t())*ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(Vec3::random_double());

        if hit_distance > distance_inside_boundary{
            return false
        }

        rec.setT(rec1.t()+hit_distance / ray_length);
        rec.setP(r.at(rec.t()));

        rec.normal = Vec3::new(1.0,0.0,0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();
        //eprintln!("DOOM!");
        true
    }
    fn bounding_box(&self)->AABB{
        self.boundary.bounding_box()
    }
}