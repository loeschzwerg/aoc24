use std::collections::{BTreeMap, BTreeSet};
use linear_solver::{solve_minimum, Inference};
use linear_solver::Inference::*;

advent_of_code::solution!(5);

pub fn infer(_cache: &std::collections::HashSet<(u32, u32)>, facts: &[(u32,u32)]) -> Option<Inference<(u32,u32)>> {
    // Put simplification rules first to find simplest set of facts.
    let mut truth = std::collections::HashSet::new();
    for f1 in facts {
        for f2 in facts {
            if f1 == f2 {
                continue;
            } else if f1.1 == f2.0 { // x < y < z -> remove x < z
                truth.insert((f1.0, f2.1));
            } else if f1.0 == f2.1 { // z < y < x -> remove z < x
                truth.insert((f2.0, f1.1));
            } else {
                continue
            }
        }
    }
    if !truth.is_empty() {
        return Some(ManyTrue {
            from: truth.iter().cloned().collect(),
        })
    }
    None
}
// Algorithm idea:
// 1. Create a custom ordering using a linear_solver
// 2. Check ordering of Pages
pub fn part_one(input: &str) -> Option<u32> {
    let (prelim, seqs): (Vec<&str>, Vec<&str>) = input.lines()
        .partition(|x|x.contains('|'));
    let facts: Vec<(u32, u32)> = prelim.iter()
        .map(|x| x.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect();
    let solve = solve_minimum(facts, infer);
    let mut path = BTreeMap::new();
    for (k, v) in &solve {
        path.insert(*k, *v);
    }
    let k: BTreeSet<u32> = path.keys().copied().collect();
    let v: BTreeSet<u32> = path.values().copied().collect();
    let binding = k.difference(&v)
        .cloned()
        .collect::<Vec<_>>();
    let begin: &u32 = binding
        .first()
        .unwrap_or(k.first().unwrap());
    let mut ordered: Vec<u32> = vec![*begin];
    let mut left = begin;
    loop {
        if let Some(right) = path.get(left){
            ordered.push(*right);
            left = right;
        } else {
            break
        }
        if left == begin {
            break
        }
    }
    let indexed: BTreeMap<u32, usize> = ordered.iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut acc, (i, v)| {acc.insert(*v, i); acc});

    fn mapped_asc(v: Vec<u32>, m: &BTreeMap<u32, usize>) -> Option<u32>{
        if v.len() == 0 {
            return None
        }
        for x in 1..v.len() {
            if m.get(&v[x-1]).unwrap() >= m.get(&v[x]).unwrap() {
               return None
            }
        }
        Some(v[v.len()/2])
    }

    let result = seqs
        .into_iter()
        .map(|x| x.split(',')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect())
        .filter_map(|v| mapped_asc(v, &indexed))
        .fold(0u32, |acc, b| acc + b);

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
