extern crate image;
use rand::Rng;

use image::{GenericImageView, RgbaImage, ImageBuffer, Rgba};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("IMG_0076.jpg").unwrap();
    let dims = img.dimensions();
    
    let mut img2: RgbaImage = ImageBuffer::new(dims.0, dims.1);
    for pixel in img.pixels() {
        if pixel.0 > 2 && pixel.1 > 2 {
            let mypix = pixel.2;
            let mut rng = rand::thread_rng();
            let rnum = rng.gen_range(0,255);
            let pix = Rgba([rnum, mypix.0[0], mypix.0[0], 255]);
            img2.put_pixel(pixel.0, pixel.1, pix);

        }
              
    }
    img2.save("test.png").unwrap();

    // The dimensions method returns the images width and height.
    // println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    

    // Write the contents of this image to the Writer in PNG format.
    // img.save("test.png").unwrap();
}
