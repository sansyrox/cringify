use clap::Parser;
use cringify::cringify;

/// The cringify command
#[derive(Parser)]
#[command(author= "Sanskar Jethi <sansyrox@gmail.com>", version="0.0.1", about="Annoy your friends with cringified text", long_about = None)]
struct Cli {
    /// The string to cringify
    string: String,
    /// The prefix you want your result to start with
    #[arg(short = 'p', default_value = "")]
    prefix: String,
}

fn main() {
    let args = Cli::parse();

    if args.string == "" {
        println!("Please enter a valid string.");
        return;
    }

    println!("{}", cringify(args.string.as_str(), args.prefix.as_str()));
}
