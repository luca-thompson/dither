#[path = "../grayscale.rs"] mod grayscale;

use image::{DynamicImage, GenericImage, GenericImageView};

pub fn bayer_dither(image: &mut DynamicImage){

    const BAYER_MATRIX_FOUR: [[u8; 4]; 4] = [[0, 192, 48, 240], [128, 64, 176, 112], [32, 224, 16, 208], [160, 96, 144, 80]];

    grayscale::grayscale(image);

    let (width, height) = image.dimensions();

    const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

    const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

    for x in 0..width{
        for y in 0..height{
            if image.get_pixel(x, y)[0] < BAYER_MATRIX_FOUR[x as usize % 4][y as usize % 4] {
                image.put_pixel(x, y, BLACK);
            }
            else{
                image.put_pixel(x, y, WHITE);
            }
            
        }
    }
    
}