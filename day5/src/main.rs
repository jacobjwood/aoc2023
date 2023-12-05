use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let seeds = contents.lines().nth(0).unwrap().split(":").nth(1).unwrap().split(' ').filter(|s| !s.is_empty()).map(|s| s.trim().parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut map_vec = seeds.iter().cloned();

    let mut old_map: Vec<usize> = seeds.iter().cloned().collect();
    let mut new_map: Vec<usize> = Vec::new();
    println!("{:?}", old_map);

    for o in &old_map {

        let mut new = *o;

        'maps: for map_text in contents.split("\n\n") {

            // let mut new_map: Vec<usize> = Vec::new();

            // println!("{}", o);
            // println!("{}", map_text);

            'lines: for (i, map_line) in map_text.lines().enumerate() {

                if i == 0 {
                    continue 'lines;
                }

                //println!("{}", map_line);

                // REMEMBER DESTINATION AND SOURCE RANGE ORDER 
                let parsed_numbers = map_line.split(' ').filter(|n| !n.is_empty()).map(|n| n.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
                if let [dest, source, range] = parsed_numbers[..] {
                    if (source..source+range).contains(&new) {
                        if *o == 14 {
                            println!("{} {} {} {}", dest, source, range, new);
                        }
                        new = dest + (new - source);
                        continue 'maps;
                    }
                }
            }
            if *o == 14 {
                println!("NEW: {}", new);
            }
        }

        println!("o {} n {}", o, new);

        new_map.push(new);
    }

    let part1 = new_map.iter().min().unwrap();
    println!("Part 1: {}", part1);



}

fn get_map(s: usize) {
    

}
