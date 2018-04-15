use std::env;
use std::fs::File;
use std::io::Read;

fn build_programs(number: usize) -> String {
    "abcdefghijklmnopqrstuvwxyz".chars().take(number).collect()
}

fn dance(programs: &mut String, input: &str) {
    for mov in input.trim().split(',') {
        let (mov_type, params) = mov.split_at(1);

        match mov_type {
            "s" => spin(programs, params),
            "x" => exchange(programs, params),
            "p" => partner(programs, params),
            _ => (),
        }
    }
}

fn spin(programs: &mut String, params: &str) {
    let steps = params.parse::<usize>().unwrap();

    unsafe {
        programs.as_bytes_mut().rotate_right(steps);
    }
}

fn exchange(programs: &mut String, params: &str) {
    let mut indexes = params.split('/').map(|n| n.parse::<usize>().unwrap());
    let indexes = (indexes.next().unwrap(), indexes.next().unwrap());

    unsafe {
        programs.as_bytes_mut().swap(indexes.0, indexes.1);
    }
}

fn partner(programs: &mut String, params: &str) {
    let indexes = {
        let mut indexes = params.split('/').map(|n| programs.find(n).unwrap());

        (indexes.next().unwrap(), indexes.next().unwrap())
    };

    unsafe {
        programs.as_bytes_mut().swap(indexes.0, indexes.1);
    }
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day16 <input file>");
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

    let mut programs = build_programs(16);

    dance(&mut programs, input.as_str());
    println!("Programs after dance: {}", programs);
}

#[test]
fn test_spin() {
    let mut programs = String::from("abcde");

    spin(&mut programs, "3");
    assert_eq!(programs, "cdeab");
}

#[test]
fn test_moves() {
    let mut programs = String::from("abcde");

    dance(&mut programs, "s1,x3/4,pe/b");
    assert_eq!(programs, "baedc");
}
