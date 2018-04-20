use std::env;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::process;
use std::str::FromStr;

pub fn load_input_str(app_name: &str) -> String {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: {} <input>", app_name);
        process::exit(-1);
    }

    arg.unwrap()
}

pub fn load_input<E: Debug + Display, T: FromStr<Err = E>>(app_name: &str) -> T {
    let result = load_input_str(app_name).parse::<T>();

    if let Err(error) = result {
        println!("{}", error);
        process::exit(-1);
    }

    result.unwrap()
}

pub fn load_file_input(app_name: &str) -> String {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: {} <input file>", app_name);
        process::exit(-1);
    }

    let file = File::open(arg.unwrap());

    if let Err(error) = file {
        println!("{}", error);
        process::exit(-1);
    }

    let mut input = String::new();

    if let Err(error) = file.unwrap().read_to_string(&mut input) {
        println!("{}", error);
        process::exit(-1);
    }

    input
}
