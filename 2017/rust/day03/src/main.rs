extern crate common;

use std::cmp::max;

fn calculate_position(input: u32) -> (i32, i32) {
    let side = {
        let ceil_sqrt = (input as f64).sqrt().ceil() as u32;

        ceil_sqrt + if ceil_sqrt % 2 == 0 { 1 } else { 0 }
    };

    if side == 1 {
        return (0, 0);
    }

    let base = (side - 2).pow(2);
    let arm_len = side - 1;
    let base_offset = input - base;
    let arm = (base_offset - 1) / arm_len;

    let half_arm_len = (arm_len / 2) as i32;
    let arm_offset = (base_offset - arm * arm_len) as i32;

    match arm {
        0 => (half_arm_len, half_arm_len - arm_offset),
        1 => (half_arm_len - arm_offset, -half_arm_len),
        2 => (-half_arm_len, arm_offset - half_arm_len),
        3 => (arm_offset - half_arm_len, half_arm_len),
        _ => panic!("Arm must be in the interval [0, 3]."),
    }
}

fn find_shortest_path(input: u32) -> u32 {
    let (x, y) = calculate_position(input);

    x.abs() as u32 + y.abs() as u32
}

fn calculate_address(position: (i32, i32)) -> u32 {
    if position == (0, 0) {
        return 1;
    }

    let (x, y) = position;
    let half_arm_len = max(x.abs(), y.abs());
    let arm_len = 2 * half_arm_len;

    let (arm, arm_offset) = {
        if x == half_arm_len && y < half_arm_len {
            (0, half_arm_len - y)
        } else if y == -half_arm_len && x < half_arm_len {
            (1, half_arm_len - x)
        } else if x == -half_arm_len && y > -half_arm_len {
            (2, half_arm_len + y)
        } else {
            (3, half_arm_len + x)
        }
    };

    let base_offset = (arm * arm_len + arm_offset) as u32;
    let base = (arm_len as u32 - 1).pow(2);

    base + base_offset
}

fn calculate_adjacents_sum(input: u32, buffer: &mut Vec<u32>) -> u32 {
    if input == 0 {
        return 0;
    }
    if buffer.len() == 0 {
        buffer.push(1);
    }

    for index in (buffer.len() as u32)..input {
        let address = index + 1;
        let (x, y) = calculate_position(address);

        let mut sum = 0;

        let mut get_neighbor_value = |buffer: &Vec<u32>, x, y| -> u32 {
            let neighbor_address = calculate_address((x, y));
            let neighbor_index = neighbor_address as usize - 1;

            if neighbor_address < address {
                buffer[neighbor_index]
            } else {
                0
            }
        };

        sum += get_neighbor_value(buffer, x + 1, y); // →
        sum += get_neighbor_value(buffer, x + 1, y - 1); // ↗
        sum += get_neighbor_value(buffer, x, y - 1); // ↑
        sum += get_neighbor_value(buffer, x - 1, y - 1); // ↖
        sum += get_neighbor_value(buffer, x - 1, y); // ←
        sum += get_neighbor_value(buffer, x - 1, y + 1); // ↙
        sum += get_neighbor_value(buffer, x, y + 1); // ↓
        sum += get_neighbor_value(buffer, x + 1, y + 1); // ↘

        buffer.push(sum);
    }

    *buffer.last().unwrap()
}

fn find_first_larger(input: u32) -> u32 {
    let mut buffer = Vec::new();

    for address in 1.. {
        let sum = calculate_adjacents_sum(address, &mut buffer);

        if sum > input {
            return sum;
        }
    }

    0
}

fn main() {
    let input: u32 = common::load_input("day03");

    println!("Shortest path: {}", find_shortest_path(input));
    println!("First larger: {}", find_first_larger(input));
}

#[test]
fn test_position() {
    assert_eq!(calculate_position(1), (0, 0));
    assert_eq!(calculate_position(2), (1, 0));
    assert_eq!(calculate_position(3), (1, -1));
    assert_eq!(calculate_position(4), (0, -1));
    assert_eq!(calculate_position(5), (-1, -1));
    assert_eq!(calculate_position(6), (-1, 0));
    assert_eq!(calculate_position(7), (-1, 1));
    assert_eq!(calculate_position(8), (0, 1));
    assert_eq!(calculate_position(9), (1, 1));
    assert_eq!(calculate_position(10), (2, 1));
    assert_eq!(calculate_position(11), (2, 0));
    assert_eq!(calculate_position(12), (2, -1));
    assert_eq!(calculate_position(13), (2, -2));
    assert_eq!(calculate_position(14), (1, -2));
    assert_eq!(calculate_position(15), (0, -2));
    assert_eq!(calculate_position(16), (-1, -2));
    assert_eq!(calculate_position(17), (-2, -2));
    assert_eq!(calculate_position(18), (-2, -1));
    assert_eq!(calculate_position(19), (-2, 0));
    assert_eq!(calculate_position(20), (-2, 1));
    assert_eq!(calculate_position(21), (-2, 2));
    assert_eq!(calculate_position(22), (-1, 2));
    assert_eq!(calculate_position(23), (0, 2));
    assert_eq!(calculate_position(24), (1, 2));
    assert_eq!(calculate_position(25), (2, 2));
}

#[test]
fn test_shortest_path() {
    assert_eq!(find_shortest_path(1), 0);
    assert_eq!(find_shortest_path(12), 3);
    assert_eq!(find_shortest_path(23), 2);
    assert_eq!(find_shortest_path(1024), 31);
}

#[test]
fn test_address() {
    assert_eq!(calculate_address((0, 0)), 1);
    assert_eq!(calculate_address((1, 0)), 2);
    assert_eq!(calculate_address((1, -1)), 3);
    assert_eq!(calculate_address((0, -1)), 4);
    assert_eq!(calculate_address((-1, -1)), 5);
    assert_eq!(calculate_address((-1, 0)), 6);
    assert_eq!(calculate_address((-1, 1)), 7);
    assert_eq!(calculate_address((0, 1)), 8);
    assert_eq!(calculate_address((1, 1)), 9);
    assert_eq!(calculate_address((2, 1)), 10);
    assert_eq!(calculate_address((2, 0)), 11);
    assert_eq!(calculate_address((2, -1)), 12);
    assert_eq!(calculate_address((2, -2)), 13);
    assert_eq!(calculate_address((1, -2)), 14);
    assert_eq!(calculate_address((0, -2)), 15);
    assert_eq!(calculate_address((-1, -2)), 16);
    assert_eq!(calculate_address((-2, -2)), 17);
    assert_eq!(calculate_address((-2, -1)), 18);
    assert_eq!(calculate_address((-2, 0)), 19);
    assert_eq!(calculate_address((-2, 1)), 20);
    assert_eq!(calculate_address((-2, 2)), 21);
    assert_eq!(calculate_address((-1, 2)), 22);
    assert_eq!(calculate_address((0, 2)), 23);
    assert_eq!(calculate_address((1, 2)), 24);
    assert_eq!(calculate_address((2, 2)), 25);
}

#[test]
fn test_adjacents_sum() {
    let mut buffer = Vec::new();

    assert_eq!(calculate_adjacents_sum(1, &mut buffer), 1);
    assert_eq!(calculate_adjacents_sum(2, &mut buffer), 1);
    assert_eq!(calculate_adjacents_sum(3, &mut buffer), 2);
    assert_eq!(calculate_adjacents_sum(4, &mut buffer), 4);
    assert_eq!(calculate_adjacents_sum(5, &mut buffer), 5);
    assert_eq!(calculate_adjacents_sum(6, &mut buffer), 10);
    assert_eq!(calculate_adjacents_sum(7, &mut buffer), 11);
    assert_eq!(calculate_adjacents_sum(8, &mut buffer), 23);
    assert_eq!(calculate_adjacents_sum(9, &mut buffer), 25);
    assert_eq!(calculate_adjacents_sum(10, &mut buffer), 26);
    assert_eq!(calculate_adjacents_sum(11, &mut buffer), 54);
    assert_eq!(calculate_adjacents_sum(12, &mut buffer), 57);
    assert_eq!(calculate_adjacents_sum(13, &mut buffer), 59);
    assert_eq!(calculate_adjacents_sum(14, &mut buffer), 122);
    assert_eq!(calculate_adjacents_sum(15, &mut buffer), 133);
    assert_eq!(calculate_adjacents_sum(16, &mut buffer), 142);
    assert_eq!(calculate_adjacents_sum(17, &mut buffer), 147);
    assert_eq!(calculate_adjacents_sum(18, &mut buffer), 304);
    assert_eq!(calculate_adjacents_sum(19, &mut buffer), 330);
    assert_eq!(calculate_adjacents_sum(20, &mut buffer), 351);
    assert_eq!(calculate_adjacents_sum(21, &mut buffer), 362);
    assert_eq!(calculate_adjacents_sum(22, &mut buffer), 747);
    assert_eq!(calculate_adjacents_sum(23, &mut buffer), 806);
}

#[test]
fn test_first_larger() {
    assert_eq!(find_first_larger(0), 1);
    assert_eq!(find_first_larger(1), 2);
    assert_eq!(find_first_larger(2), 4);
    assert_eq!(find_first_larger(4), 5);
    assert_eq!(find_first_larger(5), 10);
    assert_eq!(find_first_larger(10), 11);
}
