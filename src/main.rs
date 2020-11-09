extern crate image;
extern crate clap;
use clap::{Arg, App, SubCommand};

use image::{GenericImageView, RgbaImage, ImageBuffer, Rgba, DynamicImage};
use std::env;


const MULTIPLIER:f32 = 0.125;

fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

fn get_luma(pixel:&image::Rgba<u8> ) -> u8 {
    let mut list: [i32; 3] = [0; 3];
    list[0] = pixel[0] as i32;
    list[1] = pixel[1] as i32;
    list[2] = pixel[2] as i32;
    let luma =  mean(&list.to_vec()) as u8;
    return luma;
}

fn get_adjusted_pixel(old_pixel:image::Rgba<u8>, adjustment:i8) -> u8 {
    
    let new_pixel = Rgba([old_pixel[0], old_pixel[1], old_pixel[2], 255]);
    let luma = get_luma(&new_pixel) as i32;
    let adjusted:i32 = luma + adjustment as i32;
    let mut newluma = adjusted;
    if newluma < 0 {
        newluma = 255;
    } else if newluma > 255 {
        newluma = 255;
    }
    return newluma as u8;
}

fn atikinson(input: DynamicImage, filename:&str)  {
    let dims = input.dimensions();
    let img = input.into_rgba();
    let mut img2: RgbaImage = ImageBuffer::new(dims.0, dims.1);
    img2.put_pixel(0,0, *img.get_pixel(0,0));
    for pixel in img.enumerate_pixels() {
        // println!("Pix: {:?}", pixel);
        let my_pix = img2.get_pixel(pixel.0, pixel.1);
        let oldluma =  get_luma(&my_pix);
        let mut newluma:u8 = 0;
        if oldluma > 127 {
            newluma = 255;
        }
        let quant_error:i8;

        quant_error = oldluma as i8 - newluma as i8;
        // println!("Quant error {:?}", quant_error);
        let pix = Rgba([newluma, newluma, newluma, 255]);
        img2.put_pixel(pixel.0, pixel.1, pix);
        let mut next_pixel;
        let mut adjval:u8;
        let mut index_x:i32;
        let mut index_y:i32;

        /*

                Atikinson
        pixel[x + 1][y    ] := pixel[x + 1][y    ] + quant_error × 1 / 8
        pixel[x + 2][y    ] := pixel[x + 1][y    ] + quant_error × 1 / 8
        pixel[x - 1][y + 1] := pixel[x - 1][y + 1] + quant_error × 1 / 8
        pixel[x    ][y + 1] := pixel[x    ][y + 1] + quant_error × 1 / 8
        pixel[x + 1][y + 1] := pixel[x + 1][y + 1] + quant_error × 1 / 8
        pixel[x][y +1] := pixel[x + 2][y    ] + quant_error × 1 / 8 */

        if pixel.0 < dims.0-1 {
            index_x = pixel.0 as i32 + 1;
            index_y = pixel.1 as i32;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * MULTIPLIER;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.0 < dims.0-2 {
            index_x = pixel.0 as i32 + 2;
            index_y = pixel.1 as i32;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * MULTIPLIER;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.0 > 0  && pixel.1 < dims.1 - 1 {
            index_x = pixel.0 as i32 - 1;
            index_y = pixel.1 as i32 + 1;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * MULTIPLIER;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.1 < dims.1 -1 {
            index_x = pixel.0 as i32;
            index_y = pixel.1 as i32 + 1;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * MULTIPLIER;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.0 < dims.0-1 && pixel.1 < dims.1 -1 {
            index_x = pixel.0 as i32 + 1;
            index_y = pixel.1 as i32 + 1;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * MULTIPLIER;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.0 < dims.0-2 && pixel.1 < dims.1 -1 {
            index_x = pixel.0 as i32 + 2;
            index_y = pixel.1 as i32 + 1;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * MULTIPLIER;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }
    }

    img2.save(filename).unwrap();
}

fn floyd_steinberg(input: DynamicImage, filename:&str)  {
    let dims = input.dimensions();
    let img = input.into_rgba();
    let mut img2: RgbaImage = ImageBuffer::new(dims.0, dims.1);
    img2.put_pixel(0,0, *img.get_pixel(0,0));
    for pixel in img.enumerate_pixels() {
        // println!("Pix: {:?}", pixel);
        let my_pix = img2.get_pixel(pixel.0, pixel.1);
        let oldluma =  get_luma(&my_pix);
        let mut newluma:u8 = 0;
        if oldluma > 127 {
            newluma = 255;
        }
        let quant_error:i8;

        quant_error = oldluma as i8 - newluma as i8;
        // println!("Quant error {:?}", quant_error);
        let pix = Rgba([newluma, newluma, newluma, 255]);
        img2.put_pixel(pixel.0, pixel.1, pix);
        let mut next_pixel;
        let mut adjval:u8;
        let mut index_x:i32;
        let mut index_y:i32;
        if pixel.0 < dims.0-1 {
            index_x = pixel.0 as i32 + 1;
            index_y = pixel.1 as i32;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * 7.0/16.0;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.0 > 0  && pixel.1 < dims.1 - 1 {
            index_x = pixel.0 as i32 - 1;
            index_y = pixel.1 as i32 + 1;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * 3.0/16.0;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.1 < dims.1 -1 {
            index_x = pixel.0 as i32;
            index_y = pixel.1 as i32 + 1;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * 5.0/16.0;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }

        if pixel.0 < dims.0-1 && pixel.1 < dims.1 -1 {
            index_x = pixel.0 as i32 + 1;
            index_y = pixel.1 as i32 + 1;
            next_pixel = img.get_pixel(index_x as u32, index_y as u32);
            let quant_error_transformed:f32 = quant_error as f32 * 1.0/16.0;
            adjval = get_adjusted_pixel(*next_pixel, quant_error_transformed as i8); // cast to u8 same as floor and faster
            let fpix = Rgba([adjval, adjval, adjval, 255]);
            img2.put_pixel(index_x as u32, index_y as u32, fpix);
        }
    }

    img2.save(filename).unwrap();
}

fn main() {
    let matches = App::new("Ditherface")
        .version("0,0")
        .arg(Arg::with_name("INPUT")
            .help("Input file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("o")
            .short("o")
            .long("out")
            .help("Output path")
            .takes_value(true)
            .required(true))
            .get_matches();
    // let path = &args[1];
    let filename = matches.value_of("INPUT").unwrap();
    let outpath = matches.value_of("o").unwrap();
    let read = image::open(filename).expect("Read failure");
    atikinson(read, outpath);
}
