#[path = "dither/threshold.rs"] mod threshold;
#[path = "dither/bayer.rs"] mod bayer;
#[path = "dither/halftone.rs"] mod halftone;

use image::DynamicImage;


pub fn dispatch(image: &mut DynamicImage, algorithm: String){

    let alg = algorithm.as_str();

    match alg {
        "threshold" => {
            threshold::threshold_dither(image);
        }
        "bayer" => {
            bayer::bayer_dither(image);
        }
        "halftone" => {
            halftone::halftone_dither(image);
        }
        _ => {
            println!("Unrecognised algorithm: '{}'.", algorithm);
        }
    }
}