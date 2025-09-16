use std::sync::Arc;
use crate::hittable::Hittable as Hittable;
use crate::hittable::Hit_record as Hit_record;
use crate::ray::Ray as Ray;
use crate::interval::Interval as Interval;
use crate::material::Lambertian as Lambertian;
use crate::vec3::Vec3 as Vec3;
use crate::aabb::AABB as AABB;
use rand::Rng; 

pub struct Hittable_List{
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl Hittable_List{
    pub fn new()->Hittable_List{
        Hittable_List{
            objects: Vec::new(),
            bbox: AABB::newi(Vec3::enew(),Vec3::enew()),
        }
    }
    pub fn load(list: Vec<Arc<dyn Hittable>>)->Self{
        let mut temp_bbox = list[0].bounding_box();
        for obj in &list[1..]{
            temp_bbox = AABB::newb(temp_bbox, obj.bounding_box());
        }
        let bbox = temp_bbox;

        Hittable_List{
            objects: list.clone(), 
            bbox: bbox,
        }
    }
    pub fn objects(&self)->&Vec<Arc<dyn Hittable>>{
        &self.objects
    }
    pub fn push(&mut self, obj: Arc<dyn Hittable>){
        self.objects.push(obj.clone());
        self.bbox = AABB::newb(self.bbox, obj.bounding_box())
    }
    pub fn clear(&mut self){
        self.objects.clear()
    }
}

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
    fn bounding_box(&self)->AABB{
        self.bbox
    }
    fn pdf_value(&self, origin: Vec3, direction: Vec3)->f64{
        let weight = 1.0/self.objects.len() as f64;
        let mut sum = 0.0;
        for obj in &self.objects{
            sum += weight * obj.pdf_value(origin, direction);
        }
        //eprintln!("Weight: {} Sum: {}",weight, sum);
        sum
    }
    fn random(&self, origin: Vec3)->Vec3{
        //eprintln!("hello!");
        let listSize = self.objects.len();
        self.objects[rand::thread_rng().gen_range(0..listSize-1)].random(origin)
    }
}