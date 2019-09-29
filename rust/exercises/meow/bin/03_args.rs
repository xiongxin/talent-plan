extern crate clap;

use clap::{App, Arg};


fn main() {

    let matches = App::new("MyApp")
        // A simple Flg argument example (i.e "-d") using the builder pattern
        .arg(Arg::with_name("debug").help("turn on debugging information").short("d"))

        // Two arguments, on "Option" argument (i.e. one that takes a value) such
        // as "-c some", and one positional argument (i.e. "myapp some_file")
        .args(&[
            Arg::with_name("config")
                .help("sets the config file to use")
                .takes_value(true) // 制定为Option
                .short("c")
                .long("config"),
            Arg::with_name("input")
                .help("the input file to use")
                .index(1)
                .required(true)
        ])

        // Note the following two examples are convenience methods, if you wish
        // to still get the full configurability of Arg::with_name() and the readability
        // of arg_from_usage(), you can instantiate a new Arg with Arg::from_usage() and
        // still be able to set all additional properties, just like Arg::with_name()
        //
        //
        // On Flag using a usage string
        .arg_from_usage("-l, --license 'display the license file'") // FLAG ，没有value

        // Two args, on "Positional", and one "Option" using a usage string
        .arg_from_usage("[output] 'Supply an output file to use'")
        .arg_from_usage("-i, --int=[IFACE] 'Set an interface to use'")
        .get_matches();

    println!("Debugging mode is : {}", if matches.is_present("debug") { "ON" } else { "OFF" });

    if matches.is_present("license") {
        println!("is present license,: {:?}", matches.value_of("license"));
    }

    if let Some(config) = matches.value_of("config") {
        println!("A config file was passed in :{}", config)
    }

    if let Some(output) = matches.value_of("output") {
        println!("A output file was passed in :{}", output)
    }

    if let Some(int) = matches.value_of("int") {
        println!("A int file was passed in :{}", int)
    }
}