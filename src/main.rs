use clap::{Arg, App};

fn cringify(main_string: &str, prefix: &str) -> String {
    let mut index = 0;
    let final_string = main_string
        .chars()
        .map(|letter| {
            let return_character = letter;
            if !return_character.is_ascii_alphabetic() {
                index+=1;
                return return_character;
            }

            let cringe_character;
            if index%2==1 {
                cringe_character = return_character.to_uppercase().next().unwrap();
            } else {
                cringe_character = return_character.to_lowercase().next().unwrap();
            }
            index += 1;
            return cringe_character
    }).collect::<String>();

    if prefix!="" {
        return format!("{} {}", prefix, final_string.as_str());
    }
    else {
        return final_string;
    }
}

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
        println!("Please enter a valid string");
        return
    }
    println!("The main string is: {}", main_string);

    let prefix_string = matches.value_of("prefix").unwrap_or("");


    println!("{}", cringify(&main_string, &prefix_string));



}
