use std::env;
use std::fmt;

enum MyErr {
    Reason1(String),
    Reason2(String, u32),
}

impl fmt::Display for MyErr { 
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match *self {
            MyErr::Reason1(ref s) => 
                write!(f, "`{}` is the error", s),
            MyErr::Reason2(ref s, ref num) => 
                write!(f, "`{}` and `{}` are error", s, num),
        }
    }
}

fn foo() -> Result<(), MyErr> {

    Err(MyErr::Reason1("foo".to_string()))
}

fn main() {
    let key = "JAVA_HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key)
    };

    match foo() {
        Ok(()) => println!("ok"),
        Err(e) => println!("{}", e)
    };
}