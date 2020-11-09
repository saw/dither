extern crate image;

use image::{GenericImageView, RgbaImage, ImageBuffer, Rgba};

fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

fn getLuma(pixel:&image::Rgba<u8> ) -> u8 {
    let mut list: [i32; 3] = [0; 3];
    let mut list: [i32; 3] = [0; 3];
    list[0] = pixel[0] as i32;
    list[1] = pixel[1] as i32;
    list[2] = pixel[2] as i32;
    let luma =  mean(&list.to_vec()) as u8;
    return luma;
}

fn getAdjustedPixel(old_pixel:image::Rgba<u8>, adjustment:i8) -> u8 {
    
    let mut new_pixel = Rgba([old_pixel[0], old_pixel[1], old_pixel[2], 255]);
    let luma = getLuma(&new_pixel) as i32;
    let adjusted:i32 = luma + adjustment as i32;
    let mut newluma = adjusted;
    if newluma < 0 {
        newluma = 255;
    } else if newluma > 255 {
        newluma = 255;
    }
    return newluma as u8;
}

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut read = image::open("IMG_0076.jpg").unwrap();
    let dims = read.dimensions();
    let img = read.into_rgba();
    
    let mut img2: RgbaImage = ImageBuffer::new(dims.0, dims.1);
    img2.put_pixel(0,0, *img.get_pixel(0,0));
    for mut pixel in img.enumerate_pixels() {
        // println!("Pix: {:?}", pixel);
        let mut my_pix = img2.get_pixel(pixel.0, pixel.1);
        let oldluma =  getLuma(&my_pix);
        let mut newluma:u8 = 0;
        if oldluma > 127 {
            newluma = 255;
        }
        let quant_error:i8;

        quant_error = oldluma as i8 - newluma as i8;
        // println!("Quant error {:?}", quant_error);
        let mut pix = Rgba([newluma, newluma, newluma, 255]);
        img2.put_pixel(pixel.0, pixel.1, pix);
        let mut next_pixel;
        let mut adjval:u8;
        let mut adjpix:Rgba<u8>;
        
        if (pixel.0 < dims.0-1) {
            next_pixel = img.get_pixel(pixel.0 +1, pixel.1);
            let quant_error_transformed:f32 = quant_error as f32 * 7.0/16.0;
            adjval = getAdjustedPixel(*next_pixel, quant_error as i8); // cast to u8 same as floor and faster
            // next_pixel
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(pixel.0+1, pixel.1, fpix);
        }


        /*
        pixel[x + 1][y    ] := pixel[x + 1][y    ] + quant_error × 7 / 16
        pixel[x - 1][y + 1] := pixel[x - 1][y + 1] + quant_error × 3 / 16
        pixel[x    ][y + 1] := pixel[x    ][y + 1] + quant_error × 5 / 16
        pixel[x + 1][y + 1] := pixel[x + 1][y + 1] + quant_error × 1 / 16
        */


              
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
