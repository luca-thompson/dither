#[path = "bayer.rs"] mod bayer;

use image::DynamicImage;


pub fn dispatch(image: DynamicImage, algorithm: String){

    match algorithm {
        "Bayer" => {
            bayer::bayerDither();
        }
        _ => {
            println!("Unrecognised algorithm: '" + algorithm + "'.");
        }
    }
}