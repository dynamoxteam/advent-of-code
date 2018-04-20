extern crate common;

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PassphraseSecurity {
    Basic,
    Extended,
}

fn is_basic_passphrase_valid(input: &str) -> bool {
    let words = input.split_whitespace();

    for (index, word) in words.clone().enumerate() {
        if words.clone().skip(index + 1).any(|w| word == w) {
            return false;
        }
    }

    true
}

fn is_extended_passphrase_valid(input: &str) -> bool {
    let words = input.split_whitespace();

    let mut word_maps = Vec::new();

    for word in words.clone() {
        let mut map = HashMap::<char, u16>::new();

        word.chars().for_each(|c| {
            let count = *map.get(&c).unwrap_or(&0);

            map.insert(c, count + 1);
        });

        word_maps.push(map);
    }

    for (index, map) in word_maps.iter().enumerate() {
        if word_maps.iter().skip(index + 1).any(|m| map == m) {
            return false;
        }
    }

    true
}

fn count_valid_passphrases(input: &str, security: PassphraseSecurity) -> usize {
    let is_passphrase_valid = match security {
        PassphraseSecurity::Basic => is_basic_passphrase_valid,
        PassphraseSecurity::Extended => is_extended_passphrase_valid,
    };

    input.lines().filter(|s| is_passphrase_valid(s)).count()
}

fn main() {
    let input = common::load_file_input("day04");

    println!(
        "Basic security: {}",
        count_valid_passphrases(input.as_str(), PassphraseSecurity::Basic)
    );

    println!(
        "Extended security: {}",
        count_valid_passphrases(input.as_str(), PassphraseSecurity::Extended)
    );
}

#[test]
fn test_basic_validation() {
    assert_eq!(is_basic_passphrase_valid("aa bb cc dd ee"), true);
    assert_eq!(is_basic_passphrase_valid("aa bb cc dd aa"), false);
    assert_eq!(is_basic_passphrase_valid("aa bb cc dd aaa"), true);
}

#[test]
fn test_basic_count() {
    assert_eq!(
        count_valid_passphrases(
            "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa",
            PassphraseSecurity::Basic
        ),
        2
    );
}

#[test]
fn test_extended_validation() {
    assert_eq!(is_extended_passphrase_valid("abcde fghij"), true);
    assert_eq!(is_extended_passphrase_valid("abcde xyz ecdab"), false);
    assert_eq!(is_extended_passphrase_valid("a ab abc abd abf abj"), true);

    assert_eq!(
        is_extended_passphrase_valid("iiii oiii ooii oooi oooo"),
        true
    );

    assert_eq!(is_extended_passphrase_valid("oiii ioii iioi iiio"), false);
}

#[test]
fn test_extended_count() {
    assert_eq!(
        count_valid_passphrases(
            "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio",
            PassphraseSecurity::Extended
        ),
        3
    );
}
