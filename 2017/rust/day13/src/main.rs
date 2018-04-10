extern crate regex;

use std::env;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn calculate_severity(input: &str) -> usize {
    let layer_regex = Regex::new(r"(?m)^(?P<depth>[0-9]+) *: +(?P<range>[0-9]+)$").unwrap();

    let mut severity = 0;

    for layer in layer_regex.captures_iter(input) {
        let depth = layer
            .name("depth")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let range = layer
            .name("range")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        if depth % (2 * (range - 1)) == 0 {
            severity += depth * range;
        }
    }

    severity
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day13 <input file>");
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

    println!("Trip severity: {}", calculate_severity(input.as_str()));
}

#[test]
fn test() {
    assert_eq!(
        calculate_severity(
            "0: 3\n\
             1: 2\n\
             4: 4\n\
             6: 4\n"
        ),
        24
    );
}
