use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //let contents = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    let part1: usize = get_part1(&contents);
    println!("Part 1: {}", part1);

}

fn get_part1(contents: &str) -> usize {
    
    let all_chars: Vec<char> = contents.chars().filter(|c| !c.is_numeric()).collect::<HashSet<char>>().into_iter().collect();
    let op_chars: Vec<char> = all_chars.clone().to_owned().into_iter().filter(|c| *c != '.' && *c != '\n').to_owned().collect();

    // List of numbers parsed in
    let mut number_vec: Vec<i32> = Vec::new();
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
    let numbers: Vec<&str> = contents.split(&all_chars[..]).filter(|x| !x.is_empty()).collect();

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
        let lr = (std::cmp::min(i_max, pi + 1), std::cmp::min(j_max, pj + n.len() as i32));

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
    std::iter::zip(&numbers, &part_number_mask).map(|(n, b)| n.parse::<usize>().unwrap() * b).sum()
}
