extern crate clap;

use clap::{ App, Arg };

fn main() {
    let matches = App::new("MyApp")
        .about("Parses an input file to do awesome things")
        .version("1.0")
        .author("Xiongxin")
        .arg(Arg::with_name("input")
            .help("the input file to use")
            .index(1)
            .requires("config")
            .conflicts_with("output")
            .required(true))
        .arg(Arg::with_name("config")
            .help("the config file to use")
            .index(2))
        .get_matches();


    if matches.is_present("input") {
        println!("An input file was specified");
    }

    if let Some(in_file) = matches.value_of("input") {
        println!("Doing work with {} and {}", in_file, matches.value_of("config").unwrap());
    }
}