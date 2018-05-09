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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Collision {
    None,
    Instant(usize),
    Continuous,
}

impl Particle {
    pub fn load_group(input: &str) -> Vec<Particle> {
        input
            .lines()
            .filter_map(|l| l.parse::<Particle>().ok())
            .collect()
    }

    pub fn detect_collision(&self, other: &Particle) -> Option<usize> {
        let x_collisions = calculate_collisions(
            self.acceleration.0 - other.acceleration.0,
            self.velocity.0 - other.velocity.0,
            self.position.0 - other.position.0,
        );

        let y_collisions = calculate_collisions(
            self.acceleration.1 - other.acceleration.1,
            self.velocity.1 - other.velocity.1,
            self.position.1 - other.position.1,
        );

        let z_collisions = calculate_collisions(
            self.acceleration.2 - other.acceleration.2,
            self.velocity.2 - other.velocity.2,
            self.position.2 - other.position.2,
        );

        for x in x_collisions.iter().cloned() {
            let collision = x;

            if collision == Collision::None {
                continue;
            }

            for y in y_collisions.iter().cloned() {
                let collision = collision.intersection(&y);

                if collision == Collision::None {
                    continue;
                }

                for z in z_collisions.iter().cloned() {
                    let collision = collision.intersection(&z);

                    match collision {
                        Collision::Continuous => return Some(0),
                        Collision::Instant(n) => return Some(n),
                        Collision::None => continue,
                    }
                }
            }
        }

        None
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

impl Collision {
    fn intersection(&self, other: &Collision) -> Collision {
        match (*self, *other) {
            (Collision::Continuous, _) => *other,
            (_, Collision::Continuous) => *self,
            (Collision::Instant(is), Collision::Instant(io)) if is == io => *self,
            _ => Collision::None,
        }
    }
}

fn parse_tuple(input: &str) -> Result<(isize, isize, isize), ()> {
    let mut iter = input
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<isize>().ok());

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

fn calculate_collisions(da: isize, dv: isize, dx: isize) -> [Collision; 2] {
    let mut result = [Collision::None, Collision::None];

    if da != 0 {
        let delta = (2 * dv + da).pow(2) - 8 * da * dx;

        if delta < 0 {
            return result;
        }

        let sqrt_delta = (delta as f64).sqrt() as isize;

        if sqrt_delta.pow(2) != delta {
            return result;
        }

        let numerator_base = -(2 * dv + da);
        let denominator = 2 * da;
        let mut i = 0;

        for k in [-1, 1].iter().cloned() {
            let numerator = numerator_base + k * sqrt_delta;

            if numerator % denominator != 0 {
                continue;
            }

            let n = numerator / denominator;

            if n < 0 {
                continue;
            }

            result[i] = Collision::Instant(n as usize);
            i += 1;
        }
    } else if dv != 0 {
        if dx % dv == 0 {
            let n = -dx / dv;

            if n >= 0 {
                result[0] = Collision::Instant(n as usize);
            }
        }
    } else if dx == 0 {
        result[0] = Collision::Continuous;
    }

    result
}

fn count_colliders(particles: &Vec<Particle>) -> usize {
    let mut collisions = Vec::new();

    for i1 in 0..particles.len() {
        for i2 in (i1 + 1)..particles.len() {
            let n = particles[i1].detect_collision(&particles[i2]);

            if let Some(n) = n {
                collisions.push((n, i1, i2));
            }
        }
    }

    collisions.sort_by(|c1, c2| c1.0.cmp(&c2.0));

    let mut remove: Vec<bool> = (0..particles.len()).map(|_| false).collect();
    let mut buffer: Vec<usize> = Vec::new();

    let mut prev_tick = None;

    for collision in collisions {
        let curr_tick = Some(collision.0);

        if curr_tick != prev_tick {
            buffer.iter().cloned().for_each(|p| remove[p] = true);
            buffer.clear();
        }

        prev_tick = curr_tick;

        if remove[collision.1] || remove[collision.2] {
            continue;
        }

        buffer.push(collision.1);
        buffer.push(collision.2);
    }

    buffer.iter().cloned().for_each(|p| remove[p] = true);
    buffer.clear();

    remove.into_iter().filter(|&r| r).count()
}

fn main() {
    let input = common::load_file_input("day20");
    let particles = Particle::load_group(input.as_str());

    println!(
        "Closest to <0, 0, 0>: {}",
        find_closest_to_zero(particles.iter()).unwrap()
    );

    println!(
        "Non-collider particles: {}",
        particles.len() - count_colliders(&particles)
    );
}

#[test]
fn test_closest_to_zero() {
    let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\n\
                 p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>\n";

    let particles = Particle::load_group(input);

    assert_eq!(find_closest_to_zero(particles.iter()), Some(0));
}

#[test]
fn test_collisions_const_vel() {
    let input = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\n\
                 p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\n\
                 p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\n\
                 p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>\n";

    let particles = Particle::load_group(input);

    assert_eq!(count_colliders(&particles), 3);
}

#[test]
fn test_collisions_const_acc() {
    let input = "p=< 0, 2,-3>, v=<-1, 1,-3>, a=< 2, 0,-1>\n\
                 p=<-6, 4, 1>, v=< 7,-4,-5>, a=<-2, 1, 3>\n\
                 p=<-5, 0, 2>, v=< 0,-2, 2>, a=< 2, 0,-1>\n\
                 p=<-2,-8,-2>, v=< 2, 8, 4>, a=< 0,-3,-2>\n";

    let particles = Particle::load_group(input);

    assert_eq!(count_colliders(&particles), 2);
}
