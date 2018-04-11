extern crate regex;

use std::env;
use std::fs::File;
use std::io::Read;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
struct Layer {
    depth: usize,
    range: usize,
}

impl Layer {
    fn get_severity(&self) -> usize {
        self.depth * self.range
    }

    fn scan(&self, delay: usize) -> bool {
        (self.depth + delay) % (2 * (self.range - 1)) == 0
    }
}

fn parse_layers(input: &str) -> Vec<Layer> {
    let layer_regex = Regex::new(r"(?m)^(?P<depth>[0-9]+) *: +(?P<range>[0-9]+)$").unwrap();

    let mut layers = Vec::<Layer>::new();

    for layer in layer_regex.captures_iter(input) {
        let depth = layer
            .name("depth")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let range = layer
            .name("range")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        layers.push(Layer { depth, range });
    }

    layers
}

fn calculate_severity<'a, I>(layers: I, delay: usize) -> usize
where
    I: Iterator<Item = &'a Layer>,
{
    let mut severity = 0;

    for layer in layers {
        if layer.scan(delay) {
            severity += layer.get_severity();
        }
    }

    severity
}

fn is_caught<'a, I>(layers: I, delay: usize) -> bool
where
    I: Iterator<Item = &'a Layer>,
{
    for layer in layers {
        if layer.scan(delay) {
            return true;
        }
    }

    false
}

fn search_smallest_delay<'a, I>(layers: I) -> usize
where
    I: Clone + Iterator<Item = &'a Layer>,
{
    let mut delay = 0;

    loop {
        if !is_caught(layers.clone(), delay) {
            break delay;
        }

        delay += 1;
    }
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day13 <input file>");
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

    let layers = parse_layers(input.as_str());

    println!("Trip severity: {}", calculate_severity(layers.iter(), 0));
    println!("Smallest delay: {}", search_smallest_delay(layers.iter()));
}

#[cfg(test)]
fn build_test_layers() -> Vec<Layer> {
    parse_layers(
        "0: 3\n\
         1: 2\n\
         4: 4\n\
         6: 4\n",
    )
}

#[test]
fn test_severity() {
    assert_eq!(calculate_severity(build_test_layers().iter(), 0), 24);
}

#[test]
fn test_delay() {
    assert_eq!(search_smallest_delay(build_test_layers().iter()), 10);
}
