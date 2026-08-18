#[path = "../grayscale.rs"] mod grayscale;

use image::{DynamicImage, GenericImage, GenericImageView};

pub fn halftone_dither(image: &mut DynamicImage){

    const HALFTONE_MATRIX: [[u8; 5]; 5] = [
        [215, 174, 92, 133, 225], 
        [163, 82, 10, 51, 184], 
        [123, 41, 0, 20, 102],
        [205, 72, 31, 61, 143],
        [246, 154, 113, 195, 236],
        ];

    grayscale::grayscale(image);

    let (width, height) = image.dimensions();

    const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

    const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

    for x in 0..width{
        for y in 0..height{
            if image.get_pixel(x, y)[0] < HALFTONE_MATRIX[x as usize % 5][y as usize % 5] {
                image.put_pixel(x, y, BLACK);
            }
            else{
                image.put_pixel(x, y, WHITE);
            }
            
        }
    }
    
}