use crate::vec3::Vec3 as Vec3;
use crate::hittable::Hit_record as Hit_record;
use crate::hittable::Hittable as Hittable;
use crate::ray::Ray as Ray;
use crate::interval::Interval as Interval;
use crate::material::Material as Material;
use crate::aabb::AABB as AABB;
use crate::onb::ONB as ONB;

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
    fn random_to_sphere(radius: f64, distance_squared: f64)->Vec3{
        let r1 = Vec3::random_double();
        let r2 = Vec3::random_double();
        let z = 1.0+r2*((1.0-radius*radius/distance_squared).sqrt()-1.0);
        let phi = 2.0*std::f64::consts::PI*r1;
        let x = phi.cos()*(1.0-z*z).sqrt();
        let y = phi.sin()*(1.0-z*z).sqrt();

        Vec3::new(x,y,z)
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
    fn pdf_value(&self, origin: Vec3, direction: Vec3)->f64{
        let mut rec = Hit_record::new(self.mat.clone());
        if self.hit(&Ray::new(origin, direction),Interval::new(0.001,std::f64::INFINITY), &mut rec){
            return 0.0
        }

        let distance_squared = (self.center.at(0.0)-origin).length_squared();
        let cos_theta_max = (1.0-self.radius*self.radius/distance_squared).sqrt();
        let solid_angle = 2.0*std::f64::consts::PI*(1.0-cos_theta_max);

        1.0/solid_angle
    }
    fn random(&self, origin: Vec3)->Vec3{
        let direction = self.center.at(0.0)-origin;
        let distance_squared = direction.length_squared();
        let uvw = ONB::new(direction);
        uvw.transform(Self::random_to_sphere(self.radius, distance_squared))
    }
}

