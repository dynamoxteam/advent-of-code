extern crate regex;

use std::env;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn search_linked_to_0(input: &str) -> usize {
    let record_regex = Regex::new(r"(?m)^(?P<id>[0-9]+) +<\-> +(?P<pipes>[0-9, ]+)$").unwrap();
    let pipes_regex = Regex::new(r"(?P<id>[0-9]+)").unwrap();

    let total_programs = input.trim().lines().count();
    let mut groups: Vec<usize> = (0..total_programs).collect();

    for record in record_regex.captures_iter(input) {
        let id: usize = record
            .name("id")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let pipes = record.name("pipes").unwrap().as_str();

        let mut pipes: Vec<usize> = pipes_regex
            .captures_iter(pipes)
            .map(|p| p.name("id").unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        let mut programs: Vec<usize> = vec![id];

        programs.append(&mut pipes);

        let group: usize = programs
            .iter()
            .cloned()
            .map(|p| groups[p])
            .min()
            .unwrap();

        for program in programs {
            redirect(&mut groups, program, group);
        }
    }

    for program in 0..total_programs {
        let group = groups[program];

        groups[program] = groups[group];
    }

    groups.iter().cloned().filter(|&g| g == 0).count()
}

fn redirect(groups: &mut Vec<usize>, program: usize, new_group: usize) {
    let mut program = program;
    let mut group = groups[program];
    
    while group != new_group {
        groups[program] = new_group;

        if group == program {
            break;
        }

        program = group;
        group = groups[program];
    }
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day12 <input file>");
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
        "Programs connected to ID 0: {}",
        search_linked_to_0(input.as_str())
    );
}

#[test]
fn test() {
    assert_eq!(
        search_linked_to_0(
            "0 <-> 2\n\
             1 <-> 1\n\
             2 <-> 0, 3, 4\n\
             3 <-> 2, 4\n\
             4 <-> 2, 3, 6\n\
             5 <-> 6\n\
             6 <-> 4, 5\n"
        ),
        6
    );

    assert_eq!(
        search_linked_to_0(
            "0 <-> 5\n\
             1 <-> 2\n\
             2 <-> 1, 3\n\
             3 <-> 2, 4\n\
             4 <-> 3, 5\n\
             5 <-> 0, 4\n"
        ),
        6
    );
}
