advent_of_code::solution!(15);

fn parse<'a>(input: &'a str, matrix: &mut Vec<&'a [u8]>, movement: &mut String, pos: &mut (usize, usize)) {
    let (field, instructions): (Vec<_>, Vec<_>) = input.lines()
        .map(|l| l.trim_end())
        .partition(|&l| l.starts_with('#'));
    *matrix = field.iter()
        .map(|&l| l.as_bytes())
        .collect();
    for y in 1..matrix.len() {
        if let Some(x) = (*matrix[y]).iter().position(|&x| x  == ('@' as u8)) {
            *pos = (x, y);
        }
    }
    *movement = instructions.join("");
}

fn p1_move(matrix: &Vec<&[u8]>, x: isize, y: isize, pos: (usize, usize)) -> (usize, usize){
    let (dx, dy): (usize, usize) = ((pos.0 as isize + x) as usize, (pos.1 as isize + y) as usize);
    match matrix[dy][dx] as char {
        '.' => (dx, dy),
        '#' => pos,
        'O' => p1_move(matrix, x, y, (dx, dy)),
        _ => {
            panic!("invalid block {}", matrix[dy][dx] as char);
        }
    }
}

fn p1_step(mut matrix: &mut Vec<&[u8]>, movement: char, pos: (usize, usize)) -> (usize, usize) {
    let end = match movement {
        '^' => p1_move(&matrix,  0, -1, pos),
        'v' => p1_move(&matrix,  0,  1, pos),
        '<' => p1_move(&matrix, -1,  0, pos),
        '>' => p1_move(&matrix,  1,  0, pos),
        _ => {
            panic!("invalid movement {movement}");
        }
    };
    if pos == end {
        // no changes
        pos
    } else {
        // modify matrix
        let (rx, ry) = (pos.0 as isize - end.0 as isize, pos.1 as isize - end.1 as isize);
        let mut pos_m = matrix[pos.1].as_mut();
        let mut end_m = matrix[end.1].as_mut();
        pos_m[pos.0] = '.' as u8;
        if rx.abs() == 1 || ry.abs() == 1 {
            end_m[end.0] = '@' as u8;
            end
        } else {
            let next = ((pos.0 as isize + rx.signum()) as usize, (pos.1 as isize + ry.signum()) as usize);
            let mut next_m = matrix[next.1].as_mut();
            next_m[next.0] = '@' as u8;
            end_m[end.0] = '0' as u8;
            next
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat= Vec::new();
    let mut mov= "".to_string();
    let mut pos = (0, 0);
    parse(input, &mut mat, &mut mov, &mut pos);
    for c in mov.chars() {
        println!("{c}");
        pos = p1_step(&mut mat, c, pos);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mat= Vec::new();
    let mut mov =  "".to_string();
    let mut pos = (0, 0);
    parse(input, &mut mat, &mut mov, &mut pos);
    dbg!(&pos);
    None
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
