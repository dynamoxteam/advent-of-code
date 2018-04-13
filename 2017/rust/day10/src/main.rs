extern crate day10;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day10 <input file>");
        return;
    }

    let file = File::open(arg.unwrap());

    if let Err(error) = file {
        println!("{}", error.to_string());
        return;
    }

    let mut input = String::new();

    if let Err(error) = file.unwrap().read_to_string(&mut input) {
        println!("{}", error.to_string());
        return;
    }

    println!(
        "Simple Hash: {}",
        day10::calculate_simple_hash(input.as_str(), 256)
    );

    println!("Knot Hash: {}", day10::calculate_knot_hash(input.as_str()));
}
