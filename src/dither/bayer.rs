#[path = "../grayscale.rs"] mod grayscale;

use image::{DynamicImage, GenericImage, GenericImageView};

pub fn bayer_dither(image: &mut DynamicImage){

    const BAYER_MATRIX_TWO: [[u8; 2]; 2] = [[0, 128], [192, 64]];

    grayscale::grayscale(image);

    let (width, height) = image.dimensions();

    const BLACK: image::Rgba<u8> = image::Rgba([0,0,0,255]);

    const WHITE: image::Rgba<u8> = image::Rgba([255, 125, 255, 255]);

    for x in 0..width{
        for y in 0..height{
            if image.get_pixel(x, y)[0] < BAYER_MATRIX_TWO[x as usize % 2][y as usize % 2] {
                image.put_pixel(x, y, BLACK);
            }
            else{
                image.put_pixel(x, y, WHITE);
            }
            
        }
    }
    
}