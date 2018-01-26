use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InstructionType {
    Strange,
    EvenStranger,
}

fn find_exit(input: &str, instruction_type: InstructionType) -> usize {
    let mut jumps: Vec<isize> = input.trim().lines().filter_map(|line| line.parse::<isize>().ok()).collect();

    if jumps.len() == 0 { return 0; }

    let mut pc = 0;

    let update_instruction: fn(&mut isize) = match instruction_type {
        InstructionType::Strange => |j: &mut isize| *j = *j + 1,
        InstructionType::EvenStranger => |j: &mut isize| *j = *j + if *j < 3 { 1 } else { -1 },
    };

    for steps in 0.. {
        let instruction = jumps.get_mut(pc as usize);

        if instruction == None { return steps; }

        let jump = instruction.unwrap();

        pc = pc + *jump;
        update_instruction(jump);
    }

    0
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day05 <input file>");
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

    println!("Strange steps to the exit: {}", find_exit(input.as_str(), InstructionType::Strange));
    println!("Even stranger steps to the exit: {}", find_exit(input.as_str(), InstructionType::EvenStranger));
}

#[test]
fn test_strange() {
    assert_eq!(find_exit("0\n3\n0\n1\n-3\n", InstructionType::Strange), 5);
}

#[test]
fn test_even_stranger() {
    assert_eq!(find_exit("0\n3\n0\n1\n-3\n", InstructionType::EvenStranger), 10);
}
