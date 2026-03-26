use image::{RgbImage, Rgb};
use std::path::Path;

pub fn save_dummy_graph(path:&Path){

    let width = 1200;
    let height = 500;

    let mut img = RgbImage::new(width,height);

    for x in 0..width{
        for y in 0..height{

            let color = if (x+y)%50<25 {
                Rgb([0,255,0])
            } else {
                Rgb([0,0,0])
            };

            img.put_pixel(x,y,color);
        }
    }

    img.save(path).unwrap();
}
