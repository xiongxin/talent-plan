extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let matchs = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K.")
                    .about("Does awesome things")
                    .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'
                                    <output> 'Sets an optional output file'
                                    -d... 'Turn deugging information on'")
                    .subcommand(SubCommand::with_name("test")
                                    .about("does testing things")
                                    .arg_from_usage("-l, --list 'lists test values'"))
                    .get_matches();

    if let Some(o) = matchs.value_of("output") {
        println!("Value for output: {}", o);
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