extern crate common;
extern crate regex;

use std::iter;
use regex::Regex;

fn get_initial_values(input: &str) -> (usize, usize) {
    let val_regex =
        Regex::new(r"(?m)^Generator (?P<generator>[A-Z]?) starts with (?P<value>[0-9]+)$").unwrap();

    let mut values: (usize, usize) = (0, 0);

    for line in val_regex.captures_iter(input) {
        let generator = line.name("generator").unwrap().as_str();

        let value = line.name("value")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        match generator {
            "A" => values.0 = value,
            "B" => values.1 = value,
            _ => (),
        };
    }

    values
}

fn calculate_matches(
    pairs: usize,
    init_values: (usize, usize),
    divisors: Option<(usize, usize)>,
) -> usize {
    const FACTOR_A: usize = 16_807;
    const FACTOR_B: usize = 48_271;
    const MODULO: usize = 2_147_483_647;
    const MASK: usize = 0xFFFF;

    let get_it = |init, factor, divisor| {
        iter::repeat(())
            .scan(init, move |acc, _| {
                *acc = (*acc * factor) % MODULO;
                Some(*acc)
            })
            .filter(move |n| n % divisor == 0)
            .take(pairs)
    };

    let it_a = get_it(init_values.0, FACTOR_A, divisors.unwrap_or((1, 1)).0);
    let it_b = get_it(init_values.1, FACTOR_B, divisors.unwrap_or((1, 1)).1);

    it_a.zip(it_b)
        .filter(|&(a, b)| (a & MASK) == (b & MASK))
        .count()
}

fn main() {
    let input = common::load_file_input("day15");
    let init_values = get_initial_values(input.as_str());

    println!(
        "No criteria match count: {}",
        calculate_matches(40_000_000, init_values, None)
    );

    println!(
        "Criteria match count: {}",
        calculate_matches(5_000_000, init_values, Some((4, 8)))
    );
}

#[test]
fn test_no_criteria() {
    assert_eq!(calculate_matches(40_000_000, (65, 8921), None), 588);
}

#[test]
fn test_criteria() {
    assert_eq!(calculate_matches(5_000_000, (65, 8921), Some((4, 8))), 309);
}
