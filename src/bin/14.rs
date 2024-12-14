use std::ops::Shr;

advent_of_code::solution!(14);

#[derive(Debug, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    p: Position,
    v: Position,
}

fn to_robot(line: &str) -> Option<Robot> {
    let mut collector: Vec<isize> = vec![0; 4];
    let mut state: usize = 3; // p.x, p.y, v.x, v.y
    let l = line.len();
    let mut j: usize = l; // index to read
    for (i, ch) in line.chars().rev().enumerate() {
        // parse
        if ch == '=' || ch == ',' {
            collector[state] = line[l - i..j].parse::<isize>().unwrap();
            state = state.saturating_sub(1)
        }

        // advance
        if ch == ',' || ch == ' ' {
            j = l - i - 1;
        }
    }

    if state == 0 {
        Some(
            Robot {
                p: Position {
                    x: collector[0],
                    y: collector[1],
                },
                v: Position {
                    x: collector[2],
                    y: collector[3],
                },
            })
    } else {
        None
    }
}

// const X: isize = 11; // 0..=10
// const Y: isize = 7; // 0..=6
const X: isize = 101; // 0..=101
const Y: isize = 103; // 0..=103
const X0: isize = X / 2;
const Y0: isize = Y / 2;

pub fn part_one(input: &str) -> Option<usize> {
    let fstate = input
        .lines()
        .filter_map(to_robot)
        // pass 100 ticks
        .map(|mut robot: Robot| {
            robot.p.x = (robot.p.x + robot.v.x * 100).rem_euclid(X);
            robot.p.y = (robot.p.y + robot.v.y * 100).rem_euclid(Y);
            robot
        })
        // filter on quadrant middle
        .filter(|robot| robot.p.x != X0 && robot.p.y != Y0)
        // identify quadrant
        .fold(vec![0_usize; 4].as_mut_slice(), |quadrants: &mut [usize], robot: Robot| {
            let x = (robot.p.x - X0).shr(isize::BITS - 1) + 1;
            let y = (robot.p.y - Y0).shr(isize::BITS - 1) + 1;
            let index = (x + 2 * y) as usize;
            quadrants[index] += 1;
            quadrants
        })
        .iter()
        .product();
    // assert_eq!(fstate, 225552000);
    Some(fstate)
}

fn step(robots: &mut Vec<Robot>, amount: isize) -> () {
    for i in 0..robots.len() {
        robots[i].p.x = (robots[i].p.x + amount * robots[i].v.x).rem_euclid(X);
        robots[i].p.y = (robots[i].p.y + amount * robots[i].v.y).rem_euclid(Y);
    }
}

// To form any (semi-)solid image, robots have to assemble in a **dense or sparse formation**:
// 0. Find highest / lowest dispersion -> Repeat 1000 times after the candidate has been found
// 1. Calculate next step
// 2. Calculate the density across 2D-space
// 3. E(x) ≈ Center, σ²(x) = E(|X - E(X)|), Cov(X,Y) = E(XY) - E(X)E(Y)
//
// The christmas tree could also be an outline, which this approach would not find. Let's see!
// NOTE: Unfortunately this approach didn't work. Although it is blazingly fast, results matter.
// TRIAGE: The target Covariance of 350 was beaten by many others with about 1000
// Runtime: 30ms / 10000
#[allow(dead_code)]
fn evaluate_covariance(robots: &Vec<Robot>) -> f64 {
    let mut meanx = 0.;
    let mut meany = 0.;
    let mut c = 0.;
    let mut n = 0.;

    for i in 0..robots.len() {
        n += 1.;
        let dx = robots[i].p.x as f64 - meanx;
        let dy = robots[i].p.x as f64 - meany;
        meanx += dx / n;
        meany += dy / n;
        c += dx * dy;
    }
    // Covariance(X, Y): lim x,y Cov(X,Y) -> 0, if linear independent
    c / n
}

// This is really slow, but it gets the job done
// Runtime: 25s / 10.000
#[allow(dead_code)]
fn evaluate_x_line(robots: &Vec<Robot>) -> f64 {
    let mut counter = 0;
    let mut longest = 0;
    for y in 0..Y {
        for x in 0..X {
            if robots.iter().find(|&&r| r.p.x == x && r.p.y == y).is_some() {
                counter += 1;
                longest = std::cmp::max(counter, longest);
            } else {
                counter = 0;
            }
        }
    }
    longest as f64
}

// Let's try mean squared error
// TRIAGE: Works like a charm! Combining mean and mse calculation is fine and faster!
// Runtime: 36ms / 10_000
#[allow(dead_code)]
fn evaluate_mse(robots: &Vec<Robot>) -> f64 {

    // calculate Mean

    fn distance(left: &Robot, x: f64, y: f64) -> f64 {
        ((left.p.x as f64 - x).powf(2.) - (left.p.y as f64 - y).powf(2.)).abs().sqrt()
    }

    let mut meanx = 0.;
    let mut meany = 0.;
    let mut mse = 0.;
    for i in 0..robots.len() {
        let n = (i + 1) as f64;
        let dx = robots[i].p.x as f64 - meanx;
        let dy = robots[i].p.y as f64 - meany;
        meanx += dx / n;
        meany += dy / n;
        // incrementally calculate mean squared error -> inaccurate, but it finds clusters
        mse += distance(&robots[i], meanx, meany) / n;
    }

    mse
}

#[allow(dead_code)]
fn print_field(robots: &Vec<Robot>) -> () {
    for y in 0..Y {
        for x in 0..X {
            if robots.iter().find(|robot| robot.p.y == y && robot.p.x == x).is_some() {
                print!("x")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut positions: Vec<Robot> = input.lines()
        .filter_map(to_robot)
        .collect();

    let mut steps = 0;
    let mut best = f64::INFINITY;
    let mut tree = 0;
    while steps < 10000 {
        steps += 1;
        step(&mut positions, 1);
        // let candidate = evaluate_covariance(&mut positions); // 30ms ⌿
        // let candidate = evaluate_x_line(&mut positions); // 26s
        let candidate = evaluate_mse(&mut positions); // 50ms → 35ms
        if candidate < best {
            // println!("{}: {}", steps, candidate);
            best = candidate;
            tree = steps;
        }
    }
    // print_field(&positions);
    // assert_eq!(tree, 7371);
    Some(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
