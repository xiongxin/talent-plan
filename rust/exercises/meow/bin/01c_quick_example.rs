#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(myapp => 
        (version: "1.0")
        (author: "Xiongxin")
        (about: "Does awesome things!")
        (@arg CONFIG: -c --config +takes_value "Sets a cutom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand test => 
            (about: "controls testing features")
            (version: "31.1")
            (author: "S")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
    ).get_matches();

    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    let config = matches.value_of("CONFIG").unwrap_or("defaut.conf");
    println!("Value for config: {}", config);

    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy!"),
    }

    if let Some(m) = matches.subcommand_matches("test") {
        if m.is_present("verbose") {
            println!("Printing verbosely");
        } else {
            println!("Printing normally");
        }
    }
}