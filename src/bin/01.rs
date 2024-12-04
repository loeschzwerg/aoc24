use std::collections::BTreeMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|l| l.split_once("   "))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();
    let result = left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let lookup = right.iter().fold(BTreeMap::new(), |mut acc, r| {
        *acc.entry(r).or_insert(0) += 1;
        acc
    });

    let result = left
        .iter()
        .fold(0, |acc, l| acc + l * lookup.get(l).unwrap_or(&0));

    Some(result.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
