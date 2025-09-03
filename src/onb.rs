use crate::vec3::Vec3 as Vec3;


pub struct ONB{
    axis: [Vec3; 3],
}

impl ONB{
    pub fn new(n: Vec3)->Self{
        let mut ret = Self{axis: [Vec3::enew(); 3]};
        ret.axis[2] = Vec3::unit_vector(&n);
        let a = if ret.axis[2].x().abs() > 0.9{
            Vec3::new(0.0,1.0,0.0)
        }else{
            Vec3::new(1.0,0.0,0.0)
        };
        ret.axis[1] = Vec3::unit_vector(&Vec3::cross(&ret.axis[2], a));
        ret.axis[0] = Vec3::cross(&ret.axis[2], ret.axis[1]);
        ret
    }
    pub fn u(&self)->Vec3{
        self.axis[0]
    }
    pub fn v(&self)->Vec3{
        self.axis[1]
    }
    pub fn w(&self)->Vec3{
        self.axis[2]
    }
    pub fn transform(&self, v: Vec3)->Vec3{
        (v[0]*self.axis[0])+(v[1]*self.axis[1])+(v[2]*self.axis[2])
    }
}