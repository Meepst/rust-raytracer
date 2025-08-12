use crate::vec3::Vec3 as Vec3;
use crate::hittable::Hit_record as Hit_record;
use crate::hittable::Hittable as Hittable;
use crate::ray::Ray as Ray;
use crate::interval::Interval as Interval;
use crate::material::Material as Material;
use crate::aabb::AABB as AABB;
use std::sync::Arc;

pub struct Sphere{
    center: Ray,
    radius: f64,
    pub mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere{
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material>)->Sphere{
        let rvec: Vec3 = Vec3::new(radius,radius,radius);
        Sphere{
            center: Ray::new(center, Vec3::enew()),
            radius: f64::max(0.0,radius),
            mat: mat,
            bbox: AABB::newi(center-rvec, center+rvec),
        }
    }
    pub fn newt(center1: Vec3, center2: Vec3, radius: f64, mat: Arc<dyn Material>)->Sphere{
        let rvec: Vec3 = Vec3::new(radius,radius,radius);
        let tempcenter: Ray = Ray::new(center1, center2-center1);
        let box1: AABB = AABB::newi(tempcenter.at(0.0)-rvec,tempcenter.at(0.0)+rvec);
        let box2: AABB = AABB::newi(tempcenter.at(1.0)-rvec,tempcenter.at(1.0)+rvec);
        Sphere{
            center: tempcenter,
            radius: f64::max(0.0,radius),
            mat: mat,
            bbox: AABB::newb(box1, box2),
        }
    }
    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64){
        let theta = -p.y().acos();
        let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;

        *u = phi / (2.0*std::f64::consts::PI);
        *v = theta/std::f64::consts::PI;
    }
    pub fn bounding_box(&self)->AABB{
        self.bbox
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool{
        let current_center: Vec3 = self.center.at(r.time());
        let oc: Vec3 = current_center-r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = Vec3::dot(&r.direction(), oc);
        let c: f64 = oc.length_squared()-self.radius*self.radius;

        let discriminant: f64 = h*h-a*c;
        if discriminant < 0.0{
            return false
        }

        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (h-sqrtd)/a;
        if !ray_t.surrounds(root){
            root = (h+sqrtd) / a;
            if !ray_t.surrounds(root){
                return false
            }
        }

       
        rec.setT(root);
        rec.setP(r.at(rec.t()));
        let outward_normal: Vec3 = (rec.p() - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Self::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();
        
        return true
    }
    fn bounding_box(&self)->AABB{
        self.bbox
    }
}