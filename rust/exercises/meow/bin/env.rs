use std::env;

fn main() {
    let key = "JAVA_HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key)
    };
}