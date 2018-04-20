extern crate common;
extern crate day10;

fn main() {
    let input = common::load_file_input("day10");

    println!(
        "Simple Hash: {}",
        day10::calculate_simple_hash(input.as_str(), 256)
    );

    println!("Knot Hash: {}", day10::calculate_knot_hash(input.as_str()));
}
