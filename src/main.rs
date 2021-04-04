use clap::{Arg, App};
use cringify::cringify;

fn main() {
    let matches = App::new("Cringify")
        .version("0.1.0")
        .author("Sanskar Jethi <sansyrox@gmail.com>")
        .about("Annoy your friends with the cringified text")
        .arg(Arg::new("string")
                 .short('s')
                 .long("string")
                 .takes_value(true)
                 .about("The main string to be cringified"))
        .arg(Arg::new("prefix")
                 .short('p')
                 .long("prefix")
                 .takes_value(true)
                 .about("Prefix value for the string"))
        .get_matches();

    let main_string = matches.value_of("string").unwrap_or("");

    if main_string=="" {
        println!("Please enter a valid string.");
        return
    }

    let prefix_string = matches.value_of("prefix").unwrap_or("");


    println!("{}", cringify(&main_string, &prefix_string));



}
