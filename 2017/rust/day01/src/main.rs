extern crate common;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CaptchaType {
    NextDigit,
    HalfwayAround,
}

fn solve_captcha(input: &str, captcha_type: CaptchaType) -> u32 {
    let digits = input
        .chars()
        .filter_map((|c| c.to_digit(10)) as fn(char) -> Option<u32>);

    let digits_skipped = match captcha_type {
        CaptchaType::NextDigit => 1,
        CaptchaType::HalfwayAround => digits.clone().count() / 2,
    };

    let cycle = digits.clone().cycle().skip(digits_skipped);

    digits
        .zip(cycle)
        .filter(|&(d, n)| d == n)
        .map(|(d, _)| d)
        .sum()
}

fn main() {
    let input = common::load_file_input("day01");

    println!(
        "Next digit captcha: {}",
        solve_captcha(input.as_str(), CaptchaType::NextDigit)
    );

    println!(
        "Halfway around captcha: {}",
        solve_captcha(input.as_str(), CaptchaType::HalfwayAround)
    );
}

#[test]
fn test_next_digit() {
    assert_eq!(solve_captcha("1122", CaptchaType::NextDigit), 3);
    assert_eq!(solve_captcha("1111", CaptchaType::NextDigit), 4);
    assert_eq!(solve_captcha("1234", CaptchaType::NextDigit), 0);
    assert_eq!(solve_captcha("91212129", CaptchaType::NextDigit), 9);

    assert_eq!(solve_captcha("", CaptchaType::NextDigit), 0);
}

#[test]
fn test_halfway_around() {
    assert_eq!(solve_captcha("1212", CaptchaType::HalfwayAround), 6);
    assert_eq!(solve_captcha("1221", CaptchaType::HalfwayAround), 0);
    assert_eq!(solve_captcha("123425", CaptchaType::HalfwayAround), 4);
    assert_eq!(solve_captcha("123123", CaptchaType::HalfwayAround), 12);
    assert_eq!(solve_captcha("12131415", CaptchaType::HalfwayAround), 4);

    assert_eq!(solve_captcha("", CaptchaType::HalfwayAround), 0);
}
