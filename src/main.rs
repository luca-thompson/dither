use clap::Parser;

#[derive(Parser, Debug)]

struct Args {

    #[arg(short, long)]
    name: String,

}

fn main() {
    let args = Args::parse();

    println!("{}", args.name);
}
