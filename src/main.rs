extern crate image;

use image::{GenericImageView, RgbaImage, ImageBuffer, Rgba};

fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}


fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("IMG_0076.jpg").unwrap();
    let dims = img.dimensions();
    
    let mut img2: RgbaImage = ImageBuffer::new(dims.0, dims.1);
    for pixel in img.pixels() {
        let my_pix = pixel.2;
        let mut list: [i32; 3] = [0; 3];
        list[0] = my_pix[0] as i32;
        list[1] = my_pix[1] as i32;
        list[2] = my_pix[2] as i32;
        let avg =  mean(&list.to_vec()) as u8;
        let pix = Rgba([avg, avg, avg, 255]);
        img2.put_pixel(pixel.0, pixel.1, pix);
        // let avg:u32 = (old_pixel / 3).into();
        // println!("old pix {:?}", avg);
        // if pixel.0 > 2 && pixel.1 > 2 {
        //     // let mypix = pixel.2;
        //     // let mut rng = rand::thread_rng();
        //     // let rnum = rng.gen_range(0,255);
        //     let mut val = 0;
        //     if pixel.2.0[0] > 207 {
        //         val = 255;
        //     }
        //     let pix = Rgba([val, val, val, 255]);
        //     img2.put_pixel(pixel.0, pixel.1, pix);

        // }
              
    }
    img2.save("test.png").unwrap();
    // let my_int:u8 = 34;
    // let my_int2:u8 = 12;
    // let my_int3:u8 = 200;
    // let mut list: [i32; 3] = [0; 3];
    // list[0] = my_int2 as i32;
    // list[1] = my_int as i32;
    // list[2] = my_int3 as i32;
    // let avg = mean(&list.to_vec());
    // println!("The mean is {:?}.", avg as u8);
}
