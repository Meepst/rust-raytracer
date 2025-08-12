use crate::interval::Interval as Interval;
use crate::vec3::Vec3 as Vec3;
use crate::ray::Ray as Ray;
use crate::hittable::Hittable as Hittable;

#[derive(Clone, Copy)]
pub struct AABB{
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB{
    pub fn new(x: Interval, y: Interval, z: Interval)->AABB{
        AABB{
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn newi(a: Vec3, b: Vec3)->AABB{
        AABB{
            x:  if a.x() <= b.x(){
                    Interval::new(a.x(),b.x())
                }else{Interval::new(b.x(),a.x())},
            y:  if a.y() <= b.y(){
                    Interval::new(a.y(),b.y())
                }else{Interval::new(b.y(),a.y())},
            z:  if a.z() <= b.z(){
                    Interval::new(a.z(),b.z())
                }else{Interval::new(b.z(),a.z())},
        }
    }
    pub fn newb(box0: AABB, box1: AABB)->AABB{
        AABB{
            x: Interval::newi(box0.x(),box1.x()),
            y: Interval::newi(box0.y(),box1.y()),
            z: Interval::newi(box0.z(),box1.z()),
        }
    }
    pub fn empty()->AABB{
        Self::new(Interval::empty(),Interval::empty(),Interval::empty())
    }
    pub fn universe()->AABB{
        Self::new(Interval::universe(), Interval::universe(), Interval::universe())
    }
    pub fn x(&self)->Interval{
        self.x
    }
    pub fn y(&self)->Interval{
        self.y
    }
    pub fn z(&self)->Interval{
        self.z
    }
    pub fn axis_interval(&self, n: i32)->Interval{
        if n == 1{
            return self.y
        }else if n==2{
            return self.z
        }
        self.x
    }
    pub fn longest_axis(&self)->i32{
        if self.x().size() > self.y().size(){
            return if self.x().size() > self.z().size(){
                0
            }else{
                2
            }
        }else{
            if self.y().size() > self.z().size(){
                1
            }else{
                2
            }
        }
    }
    pub fn hit(&self, r: Ray, ray_t: &mut Interval)->bool{
        let ray_orig: Vec3 = r.origin();
        let ray_dir: Vec3 = r.direction();

        for axis in 0..3{
            let ax: Interval = self.axis_interval(axis);
            let adinv: f64 = 1.0/ray_dir[axis as usize];

            let t0: f64 = (ax.min() - ray_orig[axis as usize])*adinv;
            let t1: f64 = (ax.max() - ray_orig[axis as usize])*adinv;

            if t0 < t1{
                if t0 > ray_t.min(){
                    ray_t.min = t0;
                }
                if t1 < ray_t.max(){
                    ray_t.max = t1;
                }
            }else{
                if t1 > ray_t.min(){
                    ray_t.min = t1;
                }
                if t0 < ray_t.max(){
                    ray_t.max = t0;
                }
            }

            if ray_t.max() <= ray_t.min(){
                return false 
            }
        }
        true
    }
}
