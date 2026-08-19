#[path = "../grayscale.rs"] mod grayscale;

use clap::builder::styling::AnsiColor::White;
use image::{DynamicImage, GenericImage, GenericImageView};
use rand::RngExt;

pub fn random_dither(image: &mut DynamicImage){

    grayscale::grayscale(image);

    let (width, height) = image.dimensions();

    const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

    const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

    let mut random_numbers:Vec<u8> = Vec::new();

    for _i in 0..(width*height){
        random_numbers.push(rand::rng().random_range(0..=255));
    }

    for x in 0..width{
        for y in 0..height{
            if image.get_pixel(x, y)[0] < rand::rng().random_range(0..=255){
                image.put_pixel(x, y, BLACK);
            }
            else{
                image.put_pixel(x, y, WHITE);
            }
            
        }
    }
    
}