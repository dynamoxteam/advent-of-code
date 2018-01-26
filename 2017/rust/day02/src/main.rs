use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChecksumType {
    MinMaxDiff,
    EvenDivision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChecksumError {
    InvalidCell,
}

impl ToString for ChecksumError {
    fn to_string(&self) -> String {
        match self {
            &ChecksumError::InvalidCell => {
                String::from("One of the cells could not be converted into a value.")
            }
        }
    }
}

fn solve_checksum(input: &str, checksum_type: ChecksumType) -> Result<isize, ChecksumError> {
    let mut checksum = 0;
    let mut row = Vec::new();

    for line in input.lines() {
        for cell in line.split('\t') {
            let value = cell.parse::<isize>();

            match value {
                Ok(value) => row.push(value),
                Err(_) => return Err(ChecksumError::InvalidCell),
            };
        }

        checksum += match checksum_type {
            ChecksumType::MinMaxDiff => calculate_min_max_diff(row.iter()),
            ChecksumType::EvenDivision => calculate_even_division(row.iter()),
        };

        row.clear();
    }

    Ok(checksum)
}

fn calculate_min_max_diff<'a, I>(row: I) -> isize
where
    I: Iterator<Item = &'a isize> + Clone,
{
    let max = row.clone().max();
    let min = row.clone().min();

    if let (Some(max), Some(min)) = (max, min) {
        max - min
    } else {
        0
    }
}

fn calculate_even_division<'a, I>(row: I) -> isize
where
    I: Iterator<Item = &'a isize> + Clone,
{
    for dividend in row.clone() {
        for divisor in row.clone() {
            if divisor as *const isize == dividend as *const isize {
                continue;
            }

            let result = (
                dividend.checked_div(*divisor),
                dividend.checked_rem(*divisor),
            );

            if let (Some(quotient), Some(0)) = result {
                return quotient;
            }
        }
    }

    0
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day02 <input file>");
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

    match solve_checksum(input.as_str(), ChecksumType::MinMaxDiff) {
        Ok(value) => println!("Min-max difference checksum: {}", value),
        Err(error) => {
            println!("{}", error.to_string());
            return;
        }
    };

    match solve_checksum(input.as_str(), ChecksumType::EvenDivision) {
        Ok(value) => println!("Even division checksum: {}", value),
        Err(error) => {
            println!("{}", error.to_string());
            return;
        }
    };
}

#[test]
fn test_min_max_diff() {
    assert_eq!(
        solve_checksum("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8", ChecksumType::MinMaxDiff),
        Ok(18)
    );

    assert_eq!(solve_checksum("", ChecksumType::MinMaxDiff), Ok(0));

    assert_eq!(
        solve_checksum("01a34", ChecksumType::MinMaxDiff),
        Err(ChecksumError::InvalidCell)
    );
}

#[test]
fn test_even_division() {
    assert_eq!(
        solve_checksum(
            "5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5",
            ChecksumType::EvenDivision
        ),
        Ok(9)
    );

    assert_eq!(solve_checksum("1\t1", ChecksumType::EvenDivision), Ok(1));

    assert_eq!(solve_checksum("", ChecksumType::EvenDivision), Ok(0));

    assert_eq!(
        solve_checksum("01a34", ChecksumType::EvenDivision),
        Err(ChecksumError::InvalidCell)
    );
}
