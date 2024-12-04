use std::cmp::min;

advent_of_code::solution!(2);

const M0: u64 = 0b0000000000000000000000000000000000000000000000000000000011111111;
const M1: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
const M2: u64 = 0b0000000000000000000000000000000000000000111111110000000000000000;
const M3: u64 = 0b0000000000000000000000000000000011111111000000000000000000000000;
const M4: u64 = 0b0000000000000000000000001111111100000000000000000000000000000000;
const M5: u64 = 0b0000000000000000111111110000000000000000000000000000000000000000;
const M6: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;
const M7: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;
const M: [u64; 8] = [M0, M1, M2, M3, M4, M5, M6, M7];
const B0: u8 = 0;
const B1: u8 = 8;
const B2: u8 = 16;
const B3: u8 = 24;
const B4: u8 = 32;
const B5: u8 = 40;
const B6: u8 = 48;
const B7: u8 = 56;
const B: [u8; 8] = [B0, B1, B2, B3, B4, B5, B6, B7];

fn single_bit_cmp(b: u64, m1: u64, s1: u8, m2: u64, s2: u8, f: fn(u64, u64) -> bool) -> bool {
    if (b & m1) >> s1 > 0 && (b & m2) >> s2 > 0 {
        f((b & m1) >> s1, (b & m2) >> s2)
    } else {
        true
    }
}

fn bit_cmp(b: u64, f: fn(u64, u64) -> bool) -> bool {
    single_bit_cmp(b, M0, B0, M1, B1, f)
        && single_bit_cmp(b, M1, B1, M2, B2, f)
        && single_bit_cmp(b, M2, B2, M3, B3, f)
        && single_bit_cmp(b, M3, B3, M4, B4, f)
        && single_bit_cmp(b, M4, B4, M5, B5, f)
        && single_bit_cmp(b, M5, B5, M6, B6, f)
        && single_bit_cmp(b, M6, B6, M7, B7, f)
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|l| {
            l.split_whitespace().fold(0, |acc: u64, val| {
                let num = val.parse::<u64>().unwrap();
                (acc << 8) | num
            })
        })
        .filter(|u| bit_cmp(*u, |a, b| a > b) || bit_cmp(*u, |a, b| a < b))
        .filter(|u| bit_cmp(*u, |a, b| a.abs_diff(b) > 0 && a.abs_diff(b) < 4))
        .count();
    Some(res as u32)
}

fn bit_cmp_loc(b: u64, f: fn(u64, u64) -> bool) -> u8 {
    // boolean false is denoted as 0 - 11111011
    ((single_bit_cmp(b, M0, B0, M1, B1, f) as u8) << 0)
        + ((single_bit_cmp(b, M1, B1, M2, B2, f) as u8) << 1)
        + ((single_bit_cmp(b, M2, B2, M3, B3, f) as u8) << 2)
        + ((single_bit_cmp(b, M3, B3, M4, B4, f) as u8) << 3)
        + ((single_bit_cmp(b, M4, B4, M5, B5, f) as u8) << 4)
        + ((single_bit_cmp(b, M5, B5, M6, B6, f) as u8) << 5)
        + ((single_bit_cmp(b, M6, B6, M7, B7, f) as u8) << 6)
        + (1 << 7)
}

fn eliminate_invalid(u: u8) -> u8 {
    if u.count_zeros() < 2 {
        u
    } else if u.count_zeros() > 2 {
        0
    } else if u.leading_ones() + u.trailing_ones() == 6 {
        u
    }
    // eliminate intermittent 1
    else {
        0
    }
}

fn asc(a: u64, b: u64) -> bool {
    a < b
} // identifies ascending in negation
fn desc(a: u64, b: u64) -> bool {
    a > b
} // identifies descending in negation
fn diff(a: u64, b: u64) -> bool {
    let d = (a as i64).abs_diff(b as i64);
    d > 0 && d < 4
}
fn asc_check(a: u64, b: u64) -> bool {
    asc(a, b) && diff(a, b)
}
fn desc_check(a: u64, b: u64) -> bool {
    desc(a, b) && diff(a, b)
}
pub fn part_two(input: &str) -> Option<u32> {
    let setup = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .fold(0, |acc: u64, val| (acc << 8) | val.parse::<u64>().unwrap())
        })
        .into_iter();
    let (done, cleanup): (Vec<_>, Vec<_>) = setup
        // (u, asc, desc)
        .map(|u| (u, bit_cmp_loc(u, asc), bit_cmp_loc(u, desc)))
        // filter where no fix is available
        .filter(|(_, a, d)| a.count_zeros() < 2 || d.count_zeros() < 2)
        // set bitmask of error location
        .map(|(u, a, d)| (u, a & bit_cmp_loc(u, diff), d & bit_cmp_loc(u, diff)))
        // filter where e.g. 1 8 2 has to be valid, but 8 fails comparison
        .filter(|(_, a, d)| a.count_zeros() < 3 || d.count_zeros() < 3)
        .map(|(u, a, d)| (u, eliminate_invalid(a), eliminate_invalid(d)))
        // eliminate surplus
        .filter(|(_, a, d)| a | d != 0)
        // partition into valid (no error, or error at border) and TBD
        .partition(|(_, a, d)| {
            *a >= 0b11111110 || *d >= 0b11111110 || *a == 0b10111111 || *d == 0b10111111
        });

    let cleaned: Vec<_> = cleanup
        .iter()
        .filter(|(u, a, d)| {
            let i = (!a & !d).ilog2() as usize;
            // println!("u {:64b} a {:8b} d {:8b}", u, a, d);
            // [(0,2), (1,3), (2,4), (3,5), (4,6), (5,7)] check these pairs
            //  1 0 1 1 1 0 0
            // 1 2 3 4 5 6 1 8
            if *d == 0 {
                single_bit_cmp(*u, M[i - 1], B[i - 1], M[i + 1], B[i + 1], asc_check)
                    || single_bit_cmp(
                        *u,
                        M[i],
                        B[i],
                        M[min(i + 2, M.len() - 1)],
                        B[min(i + 2, B.len() - 1)],
                        asc_check,
                    )
            } else if *a == 0 {
                single_bit_cmp(*u, M[i - 1], B[i - 1], M[i + 1], B[i + 1], desc_check)
                    || single_bit_cmp(
                        *u,
                        M[i],
                        B[i],
                        M[min(i + 2, M.len() - 1)],
                        B[min(i + 2, B.len() - 1)],
                        desc_check,
                    )
            } else {
                // dbg!(i, a, d, u);
                false
            }
        })
        .collect();
    Some((done.len() + cleaned.len()) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
