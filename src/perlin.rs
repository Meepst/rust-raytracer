use crate::vec3::Vec3 as Vec3;

use rand::Rng;

#[derive(Clone)]
pub struct Perlin{
    point_count: usize,
    randVec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin{
    pub fn new()->Self{
        let mut per: Perlin = Perlin::initialize();
        for i in 0..256{
            per.randVec[i] = Vec3::unit_vector(&Vec3::random_vars(-1.0,1.0));
        }

        for i in 0..256{
           eprintln!("i: {} {} {} {}",i, per.randVec[i].x(), per.randVec[i].y(), per.randVec[i].z());
        }

        Self::perlin_generate_perm(&mut per.perm_x, per.point_count);
        Self::perlin_generate_perm(&mut per.perm_y, per.point_count);
        Self::perlin_generate_perm(&mut per.perm_z, per.point_count);
        per
    }
    pub fn noise(&self, p: Vec3)->f64{
        let u = p.x()-p.x().floor();
        let v = p.y()-p.y().floor();
        let w = p.z()-p.z().floor();
        

        let i: usize = p.x().floor() as usize;
        let j: usize = p.y().floor() as usize;
        let k: usize = p.z().floor() as usize;

        let mut c = [[[Vec3::enew(); 2]; 2]; 2];

        for di in 0..2{
            for dj in 0..2{
                for dk in 0..2{
                    c[di][dj][dk] = self.randVec[
                        (self.perm_x[((i+di) & 255) as usize] ^
                        self.perm_y[((j+dj) & 255) as usize] ^
                        self.perm_z[((k+dk) & 255) as usize]) as usize
                    ];
                }
            }
        }


        // for di in 0..2{
        //     for dj in 0..2{
        //         for dk in 0..2{
        //             eprintln!("{}",c[di][dj][dk]);
        //         }
        //     }
        // }
        Self::perlin_interp(c,u,v,w)
    }
    pub fn turb(&self, p: Vec3, depth: i32)->f64{
        let mut accum = 0.0;
        let mut tmp = p;
        let mut weight = 1.0;

        for i in 0..depth{
            accum += weight * self.noise(tmp);
            weight *= 0.5;
            tmp *= 2.0;
        }

        (accum).abs()
    }
    fn initialize()->Self{
        Perlin{
            point_count: 256,
            randVec: vec![Vec3::enew(); 256],
            perm_x: vec![0; 256],
            perm_y: vec![0; 256],
            perm_z: vec![0; 256],
        }
    }
    fn perlin_generate_perm(arr: &mut Vec<i32>, n: usize){
        for i in 0..n{
            arr[i] = i as i32;
        }

        Self::permute(arr, n);
    }
    fn permute(arr: &mut Vec<i32>, n: usize){
        for i in (0..n).rev(){
            let target = rand::thread_rng().gen_range(0..=i);
            let tmp = arr[i];
            arr[i] = arr[target as usize];
            arr[target as usize] = tmp;
        }
    }
    fn perlin_interp(arr: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64)->f64{
        let mut accum: f64 = 0.0;
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);

        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    let weight: Vec3 = Vec3::new(u-i as f64,v-j as f64, w-k as f64);
                    accum += (i as f64*uu+(1.0-i as f64)*(1.0-uu))*(j as f64*vv+(1.0-j as f64)*(1.0-vv))*(k as f64*ww+(1.0-k as f64)*(1.0-ww))*Vec3::dot(&arr[i][j][k], weight);
                }
            }
        }
        accum
    }
}