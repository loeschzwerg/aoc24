advent_of_code::solution!(3);

use regex::{Captures, Regex};

fn to_u32(cap: &Captures, i: usize) -> u32 {
    cap.get(i).unwrap().as_str().parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .fold(0, |acc, x| {
            acc + x.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * x.get(2).unwrap().as_str().parse::<u32>().unwrap()
        });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let result = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .fold(0, |acc, x| {
            if x.get(0).unwrap().as_str() == "do()" {
                enabled = true;
                acc
            } else if x.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
                acc
            } else if !enabled {
                acc
            } else {
                acc + to_u32(&x, 1) * to_u32(&x, 2)
            }
        });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
