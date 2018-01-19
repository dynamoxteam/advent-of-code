use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CaptchaType {
    NextDigit,
    HalfwayAround,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CaptchaError {
    InvalidChar,
}

impl ToString for CaptchaError {
    fn to_string(&self) -> String {
        match self {
            &CaptchaError::InvalidChar => String::from("Invalid character in the input."),
        }
    }
}

fn solve_captcha(input: &str, captcha_type: CaptchaType) -> Result<u32, CaptchaError> {
    let digits = input.chars().map((|c| c.to_digit(10)) as fn(char) -> Option<u32>);
    
    let digits_skipped = match captcha_type {
        CaptchaType::NextDigit => 1,
        CaptchaType::HalfwayAround => digits.clone().count() / 2,
    };

    let cycle = digits.clone().cycle().skip(digits_skipped);

    let mut sum = 0;

    for (digit, next) in digits.zip(cycle) {
        match digit {
            None => return Err(CaptchaError::InvalidChar),
            Some(value) => if digit == next { sum += value },
        };
    }

    Ok(sum)
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day01-part2 <input>");
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

    match solve_captcha(input.as_str(), CaptchaType::NextDigit) {
        Ok(value) => println!("Next digit captcha: {}", value),
        Err(error) => {
            println!("{}", error.to_string());
            return;
        }
    };

    match solve_captcha(input.as_str(), CaptchaType::HalfwayAround) {
        Ok(value) => println!("Halfway around captcha: {}", value),
        Err(error) => {
            println!("{}", error.to_string());
            return;
        }
    };
}

#[test]
fn test_next_digit() {
    assert_eq!(solve_captcha("1122", CaptchaType::NextDigit), Ok(3));
    assert_eq!(solve_captcha("1111", CaptchaType::NextDigit), Ok(4));
    assert_eq!(solve_captcha("1234", CaptchaType::NextDigit), Ok(0));
    assert_eq!(solve_captcha("91212129", CaptchaType::NextDigit), Ok(9));

    assert_eq!(solve_captcha("", CaptchaType::NextDigit), Ok(0));
    assert_eq!(solve_captcha("01a34", CaptchaType::NextDigit), Err(CaptchaError::InvalidChar));
}

#[test]
fn test_halfway_around() {
    assert_eq!(solve_captcha("1212", CaptchaType::HalfwayAround), Ok(6));
    assert_eq!(solve_captcha("1221", CaptchaType::HalfwayAround), Ok(0));
    assert_eq!(solve_captcha("123425", CaptchaType::HalfwayAround), Ok(4));
    assert_eq!(solve_captcha("123123", CaptchaType::HalfwayAround), Ok(12));
    assert_eq!(solve_captcha("12131415", CaptchaType::HalfwayAround), Ok(4));

    assert_eq!(solve_captcha("", CaptchaType::HalfwayAround), Ok(0));
    assert_eq!(solve_captcha("01a34", CaptchaType::HalfwayAround), Err(CaptchaError::InvalidChar));
}