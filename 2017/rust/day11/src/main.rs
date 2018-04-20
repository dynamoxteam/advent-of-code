extern crate common;

use std::ops::Add;
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        *self = *self + other;
    }
}

fn analyze_path(input: &str) -> (usize, usize) {
    let mut pos = Position { x: 0, y: 0 };
    let mut steps = 0;
    let mut max_steps = 0;

    let input = input.trim().split(',');

    for step in input {
        match step {
            "n" => pos += Position { x: 0, y: 2 },
            "ne" => pos += Position { x: 1, y: 1 },
            "se" => pos += Position { x: 1, y: -1 },
            "s" => pos += Position { x: 0, y: -2 },
            "sw" => pos += Position { x: -1, y: -1 },
            "nw" => pos += Position { x: -1, y: 1 },
            _ => (),
        };

        steps = calculate_steps(&pos);

        if steps > max_steps {
            max_steps = steps;
        }
    }

    (steps, max_steps)
}

fn calculate_steps(pos: &Position) -> usize {
    let abs_x = pos.x.abs() as usize;
    let abs_y = pos.y.abs() as usize;

    if abs_x >= abs_y {
        abs_x
    } else {
        (abs_x + abs_y) / 2
    }
}

fn main() {
    let input = common::load_file_input("day11");
    let (steps, max_steps) = analyze_path(input.as_str());

    println!("Fewest steps: {}", steps);
    println!("Furthest steps: {}", max_steps);
}

#[test]
fn test() {
    assert_eq!(analyze_path("ne,ne,ne"), (3, 3));
    assert_eq!(analyze_path("ne,ne,sw,sw"), (0, 2));
    assert_eq!(analyze_path("ne,ne,s,s"), (2, 2));
    assert_eq!(analyze_path("se,sw,se,sw,sw"), (3, 3));
}
