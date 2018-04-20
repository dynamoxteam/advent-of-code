use std::env;

fn compute_buffer(input: usize, elements: usize) -> Vec<usize> {
    let mut buffer = Vec::<usize>::with_capacity(elements);
    let mut pos = 0;

    buffer.push(0);

    for n in 1..elements {
        (0..input).for_each(|_| pos = buffer[pos]);

        let next = buffer[pos];
        
        buffer.push(next);
        buffer[pos] = n;
        pos = n;
    }

    buffer
}

fn compute_next(input: usize, elements: usize, value: usize) -> usize {
    compute_buffer(input, elements)[value]
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
        compute_next(input, 2018, 2017)
    );
    
    println!(
        "Next value after 0: {}",
        compute_next(input, 50_000_000, 0)
    );
}

#[test]
fn test_buffer() {
    let buffer = compute_buffer(3, 2018);

    assert_eq!(buffer[1512], 1134);
    assert_eq!(buffer[1134], 151);
    assert_eq!(buffer[151], 2017);
    assert_eq!(buffer[638], 1513);
    assert_eq!(buffer[1513], 851);
}

#[test]
fn test_after_last() {
    assert_eq!(compute_next(3, 1, 0), 0);
    assert_eq!(compute_next(3, 2, 1), 0);
    assert_eq!(compute_next(3, 3, 2), 1);
    assert_eq!(compute_next(3, 4, 3), 1);
    assert_eq!(compute_next(3, 5, 4), 3);
    assert_eq!(compute_next(3, 6, 5), 2);
    assert_eq!(compute_next(3, 7, 6), 1);
    assert_eq!(compute_next(3, 8, 7), 2);
    assert_eq!(compute_next(3, 9, 8), 6);
    assert_eq!(compute_next(3, 10, 9), 5);
}

#[test]
fn test_after_zero() {
    assert_eq!(compute_next(3, 1, 0), 0);
    assert_eq!(compute_next(3, 2, 0), 1);
    assert_eq!(compute_next(3, 3, 0), 2);
    assert_eq!(compute_next(3, 4, 0), 2);
    assert_eq!(compute_next(3, 5, 0), 2);
    assert_eq!(compute_next(3, 6, 0), 5);
    assert_eq!(compute_next(3, 7, 0), 5);
    assert_eq!(compute_next(3, 8, 0), 5);
    assert_eq!(compute_next(3, 9, 0), 5);
    assert_eq!(compute_next(3, 10, 0), 9);
}
