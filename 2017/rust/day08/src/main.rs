extern crate regex;

use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

fn execute_instructions(input: &str) -> (isize, isize) {
    let instruction_re = Regex::new(
        r"(?m)^([^\s]+) +([[:alpha:]]+) +([^\s]+) +if +([^\s]+) +([^\s]+) +([^\s]+)$",
    ).unwrap();

    let mut registers = HashMap::<&str, isize>::new();
    let mut max = 0;

    for cap in instruction_re.captures_iter(input) {
        let register = cap.get(1).unwrap().as_str();
        let instruction = cap.get(2).unwrap().as_str();
        let value = cap[3].parse::<isize>().unwrap();
        let cond_register = cap.get(4).unwrap().as_str();
        let cond_op = cap.get(5).unwrap().as_str();
        let cond_value = cap[6].parse::<isize>().unwrap();

        if !registers.contains_key(register) {
            registers.insert(register, 0);
        }
        if !registers.contains_key(cond_register) {
            registers.insert(cond_register, 0);
        }

        let cond_reg_val = *registers.get(cond_register).unwrap();

        let cond_true = match cond_op {
            ">" => cond_reg_val > cond_value,
            "<" => cond_reg_val < cond_value,
            ">=" => cond_reg_val >= cond_value,
            "<=" => cond_reg_val <= cond_value,
            "==" => cond_reg_val == cond_value,
            "!=" => cond_reg_val != cond_value,
            _ => false,
        };

        if !cond_true {
            continue;
        }

        let register = registers.get_mut(register).unwrap();

        match instruction {
            "inc" => *register += value,
            "dec" => *register -= value,
            _ => (),
        }

        if *register > max {
            max = *register;
        }
    }

    (*registers.values().max().unwrap(), max)
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day08 <input file>");
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

    let (final_max, process_max) = execute_instructions(input.as_str());

    println!("Largest final value: {}", final_max);
    println!("Largest value in process: {}", process_max);
}

#[test]
fn test() {
    assert_eq!(
        execute_instructions(
            "b inc 5 if a > 1\n\
             a inc 1 if b < 5\n\
             c dec -10 if a >= 1\n\
             c inc -20 if c == 10\n"
        ),
        (1, 10)
    );
}
