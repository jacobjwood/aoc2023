use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1: u32 = contents
        .lines()
        .map(|l| {
            l.split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|x| {
                    x.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<HashSet<_>>>()
        })
        .map(|v| 2_u32.pow(v[0].intersection(&v[1]).count() as u32) / 2)
        .sum();

    println!("Part 1: {}", part1);

    let part2 = part2(&contents);
    println!("Part 2: {}", part2);
}

fn part2(contents: &str) -> usize {
    let match_map = contents
        .lines()
        .map(|l| {
            l.split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|x| {
                    x.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<HashSet<_>>>()
        })
        .map(|v| v[0].intersection(&v[1]).count() as usize)
        .enumerate()
        .map(|(i, m)| (i + 1, m))
        .collect::<HashMap<usize, usize>>();

    let max_game = match_map.iter().map(|(i, _)| i).max().unwrap();

    let mut game_stack = match_map.iter().map(|(i, _)| *i).collect::<Vec<usize>>();

    let mut card_total = 0;

    while game_stack.len() != 0 {
        let card = game_stack.pop().unwrap();
        let matches = match_map.get(&card).unwrap();
        for m in 1..=*matches {
            let extra_game = card + m;
            if extra_game <= *max_game {
                game_stack.push(extra_game);
            }
        }
        card_total += 1;
    }

    card_total
}
