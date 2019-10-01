extern crate clap;

use clap::{ App, Arg };

fn main() {

    let matches = App::new("MyApp")
        .about("Parses an input file to do awesome things")
        .version("1.0")
        .author("Xiongxin")
        .arg(Arg::with_name("debug")
            .help("turn on debugging information")
            .short("d")
            .long("debug"))
        .arg(Arg::with_name("config")
            .help("sets the config file to use")
            .short("c")
            .takes_value(true)
            .long("config"))
        .arg(Arg::with_name("input")
            .help("the input file to use")
            .index(1)
            .required(true))
        .get_matches();


    if  matches.is_present("debug") {
        println!("Debugging is turned on");
    }

    if matches.is_present("config") {
        if let Some(file) = matches.value_of("config") {
            println!("Using config file: {}", file);
        }
    }

    println!("Doing real work with file: {}", matches.value_of("input").unwrap());
}