use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, DivAssign, MulAssign, Index};
use rand::Rng; 

#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn enew() -> Vec3{
        Vec3{
            e: [0.0,0.0,0.0],
        }
    }
    pub fn new(x: f64,y: f64,z: f64) -> Vec3 {
        Vec3{
            e: [x,y,z],
        }
    }
    pub fn x(&self)->f64{
        self.e[0]
    }
    pub fn y(&self)->f64{
        self.e[1]
    }
    pub fn z(&self)->f64{
        self.e[2]
    }
    pub fn cross(&self, other: Vec3)->Vec3{
        Vec3{
            e: [
                self.e[1]*other.e[2]-self.e[2]*other.e[1],
                self.e[2]*other.e[0]-self.e[0]*other.e[2],
                self.e[0]*other.e[1]-self.e[1]*other.e[0],
            ]
        }
    }
    pub fn length_squared(&self)->f64{
        self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]
    }
    pub fn length(&self)->f64{
        f64::sqrt(self.length_squared())
    }
    pub fn dot(&self, other: Vec3)->f64{
        self.e[0]*other.e[0]+self.e[1]*other.e[1]+self.e[2]*other.e[2]
    }
    fn random_double()->f64{
        rand::thread_rng().gen_range(0.0..1.0)
    }
    fn random_between(min: f64, max: f64)->f64{
        rand::thread_rng().gen_range(min..max)
    }
    pub fn random()->Vec3{
        Vec3::new(Self::random_double(),Self::random_double(),Self::random_double())
    }
    pub fn random_vars(min: f64, max: f64)->Vec3{
        Vec3::new(Self::random_between(min,max),Self::random_between(min,max),Self::random_between(min,max))
    }
    pub fn unit_vector(&self)->Vec3{
        *self / self.length()
    }
    pub fn random_unit_vector()->Vec3{
        loop {
            let p: Vec3 = Self::random_vars(-1.0,1.0);
            let lensq: f64 = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0{
                return p / lensq.sqrt()
            }
        }
    }
    pub fn random_on_hemisphere(normal: &Vec3)->Vec3{
        let on_unit_sphere: Vec3 = Self::random_unit_vector();
        if on_unit_sphere.dot(*normal) > 0.0 {
            return on_unit_sphere
        }else{
            -on_unit_sphere
        }
    }
    pub fn near_zero(&self)->bool{
        let s: f64 = 1e-8;
        f64::abs(self.x())<s && f64::abs(self.y())<s&& f64::abs(self.z())< s
    }
    pub fn reflect(&self, n: &Vec3)->Vec3{
        *self - 2.0*self.dot(*n)**n
    }
    pub fn refract(&self, n: Vec3, etai_over_etat)->Vec3{
        let cos_theta: f64 = f64::min(self.dot(n),1.0);
        let r_out_perp: Vec3 = etai_over_etat * (self + cos_theta*n);
        
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3)->Vec3{
        Vec3{
            e: [self.x()+other.x(),self.y()+other.y(),self.z()+other.z()]
        }
    }
}

impl Sub for Vec3{
    type Output = Vec3;

    fn sub(self, other: Vec3)->Vec3{
        Vec3{
            e: [self.x()-other.x(),self.y()-other.y(),self.z()-other.z()]
        }
    }
}

impl Neg for Vec3{
    type Output = Vec3;

    fn neg(self)->Vec3{
        Vec3{
            e: [-self.x(),-self.y(),-self.z()]
        }
    }
}

impl Index<usize> for Vec3{
    type Output = f64;

    fn index(&self, i: usize)->&f64{
        &self.e[i]
    }
}

impl Mul for Vec3{
    type Output = Vec3;

    fn mul(self, other: Vec3)->Vec3{
        Vec3{
            e: [self.x()*other.x(),self.y()*other.y(), self.z()*other.z()]
        }
    }
}

impl Mul<f64> for Vec3{
    type Output = Vec3;

    fn mul(self, scalar: f64)->Vec3{
        Vec3{
            e: [self.x()*scalar, self.y()*scalar, self.z()*scalar]
        }
    }
}

impl Mul<Vec3> for f64{
    type Output = Vec3;

    fn mul(self, other: Vec3)->Vec3{
        other * self
    }
}

impl Div<f64> for Vec3{
    type Output = Vec3;

    fn div(self, scalar: f64)->Vec3{
        (1.0/scalar) * self
    }
}


impl AddAssign for Vec3{

    fn add_assign(&mut self, other: Vec3){
        self.e[0]+=other.e[0];
        self.e[1]+=other.e[1];
        self.e[2]+=other.e[2];
    }
}

impl AddAssign<f64> for Vec3{
    fn add_assign(&mut self, scalar: f64){
        self.e[0]+=scalar;
        self.e[1]+=scalar;
        self.e[2]+=scalar;
    }
}

impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, scalar: f64){
        self.e[0]/=scalar;
        self.e[1]/=scalar;
        self.e[2]/=scalar;
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, scalar: f64){
        self.e[0]*=scalar;
        self.e[1]*=scalar;
        self.e[2]*=scalar;
    }
}