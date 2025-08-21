use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Interval{
    pub min: f64,
    pub max: f64,
}

impl Interval{
    pub fn new(min: f64, max: f64)->Interval{
        Interval{
            min: min,
            max: max,
        }
    }
    pub fn newi(a: Interval, b: Interval)->Interval{
        Interval{
            min: if a.min() <= b.min(){
                a.min()
            }else{
                b.min()
            },
            max: if a.max() >= b.max(){
                a.max()
            }else{
                b.max()
            },
        }
    }
    pub fn empty()->Interval{
        Interval{
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    pub fn universe()->Interval{
        Interval{
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
    pub fn min(&self)->f64{
        self.min
    }
    pub fn max(&self)->f64{
        self.max
    }
    pub fn size(&self)->f64{
        self.max-self.min
    }
    pub fn contains(&self, x: f64)->bool{
        self.min<= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64)->bool{
        self.min<x && x<self.max
    }
    pub fn clamp(&self, x: f64)->f64{
        if x<self.min{
            return self.min
        }else if x > self.max{
            return self.max
        }
        x
    }
    pub fn expand(&self, delta: f64)->Interval{
        let padding: f64 = delta / 2.0;
        Interval{
            min: self.min - padding,
            max: self.max - padding,
        }
    }
}

impl Add<f64> for Interval{
    type Output = Interval;

    fn add(self, displacement: f64)->Self{
        Self::new(self.min+displacement,self.max+displacement)
    }
}

impl Add<Interval> for f64{
    type Output = Interval;

    fn add(self, ival: Interval)->Interval{
        ival+self
    }
}