use crate::aabb::AABB as AABB;
use crate::hittable::Hittable as Hittable;
use crate::hittable_list::Hittable_List as Hittable_List;
use crate::Interval;
use crate::Ray;
use crate::Hit_record;

use std::sync::Arc;
use rand::Rng;
use std::cmp::Ordering;

pub struct BVH{
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVH{
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>)-> Self{
        let mut bbox: AABB = AABB::empty();
        for obj in &mut *objects{
            bbox = AABB::newb(bbox, obj.bounding_box());
        }

        let axis: i32 = bbox.longest_axis();

        let comparator: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering = if axis == 0{
            box_x_compare
        }else if axis == 1{
            box_y_compare
        }else{
            box_z_compare
        };

        let object_span = objects.len();

        if object_span==1{
            let obj = objects[0].clone();
            BVH {
                left: obj.clone(),
                right: obj,
                bbox: objects[0].bounding_box(),
            }
        }else if object_span == 2{
            let left = objects[0].clone();
            let right = objects[1].clone();
            let bbox = AABB::newb(left.bounding_box(), right.bounding_box());
            BVH{
                left: left,
                right: right,
                bbox: bbox,
            }
        }else{
            objects.sort_by(comparator);

            let mid = object_span / 2;
            let left = Arc::new(BVH::new(objects[..mid].to_vec()));
            let right = Arc::new(BVH::new(objects[mid..].to_vec()));

            let bbox = AABB::newb(left.bounding_box(), right.bounding_box());

            BVH{
                left: left,
                right: right,
                bbox: bbox,
            }
        }
    }
    // pub fn form(&mut self, objects: &Vec<Arc<dyn Hittable + Sync + Send>>, start: usize, end: usize)->BVH{
    //     let axis: i32 = rand::thread_rng().gen_range(0..2);

    //     let comparator: fn(&Arc<dyn Hittable+Send+Sync>, &Arc<dyn Hittable+Send+Sync>)->Ordering = if axis == 0{box_x_compare}
    //     else if axis == 1{box_y_compare}else{box_z_compare};
        
    //     let object_span: usize = end - start;

    //     if object_span == 1{ 
    //         self.left = objects[start].clone();
    //         self.right = objects[start].clone();
    //     }
    //     else if object_span == 2{
    //         self.left = objects[start].clone();
    //         self.right = objects[start+1].clone();
    //     }else{
    //         objects.sort_by(|a, b| comparator(a, b));
            
    //         let mid: usize = start+object_span/2;
    //         self.left = Arc::new(BVH::form(self.left, objects, start, mid));
    //         self.right = Arc::new(BVH::form(self.left, objects, mid, end));
    //     }

    //     self.bbox = AABB::newb(self.left.bounding_box(), self.right.bounding_box());
    // }   
    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32)->Ordering{
        let a_axis_interval: Interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval: Interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval.min().partial_cmp(&b_axis_interval.min()).unwrap_or(Ordering::Equal)
    }
}   

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>)->Ordering{
    BVH::box_compare(a,b,0)
}
fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>)->Ordering{
    BVH::box_compare(a,b,1)
}
fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>)->Ordering{
    BVH::box_compare(a,b,2)
}

impl Hittable for BVH{
    fn hit(&self, r: &Ray, mut ray_t: Interval, rec: &mut Hit_record)->bool{
        if !self.bbox.hit(*r, &mut ray_t){
            return false 
        }

        let hit_left: bool = self.left.hit(r, ray_t, rec);
        let hit_right: bool = self.right.hit(r,Interval::new(ray_t.min(), if hit_left{rec.t()}else{ray_t.max()}), rec);

        hit_left || hit_right
    }
    fn bounding_box(&self)->AABB{
        self.bbox
    }
}