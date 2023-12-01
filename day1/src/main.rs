use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let part1_one_liner : i32 =  contents.lines().map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>()).map(|s| (s.chars().nth(0).unwrap().to_string() + &s.chars().rev().nth(0).unwrap().to_string()).parse::<i32>().unwrap()).sum();
    println!("Part 1: {}", part1_one_liner);

    let mut parse_map = HashMap::<String, i32>::from([
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]);

    for k in 1..10 {
        parse_map.insert(String::from(k.to_string()), k);
    }

    let part2: i32 = contents.lines().map(|l| part2_parse_line(l, &parse_map)).sum();

    println!("Part 2: {}", part2);


}

fn part2_parse_line(line: &str, parse_map: &HashMap<String, i32>) -> i32 {

    let mut left_amount = 0;
    let mut right_amount = 0;
    let mut left_ptr = 0;
    let mut right_ptr = line.len();

    while (left_amount == 0) || (right_amount == 0) {
        
        let left_substr = line[0..left_ptr+1].to_string();
        let right_substr = line[right_ptr-1..].to_string();

        for key in parse_map.keys() {
            if left_substr.contains(key) && left_amount == 0 {
                left_amount += parse_map.get(key).unwrap();
            }
            if right_substr.contains(key) && right_amount == 0 {
                right_amount += parse_map.get(key).unwrap();
            }
        }

        right_ptr -= (right_amount == 0) as usize;
        left_ptr += (left_amount == 0) as usize;
    }

    (left_amount.to_string() + &right_amount.to_string()).parse::<i32>().unwrap()
}
