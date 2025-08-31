use crate::hittable::Hittable as Hittable;
use crate::vec3::Vec3 as Vec3;
use crate::material::Material as Material;
use crate::aabb::AABB as AABB;
use crate::hittable::Hit_record as Hit_record;
use crate::ray::Ray as Ray;
use crate::interval::Interval as Interval;
use crate::hittable_list::Hittable_List as Hittable_List;

use std::sync::Arc;

pub struct Quad{
    Q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    D: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

// box from RTW but Box is a reserved word
pub struct Cube{
    pub sides: Vec<Arc<dyn Hittable>>,
}

impl Quad{
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>)->Self{
        let mut ret: Quad = Self{
            Q: Q,
            u: u,
            v: v,
            w: Vec3::enew(),
            normal: Vec3::enew(),
            D: 0.0,
            mat: mat,
            bbox: AABB::newi(Vec3::enew(),Vec3::enew()),
        };

        let n = Vec3::cross(&ret.u, ret.v);
        ret.normal = Vec3::unit_vector(&n);
        ret.D = ret.normal.dot(ret.Q);
        ret.w = n.clone() / n.dot(n.clone());

        ret.set_bounding_box();
        ret
    }
    fn set_bounding_box(&mut self){
        let bbox_diag1: AABB = AABB::newi(self.Q, self.Q+self.u+self.v);
        let bbox_diag2: AABB = AABB::newi(self.Q+self.u,self.Q+self.v);
        self.bbox = AABB::newb(bbox_diag1, bbox_diag2);
    }
    fn is_interior(a: f64, b: f64, rec: &mut Hit_record)->bool{
        let interval = Interval::new(0.0,1.0);

        if !interval.contains(a) || !interval.contains(b){
            return false 
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Cube{
    pub fn new(a: Vec3, b: Vec3, mat: Arc<dyn Material>)->Arc<Hittable_List>{
        let min = Vec3::new(a.x().min(b.x()),a.y().min(b.y()),a.z().min(b.z()));
        let max = Vec3::new(a.x().max(b.x()),a.y().max(b.y()),a.z().max(b.z()));

        let dx = Vec3::new(max.x()-min.x(),0.0,0.0);
        let dy = Vec3::new(0.0, max.y()-min.y(), 0.0);
        let dz = Vec3::new(0.0,0.0,max.z()-min.z());
        
        let mut retCube = Hittable_List::new();

        retCube.push(Arc::new(Quad::new(Vec3::new(min.x(),min.y(),max.z()),dx,dy,mat.clone())));//front
        retCube.push(Arc::new(Quad::new(Vec3::new(max.x(),min.y(),max.z()),-dz,dy,mat.clone())));//right
        retCube.push(Arc::new(Quad::new(Vec3::new(max.x(),min.y(),min.z()),-dx,dy,mat.clone())));//back
        retCube.push(Arc::new(Quad::new(Vec3::new(min.x(),min.y(),min.z()),dz,dy,mat.clone())));//left
        retCube.push(Arc::new(Quad::new(Vec3::new(min.x(),max.y(),max.z()),dx,-dz,mat.clone())));//top
        retCube.push(Arc::new(Quad::new(Vec3::new(min.x(),min.y(),min.z()),dx,dz,mat)));//bottom

        Arc::new(retCube)
    }
}

impl Hittable for Quad{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool{
        let denom = self.normal.dot(r.direction());

        if denom.abs() < 1e-8{
            return false
        }

        let t = (self.D-self.normal.dot(r.origin()))/denom;
        if !ray_t.contains(t){
            return false 
        }

        let intersection = r.at(t);
        let p_vec = intersection-self.Q; // planar vec hitpoint
        let alpha = self.w.dot(Vec3::cross(&p_vec, self.v));
        let beta = self.w.dot(Vec3::cross(&self.u, p_vec));

        if !Self::is_interior(alpha, beta, rec){
            return false
        }

        rec.setT(t);
        rec.setP(intersection);
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }
    fn bounding_box(&self)->AABB{
        self.bbox
    }
}

