extern crate common;
extern crate day10;

fn get_hash_input(key: &str, row: usize) -> String {
    format!("{}-{}", key, row)
}

fn calculate_hash(key: &str, row: usize) -> String {
    day10::calculate_knot_hash(get_hash_input(key, row).as_str())
}

fn calculate_used_squares(key: &str) -> usize {
    (0..128)
        .map(|r| {
            calculate_hash(key, r)
                .chars()
                .map(|c| c.to_digit(16).unwrap_or(0).count_ones() as usize)
                .sum::<usize>()
        })
        .sum()
}

fn get_grid(key: &str) -> Vec<Vec<bool>> {
    (0..128)
        .map(|r| {
            calculate_hash(key, r)
                .chars()
                .map(|c| c.to_digit(16).unwrap_or(0))
                .flat_map(|c| (0..4).rev().map(move |i| (c >> i) & 1u32 == 1u32))
                .collect()
        })
        .collect()
}

fn calculate_regions(key: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = get_grid(key);

    let mut regions = 0;

    for i in 0..128 {
        for j in 0..128 {
            regions += inspect_square(&mut grid, i, j);
        }
    }

    regions
}

fn inspect_square(grid: &mut Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    if !grid[i][j] {
        return 0;
    }

    grid[i][j] = false;

    for di in [-1, 1].iter() {
        let ni = (i as isize + di) as usize;

        if grid.get(ni).is_none() {
            continue;
        }

        inspect_square(grid, ni, j);
    }

    for dj in [-1, 1].iter() {
        let nj = (j as isize + dj) as usize;

        if grid[i].get(nj).is_none() {
            continue;
        }

        inspect_square(grid, i, nj);
    }

    1
}

fn main() {
    let input = common::load_input_str("day14");

    println!("Used squares: {}", calculate_used_squares(input.as_str()));
    println!("Regions: {}", calculate_regions(input.as_str()));
}

#[test]
fn test_hash_inputs() {
    assert_eq!(get_hash_input("flqrgnkx", 0), "flqrgnkx-0");
    assert_eq!(get_hash_input("flqrgnkx", 5), "flqrgnkx-5");
    assert_eq!(get_hash_input("flqrgnkx", 10), "flqrgnkx-10");
    assert_eq!(get_hash_input("flqrgnkx", 27), "flqrgnkx-27");
    assert_eq!(get_hash_input("flqrgnkx", 81), "flqrgnkx-81");
    assert_eq!(get_hash_input("flqrgnkx", 100), "flqrgnkx-100");
    assert_eq!(get_hash_input("flqrgnkx", 127), "flqrgnkx-127");
}

#[test]
fn test_individual_squares() {
    assert!(calculate_hash("flqrgnkx", 0).starts_with("d4"));
    assert!(calculate_hash("flqrgnkx", 1).starts_with("55"));
    assert!(calculate_hash("flqrgnkx", 2).starts_with("0a"));
    assert!(calculate_hash("flqrgnkx", 3).starts_with("ad"));
    assert!(calculate_hash("flqrgnkx", 4).starts_with("68"));
    assert!(calculate_hash("flqrgnkx", 5).starts_with("c9"));
    assert!(calculate_hash("flqrgnkx", 6).starts_with("44"));
    assert!(calculate_hash("flqrgnkx", 7).starts_with("d6"));
}

#[test]
fn test_used_squares() {
    assert_eq!(calculate_used_squares("flqrgnkx"), 8108);
}

#[test]
fn test_regions() {
    assert_eq!(calculate_regions("flqrgnkx"), 1242);
}
