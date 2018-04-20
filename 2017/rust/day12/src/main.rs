extern crate common;
extern crate regex;

use regex::Regex;

fn analyze_pipes(input: &str) -> (usize, usize) {
    let record_regex = Regex::new(r"(?m)^(?P<id>[0-9]+) +<\-> +(?P<pipes>[0-9, ]+)\r?$").unwrap();
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

        let group: usize = programs.iter().cloned().map(|p| groups[p]).min().unwrap();

        for program in programs {
            redirect(&mut groups, program, group);
        }
    }

    for program in 0..total_programs {
        let group = groups[program];

        groups[program] = groups[group];
    }

    let group_0_len = groups.iter().cloned().filter(|&g| g == 0).count();
    
    let total_groups = groups
        .iter()
        .cloned()
        .enumerate()
        .filter(|&(p, g)| p == g)
        .count();

    (group_0_len, total_groups)
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
    let input = common::load_file_input("day12");
    let (group_0_len, total_groups) = analyze_pipes(input.as_str());

    println!("Programs connected to ID 0: {}", group_0_len);
    println!("Number of groups: {}", total_groups);
}

#[test]
fn test() {
    assert_eq!(
        analyze_pipes(
            "0 <-> 2\n\
             1 <-> 1\n\
             2 <-> 0, 3, 4\n\
             3 <-> 2, 4\n\
             4 <-> 2, 3, 6\n\
             5 <-> 6\n\
             6 <-> 4, 5\n"
        ),
        (6, 2)
    );

    assert_eq!(
        analyze_pipes(
            "0 <-> 5\n\
             1 <-> 2\n\
             2 <-> 1, 3\n\
             3 <-> 2, 4\n\
             4 <-> 3, 5\n\
             5 <-> 0, 4\n"
        ),
        (6, 1)
    );
}
