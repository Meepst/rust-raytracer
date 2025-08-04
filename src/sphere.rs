use crate::vec3::Vec3 as Vec3;
use crate::hittable::Hit_record as Hit_record;
use crate::hittable::Hittable as Hittable;
use crate::ray::Ray as Ray;

pub struct Sphere{
    center: Vec3,
    radius: f64,
}

impl Sphere{
    pub fn new(center: &Vec3, radius: f64)->Sphere{
        Sphere{
            center: *center,
            radius: f64::max(0.0,radius),
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut Hit_record)->bool{
        let oc: Vec3 = self.center-r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = Vec3::dot(&r.direction(), oc);
        let c: f64 = oc.length_squared()-self.radius*self.radius;

        let discriminant: f64 = h*h-a*c;
        if discriminant < 0.0{
            return false
        }

        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (h-sqrtd)/a;
        if root <= ray_tmin || ray_tmax <= root{
            root = (h+sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root{
                return false
            }
        }

       
        rec.setT(root);
        rec.setP(r.at(rec.t()));
        let outward_normal: Vec3 = (rec.p() - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);


        return true
    }
}