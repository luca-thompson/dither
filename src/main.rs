//args
use clap::Parser;

//image
use image::ImageReader;


#[derive(Parser, Debug)]
struct Args {

    #[arg(long)]
    f_in: String,

    #[arg(long)]
    f_out: String

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    println!("{}", args.f_in);
    println!("{}", args.f_out);

    let img = ImageReader::open(args.f_in)?.decode()?;

    img.save(args.f_out)?;

    Ok(())
}
