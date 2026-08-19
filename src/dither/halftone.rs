#[path = "../grayscale.rs"] mod grayscale;

use image::{DynamicImage, GenericImage, GenericImageView};

pub fn halftone_dither(image: &mut DynamicImage){

    const HALFTONE_MATRIX: [[u8; 5]; 5] = [
        [215, 174, 92, 133, 225], 
        [163, 82, 0, 51, 184], 
        [123, 40, 0, 30, 102],
        [205, 72, 0, 61, 143],
        [246, 154, 113, 195, 236],
        ];

    grayscale::grayscale(image);

    let (width, height) = image.dimensions();

    const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

    const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

    let mut matrix_value: u8;

    for x in 0..width{
        for y in 0..height{

            matrix_value = HALFTONE_MATRIX[x as usize % 5][y as usize % 5];

            if matrix_value == 0 {
                if image.get_pixel(x, y)[0]  < 128 {
                    image.put_pixel(x, y, WHITE);
                }
                else{
                    image.put_pixel(x, y, BLACK);
                }

                continue;
            }
            else if matrix_value == 255 {
                
            }

            if image.get_pixel(x, y)[0] < matrix_value {
                image.put_pixel(x, y, BLACK);
            }
            else{
                image.put_pixel(x, y, WHITE);
            }
            
        }
    }
    
}