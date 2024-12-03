advent_of_code::solution!(2);

const M0: u64=0b0000000000000000000000000000000000000000000000000000000011111111;
const M1: u64=0b0000000000000000000000000000000000000000000000001111111100000000;
const M2: u64=0b0000000000000000000000000000000000000000111111110000000000000000;
const M3: u64=0b0000000000000000000000000000000011111111000000000000000000000000;
const M4: u64=0b0000000000000000000000001111111100000000000000000000000000000000;
const M5: u64=0b0000000000000000111111110000000000000000000000000000000000000000;
const M6: u64=0b0000000011111111000000000000000000000000000000000000000000000000;
const M7: u64=0b1111111100000000000000000000000000000000000000000000000000000000;
const B0: u8=0;
const B1: u8=8;
const B2: u8=16;
const B3: u8=24;
const B4: u8=32;
const B5: u8=40;
const B6: u8=48;
const B7: u8=56;
fn single_bit_cmp(b: u64, m1: u64, s1: u8, m2: u64, s2: u8, f: fn(u64, u64) -> bool) -> bool {
    if (b & m1) >> s1 > 0 && (b & m2) >> s2 > 0 {
        f((b & m1) >> s1, (b & m2) >> s2)
    } else {
        true
    }
}
fn bit_cmp(b: u64, f: fn(u64, u64) -> bool) -> bool {
    single_bit_cmp(b, M0, B0, M1, B1, f) &&
        single_bit_cmp(b, M1, B1, M2, B2, f) &&
        single_bit_cmp(b, M2, B2, M3, B3, f) &&
        single_bit_cmp(b, M3, B3, M4, B4, f) &&
        single_bit_cmp(b, M4, B4, M5, B5, f) &&
        single_bit_cmp(b, M5, B5, M6, B6, f) &&
        single_bit_cmp(b, M6, B6, M7, B7, f)

}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.lines()
        .map(|l|
            l.split_whitespace()
                .fold(0, |acc: u64, val| {
                    let num = val.parse::<u64>().unwrap();
                    (acc << 8) | num
                }))
        .filter(|u| bit_cmp(*u, |a, b| a > b) || bit_cmp(*u, |a, b| a < b))
        .filter(|u| bit_cmp(*u, |a, b| a.abs_diff(b) > 0 && a.abs_diff(b) < 4))
        .count();
    Some(res as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
