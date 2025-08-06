use std::sync::{Arc, OnceLock};
use crate::hittable::Hittable as Hittable;
use crate::hittable::Hit_record as Hit_record;
use crate::ray::Ray as Ray;
use crate::interval::Interval as Interval;
use crate::material::Material as Material;
use crate::material::Lambertian as Lambertian;
use crate::vec3::Vec3 as Vec3;

pub struct Hittable_List{
    objects: Vec<Arc<dyn Hittable>>,
}

impl Hittable_List{
    pub fn new()->Hittable_List{
        Hittable_List{
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, obj: Arc<dyn Hittable>){
        self.objects.push(obj)
    }
    pub fn clear(&mut self){
        self.objects.clear()
    }
}

static DUMMY_MAT: OnceLock<Arc<dyn Material>> = OnceLock::new();

impl Hittable for Hittable_List{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit_record)->bool{
        let dummy_mat = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));

        let mut temp_rec: Hit_record = Hit_record::new(dummy_mat);
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max();

        for object in &self.objects {
            if object.hit(r,Interval::new(ray_t.min(),closest_so_far), &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t();
                *rec=temp_rec.clone();
            }
        }

        hit_anything
    }
}