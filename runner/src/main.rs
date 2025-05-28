use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    println!("Here's the interest: {:?}", &args.interest);
}
