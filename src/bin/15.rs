#[allow(unused)]
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

advent_of_code::solution!(15);

fn parse<'a>(input: &'a str, matrix: &'a mut Vec<Vec<char>>, movement: &mut String, pos: &mut (usize, usize)) {
    let (field, instructions): (Vec<_>, Vec<_>) = input.lines()
        .map(|l| l.trim_end())
        .partition(|&l| l.starts_with('#'));
    *matrix = field.iter()
        .map(|&l| l.chars().collect())
        .collect();
    for y in 1..matrix.len() {
        if let Some(x) = (*matrix[y]).iter().position(|&x| x  == '@' ) {
            *pos = (x, y);
        }
    }
    *movement = instructions.join("");
}

fn p1_move(matrix: &mut Vec<Vec<char>>, x: isize, y: isize, pos: (usize, usize)) -> (usize, usize){
    let (dx, dy): (usize, usize) = ((pos.0 as isize + x) as usize, (pos.1 as isize + y) as usize);
    match matrix[dy][dx] {
        '.' => (dx, dy),
        '#' => (0, 0),
        'O' => p1_move(matrix, x, y, (dx, dy)),
        '0' => p1_move(matrix, x, y, (dx, dy)),
        _ => {
            panic!("invalid block {}", matrix[dy][dx]);
        }
    }
}

fn p1_step(matrix: &mut Vec<Vec<char>>, movement: char, pos: (usize, usize)) -> (usize, usize) {
    let end = match movement {
        '^' => p1_move(matrix,  0, -1, pos),
        'v' => p1_move(matrix,  0,  1, pos),
        '<' => p1_move(matrix, -1,  0, pos),
        '>' => p1_move(matrix,  1,  0, pos),
        _ => {
            panic!("invalid movement {movement}");
        }
    };
    if end == (0,0) || pos == end {
        // no changes
        pos
    } else {
        // modify matrix
        let (rx, ry) = ( end.0 as isize - pos.0 as isize, end.1 as isize - pos.1 as isize);
        matrix[pos.1][pos.0] = '.';
        if rx.abs() == 1 || ry.abs() == 1 {
            matrix[end.1][end.0] = '@';
            end
        } else {
            let next = ((pos.0 as isize + rx.signum()) as usize, (pos.1 as isize + ry.signum()) as usize);
            matrix[next.1][next.0] = '@';
            matrix[end.1][end.0] = '0';
            next
        }
    }
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<char>>) {
    let mut lock = stdout().lock();
    write!(lock, "{}[2J", 27 as char).ok();
    for y in 0..matrix.len() {
         writeln!(lock, "{}", matrix[y].iter().collect::<String>()).unwrap();
     }
    sleep(Duration::from_millis(250));
 }

pub fn part_one(input: &str) -> Option<usize> {
    let mut mat= Vec::new();
    let mut mov= "".to_string();
    let mut pos = (0, 0);
    parse(input, &mut mat, &mut mov, &mut pos);
    // let start = mat.clone();
    for c in mov.chars() {
        pos = p1_step(&mut mat, c, pos);
        // == visualize steps ==
        // print_matrix(&mat);
    }

    let mut result = 0;
    for y in 1..mat.len() {
        for x in 1..mat[0].len() {
            if mat[y][x] == '0' || mat[y][x] == 'O' {
                result += y*100 + x;
            }
        }
    }
    assert_eq!(result, 1294459);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mat = Vec::new();
    let mut mov =  "".to_string();
    let mut pos = (0, 0);
    parse(input, &mut mat, &mut mov, &mut pos);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
