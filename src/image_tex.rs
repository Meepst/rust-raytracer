use std::env;
use std::path::PathBuf;
use stb_image::image::{Image, load, LoadResult};

pub struct RtwImage{
    bytes_per_pixel: usize,
    bdata: Option<Vec<u8>>,
    fdata: Option<Vec<f32>>,
    image_width: usize,
    image_height: usize,
    bytes_per_scanline: usize,
}

impl RtwImage{
    pub fn new()->Self{
        Self{
            bytes_per_pixel: 3,
            bdata: None,
            fdata: None,
            image_width: 0,
            image_height: 0,
            bytes_per_scanline: 0,
        }
    }
    pub fn from_file(filename: &str)->Self{
        let mut img = Self::new();
        let rootDir = env!("CARGO_MANIFEST_DIR");
        let imagePath = format!("{}/assets/images/{}", rootDir, filename);

        let search_paths = [
            Some(imagePath),
            Some(filename.to_string()),
            Some(format!("images/{}", filename)),
        ];

        for pathstr in search_paths.iter().flatten(){
            if img.load(pathstr){
                return img
            }
        }

        eprintln!("failed to load image file'{}'", filename);
        img
    }
    pub fn load(&mut self, filename: &str)->bool{
        eprintln!("{}", filename);
        let path = PathBuf::from(filename);
        if !path.exists(){
            return false
        }

        match load(path){
            LoadResult::ImageF32(body)=>{
                eprintln!("f32");
                self.image_width = body.width;
                self.image_height = body.height;
                self.bytes_per_scanline = body.width*self.bytes_per_pixel;
                self.fdata = Some(body.data);
                self.convert_to_bytes();
                true
            }
            LoadResult::ImageU8(body)=>{
                eprintln!("u8");
                self.image_width = body.width;
                self.image_height = body.height;
                self.bytes_per_scanline = body.width*self.bytes_per_pixel;
                self.bdata = Some(body.data);
                self.fdata = None;
                true
            }
            LoadResult::Error(_)=>{
                eprintln!("failed to load image");
                false
            }
        }
    }
    pub fn width(&self)->usize{
        self.image_width
    }
    pub fn height(&self)->usize{
        self.image_height
    }
    fn convert_to_bytes(&mut self){
        if let Some(fdata) = &self.fdata{
            let total_bytes = self.width()*self.height()*self.bytes_per_pixel;
            let mut bdata = Vec::with_capacity(total_bytes);
            for &val in fdata.iter().take(total_bytes){
                bdata.push(Self::float_to_byte(val));
            }
            self.bdata = Some(bdata);
        }
    }
    fn float_to_byte(val: f32)->u8{
        if val <= 0.0{
            return 0
        }else if val >= 1.0{
            return 255
        }
        (256.0*val) as u8
    }
    pub fn pixel_data(&self, x: usize, y: usize)->[u8; 3]{
        static MAGENTA: [u8; 3] = [255,0,255];
        if let Some(bdata) = &self.bdata{
            let x = Self::clamp(x, 0, self.width());
            let y = Self::clamp(y, 0, self.height());
            let start = y*self.bytes_per_scanline+x*self.bytes_per_pixel;
            return [bdata[start],bdata[start+1],bdata[start+2]]
        }
        MAGENTA
    }
    fn clamp(x: usize, low: usize, high: usize)->usize{
        if x < low{
            return low 
        }else if x < high{
            return x
        }
        high-1
    }
}