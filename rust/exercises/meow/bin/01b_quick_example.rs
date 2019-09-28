extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matchs = App::new("MyApp")
                        .version("1.0")
                        .author("xiongxin")
                        .about("Does awesome things")
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Sets a custom config file")
                                    .takes_value(true))
                        .arg(Arg::with_name("output")
                                    .help("Sets an optional output file")
                                    .index(1))
                        .arg(Arg::with_name("output2")
                                    .help("Sets an optional output file2")
                                    .index(2))
                        .arg(Arg::with_name("debug")
                                    .short("d")
                                    .multiple(true)
                                    .help("Turn debugging information on"))
                        .subcommand(SubCommand::with_name("test")
                                    .about("does testing things")
                                    .arg(Arg::with_name("list")
                                        .short("l")
                                        .help("lists test values")))
                        .get_matches();

    if let Some(o) = matchs.value_of("output") {
        println!("Value for output: {}", o);
    }

    if let Some(o) = matchs.value_of("output2") {
        println!("Value for output2: {}", o);
    }


    if let Some(c) = matchs.value_of("config") {
        println!("value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matchs.occurrences_of("d") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommand, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matchs.subcommand_matches("test") {
        if matches.is_present("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not pringting testing lists...");
        }
    }
}