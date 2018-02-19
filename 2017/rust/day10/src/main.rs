use std::env;
use std::fs::File;
use std::io::Read;

fn calculate_simple_hash(input: &str, list_length: usize) -> usize {
    let mut list = initialize_list(list_length);

    let lengths = input.trim().split(',').map(|l| l.parse::<u8>().unwrap());

    let mut curr_pos = 0;
    let mut skip_size = 0;

    process_round(&mut list, lengths, &mut curr_pos, &mut skip_size);

    list[0] as usize * list[1] as usize
}

fn calculate_knot_hash(input: &str) -> String {
    let mut list = initialize_list(256);

    let suffix = [17, 31, 73, 47, 23];
    let lengths = input.trim().bytes().chain(suffix.iter().cloned());

    let mut curr_pos: u8 = 0;
    let mut skip_size: u8 = 0;

    for _ in 0..64 {
        process_round(&mut list, lengths.clone(), &mut curr_pos, &mut skip_size);
    }

    let hash_len = list.len() / 16;
    let mut dense_hash = Vec::<u8>::with_capacity(hash_len);

    for i in 0..hash_len {
        let byte = list.iter().skip(16 * i).take(16).fold(0, |acc, b| acc ^ b);

        dense_hash.push(byte);
    }

    let mut hex = String::new();

    for byte in dense_hash {
        hex.push_str(format!("{:02x}", byte).as_str());
    }

    hex
}

fn initialize_list(size: usize) -> Vec<u8> {
    let mut list = Vec::<u8>::with_capacity(size);

    for i in 0..size {
        list.push(i as u8);
    }

    list
}

fn process_round<L>(list: &mut Vec<u8>, lengths: L, curr_pos: &mut u8, skip_size: &mut u8)
where
    L: Iterator<Item = u8>,
{
    for len in lengths {
        reverse_section(list, *curr_pos as usize, len as usize);

        let pos_inc = skip_size.wrapping_add(len);

        *curr_pos = curr_pos.wrapping_add(pos_inc);
        *skip_size = skip_size.wrapping_add(1);
    }
}

fn reverse_section(list: &mut Vec<u8>, start: usize, length: usize) {
    for i in 0..length / 2 {
        let index1 = (start + i) % list.len();
        let index2 = (start + length - i - 1) % list.len();

        list.swap(index1, index2);
    }
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day10 <input file>");
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

    println!(
        "Simple Hash: {}",
        calculate_simple_hash(input.as_str(), 256)
    );

    println!("Knot Hash: {}", calculate_knot_hash(input.as_str()));
}

#[test]
fn test_simple_hash() {
    assert_eq!(calculate_simple_hash("3,4,1,5", 5), 12);
}

#[test]
fn test_knot_hash() {
    assert_eq!(calculate_knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");

    assert_eq!(
        calculate_knot_hash("AoC 2017"),
        "33efeb34ea91902bb2f59c9920caa6cd"
    );

    assert_eq!(
        calculate_knot_hash("1,2,3"),
        "3efbe78a8d82f29979031a4aa0b16a9d"
    );

    assert_eq!(
        calculate_knot_hash("1,2,4"),
        "63960835bcdc130f0b66d7ff4f6a5a8e"
    );
}
