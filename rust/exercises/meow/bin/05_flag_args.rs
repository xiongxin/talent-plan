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
        .arg(Arg::with_name("output")
            .help("the output file to use")
            .index(2))
        .arg(Arg::with_name("awesome")
            .help("turns up the awesome")
            .short("a")
            .long("awesome")
            .multiple(true) // This flag should allow multiple
                                  // occurrences such as "-aaa" or "-a -a
            .requires("config") // 必须联合参数制定, 必须制定config参数
            .conflicts_with("output") // 排他参数制定, 不能再使用output参数了
        ).get_matches();

    if  matches.is_present("debug") {
        println!("Debugging is turned on");
    }

    if matches.is_present("config") {
        if let Some(file) = matches.value_of("config") {
            println!("Using config file: {}", file);
        }
    }

    println!("Doing real work with file: {}", matches.value_of("input").unwrap());

    if matches.is_present("awesome") {
        println!("Awesomeness is turned on");
    }

    match matches.occurrences_of("awesome") {
        0 => println!("Nothing is awesome"),
        1 => println!("Some things are awesome"),
        2 => println!("Lots of things are awesome"),
        3 | _ => println!("EVERYTHING is awesome")
    }
}