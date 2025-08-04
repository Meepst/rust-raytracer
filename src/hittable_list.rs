use std::sync::Arc;
use crate::hittable::Hittable as Hittable;
use crate::hittable::Hit_record as Hit_record;
use crate::ray::Ray as Ray;

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

impl Hittable for Hittable_List{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hit_record)->bool{
        let mut temp_rec: Hit_record = Hit_record::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object in &self.objects {
            if object.hit(r,t_min,closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t();
                *rec=temp_rec;
            }
        }

        hit_anything
    }
}