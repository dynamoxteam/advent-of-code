extern crate common;
extern crate regex;

use std::str::FromStr;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
struct Particle {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
    acceleration: (isize, isize, isize),
}

impl Particle {
    fn load_group(input: &str) -> Vec<Particle> {
        let mut particles = Vec::new();

        for line in input.lines() {
            let particle = line.parse::<Particle>();

            if let Ok(particle) = particle {
                particles.push(particle)
            }
        }

        particles
    }
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let particle_re =
            Regex::new(r"^p=<(?P<pos>.*)>, v=<(?P<vel>.*)>, a=<(?P<acc>.*)>$").unwrap();

        let captures = particle_re.captures(s).ok_or(())?;

        let parse = |n| parse_tuple(captures.name(n).map(|c| c.as_str()).ok_or(())?);

        let position = parse("pos")?;
        let velocity = parse("vel")?;
        let acceleration = parse("acc")?;

        Ok(Particle {
            position,
            velocity,
            acceleration,
        })
    }
}

fn parse_tuple(input: &str) -> Result<(isize, isize, isize), ()> {
    let mut iter = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<isize>().ok());

    Ok((
        iter.next().ok_or(())?,
        iter.next().ok_or(())?,
        iter.next().ok_or(())?,
    ))
}

fn find_closest_to_zero<'a, I>(particles: I) -> Option<usize>
where
    I: Iterator<Item = &'a Particle>,
{
    particles
        .map(|p| p.acceleration.0.abs() + p.acceleration.1.abs() + p.acceleration.2.abs())
        .enumerate()
        .min_by(|x, y| x.1.cmp(&y.1))
        .map(|e| e.0)
}

fn main() {
    let input = common::load_file_input("day20");
    let particles = Particle::load_group(input.as_str());

    println!(
        "Closest to <0, 0, 0>: {}",
        find_closest_to_zero(particles.iter()).unwrap()
    );
}

#[test]
fn test() {
    let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\n\
                 p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";

    let particles = Particle::load_group(input);

    assert_eq!(find_closest_to_zero(particles.iter()), Some(0));
}
