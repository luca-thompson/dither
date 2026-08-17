#[path = "bayer.rs"] mod bayer;

use image::DynamicImage;


pub fn dispatch(image: DynamicImage, algorithm: String){

    let alg = algorithm.as_str();

    match alg {
        "bayer" => {
            bayer::bayer_dither(image);
        }
        _ => {
            println!("Unrecognised algorithm: '{}'.", algorithm);
        }
    }
}