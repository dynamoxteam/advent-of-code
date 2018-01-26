use std::env;
use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;

fn reallocate(input: &str) -> (usize, usize) {
    let mut banks: Vec<usize> = input
        .trim()
        .split('\t')
        .filter_map(|s| s.parse().ok())
        .collect();

    if banks.is_empty() {
        return (0, 0);
    }

    let mut history = vec![banks.clone()];

    for cycles in 1.. {
        let (mut index, mut slots) = banks
            .iter()
            .cloned()
            .enumerate()
            .max_by(|&(i1, s1): &(usize, usize), &(i2, s2): &(usize, usize)| {
                let slots_cmp = s1.cmp(&s2);

                if slots_cmp != Ordering::Equal {
                    slots_cmp
                } else {
                    i1.cmp(&i2).reverse()
                }
            })
            .unwrap();

        banks[index] = 0;

        while slots > 0 {
            index = (index + 1) % banks.len();
            banks[index] = banks[index] + 1;
            slots = slots - 1;
        }

        if let Some(position) = history.iter().position(|b| *b == banks) {
            return (cycles, cycles - position);
        } else {
            history.push(banks.clone())
        }
    }

    (0, 0)
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day06 <input file>");
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

    let (cycles, loop_size) = reallocate(input.as_str());

    println!("Redistribution cycles: {}", cycles);
    println!("Loop size: {}", loop_size);
}

#[test]
fn test() {
    assert_eq!(reallocate("0\t2\t7\t0\n"), (5, 4));
}
