use clap::Parser;
use cringify::cringify;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author= "Sanskar Jethi <sansyrox@gmail.com>", version="0.0.1", about="Annoy your friends with cringified text", long_about = None)]
struct Cli {
    /// The pattern to look for
    string: String,
    /// The path to the file to read
    #[arg(default_value = "")]
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
