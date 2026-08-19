#[path = "dispatch.rs"] mod dispatch;

//args
use clap::{Parser};

//image
use image::{ImageReader};


#[derive(Parser, Debug)]
struct Args {

    #[arg(long)]
    f_in: String,

    #[arg(long)]
    f_out: String,

    #[arg(long)]
    algorithm: String

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    println!("File in: '{}'.", args.f_in);

    let mut img = ImageReader::open(args.f_in)?.decode()?;

    println!("Dithering using '{}' algorithm.", args.algorithm);
    dispatch::dispatch(&mut img, args.algorithm);

    println!("Success! Saving as: '{}'.", args.f_out);

    let _ = img.save(args.f_out);

    Ok(())
}
