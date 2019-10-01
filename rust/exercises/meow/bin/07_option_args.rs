extern crate clap;

use clap::{ App, Arg};

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
            .takes_value(true) // MUST be set to true in order to be an "option" argument
            .short("c")
            .long("config")
            .multiple(true) // Set to true if you wish to allow multiple occurrences
                                    // such as "-i file1 -i file2"
            .required(true)
            .requires("input")
            .conflicts_with("output")
        ).get_matches();


    if let Some(config) = matches.value_of("config") {
        println!("An config file: {}", config);
    }

    if let Some(config_v) = matches.values_of("config") {
        for conf in config_v {
            println!("An config file: {}", conf);
        }
    }

    println!("The \"input\" argument was used {} times", matches.occurrences_of("config"));
}