extern crate regex;

use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

fn find_bottom_program<'a>(input: &'a str) -> Option<&'a str> {
    let program_regex = Regex::new(r"(?m)^(?P<name>[[:alpha:]]+) +\((?P<weight>\d+)\)(?: +\->(?P<children>.*))?$").unwrap();
    let children_regex = Regex::new(r"(?P<name>[[:alpha:]]+)").unwrap();
    
    let mut programs: HashMap<&'a str, bool> = HashMap::new();

    for program in program_regex.captures_iter(input) {
        let program_name = program.name("name").unwrap().as_str();

        if !programs.contains_key(program_name) { programs.insert(program_name, false); }

        if let Some(children) = program.name("children") {
            for child in children_regex.captures_iter(children.as_str()) {
                programs.insert(child.name("name").unwrap().as_str(), true);
            }
        }
    }

    let bottom_programs = programs.iter().filter((|&(_, held)| !held) as fn(&(&&str, &bool)) -> bool);

    assert_eq!(bottom_programs.clone().count(), 1);

    bottom_programs
        .clone()
        .map(|(program, _)| *program)
        .next()
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day07 <input file>");
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

    println!("Bottom program: {}", find_bottom_program(input.as_str()).unwrap_or("(none)"));
}

#[test]
fn test() {
    assert_eq!(
        find_bottom_program(
            "pbga (66)\n\
            xhth (57)\n\
            ebii (61)\n\
            havc (66)\n\
            ktlj (57)\n\
            fwft (72) -> ktlj, cntj, xhth\n\
            qoyq (66)\n\
            padx (45) -> pbga, havc, qoyq\n\
            tknk (41) -> ugml, padx, fwft\n\
            jptl (61)\n\
            ugml (68) -> gyxo, ebii, jptl\n\
            gyxo (61)\n\
            cntj (57)\n"),
        Some("tknk")
    );
}
