use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1: usize = get_part1(&contents);
    println!("Part 1: {}", part1);
    let part2: usize = get_part2(&contents);
    println!("Part 2: {}", part2);
}

fn get_part1(contents: &str) -> usize {
    let all_chars: Vec<char> = contents
        .chars()
        .filter(|c| !c.is_numeric())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();
    let op_chars: Vec<char> = all_chars
        .clone()
        .to_owned()
        .into_iter()
        .filter(|c| *c != '.' && *c != '\n')
        .to_owned()
        .collect();

    // Grid with top left as 0, 0 keeping track of leftmost number char
    let mut position_vec: Vec<(i32, i32)> = Vec::new();
    for (i, l) in contents.lines().enumerate() {
        let mut prev_char_numeric = false;
        for (j, c) in l.chars().enumerate() {
            let current_char_numeric = c.is_numeric();

            if !prev_char_numeric && current_char_numeric {
                position_vec.push((i as i32, j as i32));
            }
            prev_char_numeric = c.is_numeric();
        }
    }
    let numbers: Vec<&str> = contents
        .split(&all_chars[..])
        .filter(|x| !x.is_empty())
        .collect();

    let contents_grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    // Now check numbers based on surrounding grid
    let j_max = (contents_grid[0].len() - 1) as i32;
    let i_max = (contents_grid.len() - 1) as i32;

    let op_char_set = op_chars.into_iter().collect::<HashSet<_>>();

    let mut part_number_mask: Vec<usize> = Vec::new();
    for (n, p) in std::iter::zip(&numbers, &position_vec) {
        let (pi, pj) = p;
        // Define rectangle padding around number in all directions (upper left, lower right)
        let ul = (std::cmp::max(0, pi - 1), std::cmp::max(0, pj - 1));
        let lr = (
            std::cmp::min(i_max, pi + 1),
            std::cmp::min(j_max, pj + n.len() as i32),
        );

        let mut part_number = 0;
        for y in ul.0..(lr.0 + 1) {
            for x in ul.1..(lr.1 + 1) {
                let c = contents_grid[y as usize][x as usize];
                if op_char_set.contains(&c) {
                    part_number = 1;
                }
            }
        }
        part_number_mask.push(part_number);
    }

    //println!("{:?} {:?} {:?}", numbers, position_vec, part_number_mask);
    std::iter::zip(&numbers, &part_number_mask)
        .map(|(n, b)| n.parse::<usize>().unwrap() * b)
        .sum()
}

fn get_part2(contents: &str) -> usize {
    let all_chars: Vec<char> = contents
        .chars()
        .filter(|c| !c.is_numeric())
        .collect::<HashSet<char>>()
        .into_iter()
        .collect();
    let mut multi_positions: Vec<(i32, i32)> = Vec::new();

    for (i, l) in contents.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '*' {
                multi_positions.push((i as i32, j as i32));
            }
        }
    }

    let mut position_vec: Vec<(i32, i32)> = Vec::new();
    for (i, l) in contents.lines().enumerate() {
        let mut prev_char_numeric = false;
        for (j, c) in l.chars().enumerate() {
            let current_char_numeric = c.is_numeric();

            if !prev_char_numeric && current_char_numeric {
                position_vec.push((i as i32, j as i32));
            }
            prev_char_numeric = c.is_numeric();
        }
    }
    let numbers: Vec<&str> = contents
        .split(&all_chars[..])
        .filter(|x| !x.is_empty())
        .collect();

    // Generate map
    let mut number_map = HashMap::new();
    let mut position_map = HashMap::new();

    for (n, p) in std::iter::zip(numbers, position_vec) {
        for i in 0..n.len() {
            number_map.insert((p.0, p.1 + i as i32), n);
            position_map.insert((p.0, p.1 + i as i32), p);
        }
    }

    let contents_grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    // Now check numbers based on surrounding grid
    let j_max = (contents_grid[0].len() - 1) as i32;
    let i_max = (contents_grid.len() - 1) as i32;

    // define grid around *
    let mut adjacency_vec: Vec<Vec<&str>> = Vec::new();
    for mp in multi_positions.iter() {
        let mut mp_adjacent: Vec<&str> = Vec::new();
        let (pi, pj) = mp;
        // Define rectangle padding around number in all directions (upper left, lower right)
        let ul = (std::cmp::max(0, pi - 1), std::cmp::max(0, pj - 1));
        let lr = (std::cmp::min(i_max, pi + 1), std::cmp::min(j_max, pj + 1));

        let mut numbers_got: HashSet<(usize, usize)> = HashSet::new();

        for y in ul.0..(lr.0 + 1) {
            for x in ul.1..(lr.1 + 1) {
                match (number_map.get(&(y, x)), position_map.get(&(y, x))) {
                    (Some(n), Some(p)) => {
                        if !numbers_got.contains(&(p.0 as usize, p.1 as usize)) {
                            mp_adjacent.push(n);
                            numbers_got.insert((p.0 as usize, p.1 as usize));
                        }
                    }
                    _ => (),
                }
            }
        }

        adjacency_vec.push(mp_adjacent);
    }

    let out = adjacency_vec
        .iter()
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().map(|x| x.parse::<usize>().unwrap()).collect())
        .map(|x: Vec<usize>| x.iter().product::<usize>())
        .sum::<usize>();
    out
}
