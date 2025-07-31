use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, DivAssign, MulAssign, Index};

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
    pub fn unit_vector(&self)->Vec3{
        *self / self.length()
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
        self.e[0] *= scalar;
        self.e[1] *= scalar;
        self.e[2] *= scalar;
    }
}