use std::env;

fn compute_buffer(input: usize, elements: usize) -> (usize, Vec<usize>) {
    let mut buffer = Vec::<usize>::with_capacity(elements);
    let mut pos = 0;

    buffer.push(0);

    for n in 1..elements {
        pos = (pos + input) % buffer.len() + 1;
        buffer.insert(pos, n);

        if n % 10000 == 0 {
            println!("{}", n);
        }
    }

    (pos, buffer)
}

fn compute_next_value(input: usize, elements: usize) -> usize {
    let (mut pos, buffer) = compute_buffer(input, elements);

    pos = (pos + 1) % buffer.len();
    buffer[pos]
}

fn compute_value_after_0(input: usize, elements: usize) -> usize {
    let (_, buffer) = compute_buffer(input, elements);
    let mut pos = buffer.iter().position(|&n| n == 0).unwrap();

    pos = (pos + 1) % buffer.len();
    buffer[pos]
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day17 <input>");
        return;
    }

    let input = arg.unwrap().parse::<usize>().unwrap();

    println!(
        "Next value after the last: {}",
        compute_next_value(input, 2018)
    );
    
    println!(
        "Next value after 0: {}",
        compute_value_after_0(input, 50_000_000)
    );
}

#[test]
fn test_buffer() {
    let (pos, buffer) = compute_buffer(3, 2018);

    assert_eq!(buffer[pos - 3], 1512);
    assert_eq!(buffer[pos - 2], 1134);
    assert_eq!(buffer[pos - 1], 151);
    assert_eq!(buffer[pos], 2017);
    assert_eq!(buffer[pos + 1], 638);
    assert_eq!(buffer[pos + 2], 1513);
    assert_eq!(buffer[pos + 3], 851);
}

#[test]
fn test_next_value() {
    assert_eq!(compute_next_value(3, 1), 0);
    assert_eq!(compute_next_value(3, 2), 0);
    assert_eq!(compute_next_value(3, 3), 1);
    assert_eq!(compute_next_value(3, 4), 1);
    assert_eq!(compute_next_value(3, 5), 3);
    assert_eq!(compute_next_value(3, 6), 2);
    assert_eq!(compute_next_value(3, 7), 1);
    assert_eq!(compute_next_value(3, 8), 2);
    assert_eq!(compute_next_value(3, 9), 6);
    assert_eq!(compute_next_value(3, 10), 5);
}

#[test]
fn test_after_0() {
    assert_eq!(compute_value_after_0(3, 1), 0);
    assert_eq!(compute_value_after_0(3, 2), 1);
    assert_eq!(compute_value_after_0(3, 3), 2);
    assert_eq!(compute_value_after_0(3, 4), 2);
    assert_eq!(compute_value_after_0(3, 5), 2);
    assert_eq!(compute_value_after_0(3, 6), 5);
    assert_eq!(compute_value_after_0(3, 7), 5);
    assert_eq!(compute_value_after_0(3, 8), 5);
    assert_eq!(compute_value_after_0(3, 9), 5);
    assert_eq!(compute_value_after_0(3, 10), 9);
}
