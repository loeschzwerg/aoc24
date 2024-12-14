advent_of_code::solution!(4);

const X: u8 = b'X';
const M: u8 = b'M';
const A: u8 = b'A';
const S: u8 = b'S';

fn check_one(matrix: &Vec<Vec<u8>>, x: isize, y: isize, x_dir: isize, y_dir: isize) -> u32 {
    // dbg!(x, y, x_dir, y_dir);
    if matrix[y as usize][x as usize] == X {
        if matrix[(y + y_dir) as usize][(x + x_dir) as usize] == M {
            if matrix[(y + 2 * y_dir) as usize][(x + 2 * x_dir) as usize] == A {
                if matrix[(y + 3 * y_dir) as usize][(x + 3 * x_dir) as usize] == S {
                    return 1;
                }
            }
        }
    }
    0
}

fn check_xmas(
    matrix: &Vec<Vec<u8>>,
    x: isize,
    y: isize,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
) -> u32 {
    let mut count = 0;
    if up {
        count += check_one(matrix, x, y, 0, -1);
    }
    if down {
        count += check_one(matrix, x, y, 0, 1);
    }
    if left {
        count += check_one(matrix, x, y, -1, 0);
    }
    if right {
        count += check_one(matrix, x, y, 1, 0);
    }
    if up && left {
        count += check_one(matrix, x, y, -1, -1);
    }
    if up && right {
        count += check_one(matrix, x, y, 1, -1);
    }
    if down && left {
        count += check_one(matrix, x, y, -1, 1);
    }
    if down && right {
        count += check_one(matrix, x, y, 1, 1);
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let x_max = matrix[0].len() as isize;
    let y_max = matrix.len() as isize;
    let mut counter = 0;
    let mut up;
    let mut down;
    let mut left;
    let mut right;
    for y in 0..y_max {
        up = y >= 3;
        down = y + 3 < y_max;
        for x in 0..x_max {
            left = x >= 3;
            right = x + 3 < x_max;
            counter += check_xmas(&matrix, x, y, up, down, left, right);
        }
    }
    Some(counter)
}

fn check_masx(matrix: &Vec<Vec<u8>>, x: isize, y: isize) -> u32 {
    if matrix[y as usize][x as usize] == A {
        let upl = matrix[(y - 1) as usize][(x - 1) as usize];
        let dor = matrix[(y + 1) as usize][(x + 1) as usize];
        if (upl == M && dor == S) || (upl == S && dor == M) {
            let upr = matrix[(y - 1) as usize][(x + 1) as usize];
            let dol = matrix[(y + 1) as usize][(x - 1) as usize];
            if (upr == M && dol == S) || (upr == S && dol == M) {
                return 1;
            }
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let x_max = matrix[0].len() as isize;
    let y_max = matrix.len() as isize;
    let mut counter = 0;
    for y in 1..(y_max - 1) {
        for x in 1..(x_max - 1) {
            counter += check_masx(&matrix, x, y)
        }
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
