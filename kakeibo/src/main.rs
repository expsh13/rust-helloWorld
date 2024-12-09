use clap::Parser;
#[derive(Parser)]
#[clap(version = "1.0")]
struct Args {
    arg1: String,
    arg2: String,
}

fn main() {
    let _args = Args::parse();
}
