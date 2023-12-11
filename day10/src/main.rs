use std::fs;
use std::collections::{HashSet, HashMap};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // let contents = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";


    let map_grid = contents.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut current_pos = (0, 0);

    'outer: for (i, line) in map_grid.iter().enumerate() {
        for (j, pos) in line.iter().enumerate() {
            if pos == &'S' {
                current_pos = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    let s_pos = (current_pos.0, current_pos.1);

    // initial tries
    let mut cycle_steps = 0;
    let mut cycle_set = HashSet::new();
    let mut s_shape = ' ';
    'outer: for (initial_entry_dir, initial_pipe_shape) in [('E', '-'), ('E', 'J'), ('N', '|'), ('S', 'L'), ('E', '7'), ('N', 'F')].iter() {

        let mut pipe_set = HashSet::new();
        current_pos = (s_pos.0, s_pos.1);
        pipe_set.insert(current_pos);
        let mut total_steps = 0;
        let piped = pipe(*initial_entry_dir, *initial_pipe_shape);
        let mut next = (current_pos.0 + piped.0, current_pos.1 + piped.1);
        if next.0 < 0 || next.1 < 0 {
            continue 'outer;
        }

        current_pos = (next.0, next.1);
        pipe_set.insert(current_pos);
        total_steps += 1;
        let mut entry_dir = get_entry_dir(&piped);

        'inner: while current_pos != s_pos {
            let pipe_shape = map_grid[current_pos.0 as usize][current_pos.1 as usize];
            let piped = pipe(entry_dir, pipe_shape);
            if piped == (0, 0) { continue 'outer; }
            next = (current_pos.0 + piped.0, current_pos.1 + piped.1);
            if next.0 < 0 || next.1 < 0 {
                continue 'outer;
            }
            entry_dir = get_entry_dir(&piped);

            current_pos = (next.0, next.1);
            pipe_set.insert(current_pos);
            total_steps += 1;
        }
        
        cycle_steps += total_steps;
        cycle_set = pipe_set;
        println!("PIPE SHAPE: {}", initial_pipe_shape);
        s_shape = *initial_pipe_shape;
        break;
    };

    println!("Part 1: {}", cycle_steps / 2);
    println!("PIPESET LEN: {}", cycle_set.len() / 2);

    let mut square_count = 0;
    let mut vis_grid = Vec::new();
    
    for i in 0..map_grid.len() {
        let mut prev = '.';
        let mut v = Vec::new();
        let mut on = false;
        for j in 0..map_grid[0].len() {
            let elem = map_grid[i as usize][j as usize];
            let elem = if (i as i32, j as i32) == s_pos { s_shape } else { elem };
            if cycle_set.contains(&(i as i32, j as i32)) {
                // on = match (prev, elem) {
                //     ('-', 'J') => !on,
                //     ('F', 'J') => !on,
                //     ('L', '7') => !on,
                //     ('L', 'J') => !on,
                //     ('-', '7') => on,
                //     _ => true,
                // };
                on = match elem {
                    'J' | 'L' | 'F' | '7' | '|' => !on,
                    _ => on,
                };
                // println!("{} {}", prev, elem);
                on = match (prev, elem) {
                    ('L', 'J') => on,
                    ('L', '7') => !on,
                    ('F', 'J') => !on,
                    ('F', '7') => on,
                    ('-', '7') => !on,
                    _ => on,
                };
                // v.push((on as i32).to_string().chars().nth(0).unwrap() );
                v.push(elem);
            } else {
                square_count += on as i32;
                match on {
                    true => v.push('X'),
                    false => v.push('.'),
                }
            }
            if elem != '-' && elem != '|' {
                prev = elem;
            }
        }
        vis_grid.push(v);
    }

    for v in vis_grid {
        println!("{:?}", v.iter().collect::<String>());
    }

    println!("Part 2: {}", square_count);


}

fn pipe(entry_dir: char, pipe_shape: char) -> (i32, i32) {
    match (entry_dir, pipe_shape) {
        ('E', '-') => (0, 1),
        ('W', '-') => (0, -1),
        ('N', '|') => (-1, 0),
        ('S', '|') => (1, 0),
        ('W', 'L') => (-1, 0),
        ('W', 'F') => (1, 0),
        ('S', 'L') => (0, 1),
        ('N', 'F') => (0, 1),
        ('N', '7') => (0, -1),
        ('S', 'J') => (0, -1),
        ('E', '7') => (1, 0),
        ('E', 'J') => (-1, 0),
        _ => (0, 0),
    }
}

fn get_entry_dir(piped: &(i32, i32)) -> char {
    match *piped {
        (0, 1) => 'E',
        (0, -1) => 'W',
        (1, 0) => 'S',
        (-1, 0) => 'N',
        _ => 'X',
    }
}