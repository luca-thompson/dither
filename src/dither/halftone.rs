#[path = "../grayscale.rs"] mod grayscale;

use image::{DynamicImage, GenericImage, GenericImageView};

pub fn halftone_dither(image: &mut DynamicImage){

    const HALFTONE_MATRIX: [[u8; 3]; 3] = [[0, 84, 168], [28, 112, 196], [56, 140, 224]];

    grayscale::grayscale(image);

    let (width, height) = image.dimensions();

    const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

    const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

    for x in 0..width{
        for y in 0..height{
            if image.get_pixel(x, y)[0] < HALFTONE_MATRIX[x as usize % 3][y as usize % 3] {
                image.put_pixel(x, y, BLACK);
            }
            else{
                image.put_pixel(x, y, WHITE);
            }
            
        }
    }
    
}