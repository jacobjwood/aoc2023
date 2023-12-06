use std::fs;
use std::collections::{HashSet, HashMap};

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

    part2(&contents);


}

fn part2(contents: &str) {

    let seed_ranges = contents.lines().nth(0).unwrap().split(":").nth(1).unwrap().trim().split(" ").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut min = 100000000;
    
    for r in (0..seed_ranges.len()).step_by(2) {
        println!("r {}", r);

        let mut current_stack : Vec<(usize, usize)> = Vec::from([(seed_ranges[r], seed_ranges[r+1])]);

        for (_, map_text) in contents.split("\n\n").enumerate().filter(|(i, m)| *i != 0) {

            let mut this_line_stack: Vec<(usize, usize)> = current_stack[..].iter().cloned().collect();
            let mut new_stack: Vec<(usize, usize)> = Vec::new();

            for (i, map_line) in map_text.lines().enumerate().filter(|(i, m)| *i != 0) {

                let parsed_numbers = map_line.split(' ').filter(|n| !n.is_empty()).map(|n| n.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
                let mut next_line_stack = Vec::new();

                let (dest, source, range) = (parsed_numbers[0], parsed_numbers[1], parsed_numbers[2]);

                // println!("TLS: {:?}", this_line_stack);

                for item in &this_line_stack {
                    // inclusive
                    let item_range = (item.0, item.0 + item.1 - 1);
                    let source_range = (source, source + range - 1);
                    let overlap = (std::cmp::max(item_range.0, source_range.0), std::cmp::min(item_range.1, source_range.1));
                    let full_dest_range = (dest, dest + range - 1);

                    // println!("RANGE: {:?} | MAP: {:?} | OVERLAP: {:?} | DEST: {:?}", item_range, source_range, overlap, full_dest_range);
                    if overlap.0 > overlap.1 {
                        next_line_stack.push((item_range.0, item_range.1 - item_range.0 + 1));
                        continue;
                    } else {

                        let dest_range = (overlap.0 - source_range.0 + dest, overlap.1 - overlap.0 + 1);
                        // println!("OVERLAP RANGE: {:?} | DEST RANGE: {:?}", overlap, dest_range);

                        new_stack.push(dest_range);
                        //println!("NS {:?}", new_stack);

                        if overlap.0 > item_range.0 {
                            next_line_stack.push((item_range.0, overlap.0 - item_range.0));
                        }
    
                        if overlap.1 < item_range.1 {
                            next_line_stack.push((overlap.1 + 1, item_range.1 - overlap.1 + 1));
                        }

                        // println!("NLS: {:?}", next_line_stack);

                    }
                    

                }
                
                this_line_stack = next_line_stack;
                
                //let item_end = seed_ranges[r], seed_ranges[r] + seed_ranges[r+1], source, source+range-1seed_ranges[r] + seed_ranges[r+1], source, source+range-1
                //println!("SEED RANGE: ({} {}) SOURCE RANGE: ({} {})", );

                
            }

            // map remainders over
            for l in &this_line_stack {
                // println!("L0 L1 {} {}", l.0, l.1);
                new_stack.push(*l);
            }
        
            // println!("NS: {:?}", new_stack);
            current_stack = new_stack;
        }

        // println!("CS {:?}", current_stack);
        let seed_min = current_stack.iter().map(|x| x.0).min().unwrap();
        min = std::cmp::min(min, seed_min);

    }

    println!("Part 2: {}", min);

}

fn part2x(contents: &str) {

    let seed_ranges = contents.lines().nth(0).unwrap().split(":").nth(1).unwrap().trim().split(" ").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut min_per_range: Vec<usize> = Vec::new();
    println!("{:?}", seed_ranges);
    for r in (0..seed_ranges.len()).step_by(2) {

        // initialise
        let mut old = Vec::from([(seed_ranges[r], seed_ranges[r+1])]);

        println!("R: {}", r);
        

        'maps: for map_text in contents.split("\n\n") {

            println!("OLD LEN: {}", old.len());

            let mut new: Vec<(usize, usize)> = Vec::new();

            // INFINITE LOOPS 
            
            while let Some(x) = old.pop() {

                
                
                let mut loop_stack = Vec::from([x]);
                let mut seen_set: HashSet<(usize, usize)> = HashSet::new();

                while let Some(y) = loop_stack.pop() {

                    println!("Y: {:?}", y);
                    if y == (0, 0) {
                        //continue;
                    }
                    let input_min = y.0;
                    let input_max = y.0 + y.1 - 1;

                    'lines: for (i, map_line) in map_text.lines().enumerate() {

                        if i == 0 {
                            continue 'lines;
                        }

                        println!("{}", map_line);

                        // KEEP TRACK OF RANGES THAT DON'T MATCH and process in next line. 
                        let parsed_numbers = map_line.split(' ').filter(|n| !n.is_empty()).map(|n| n.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
                        if let [dest, source, range] = parsed_numbers[..] {

                            let s_min = source;
                            let s_max = source+range-1;
                            

                            // Map what we can to new
                            // otherwise stick back on old stack
                            let map_min = std::cmp::min(s_max, std::cmp::max(s_min, input_min));
                            let map_max = std::cmp::max(s_min, std::cmp::min(s_max, input_max));

                            // println!("MAP MIN MAP MAX: {} {}", map_min, map_max);

                            
                            println!("PUSHING");
                            new.push((dest + map_min - s_min, dest + map_max - s_min));
                            seen_set.insert((map_min, map_max));
                                

                            // Remainders
                            if map_min != input_min {
                                let rem_below = (input_min, map_min);
                                if !seen_set.contains(&rem_below) {
                                    loop_stack.push(rem_below);
                                    seen_set.insert(rem_below);
                                }
                            }

                            // println!("MAP MAX {}", map_max);
                            if map_max != input_max {
                                let rem_above = (map_max+1, input_max);
                                // println!("MIN MAX {} {} -- {:?}", input_min, input_max, rem_above);
                                println!("LEN SEEN SET {}", seen_set.len());
                                // println!("{}", seen_set.contains(&rem_above));
                                if !seen_set.contains(&rem_above) {
                                    // println!("PUSHING");
                                    loop_stack.push(rem_above);
                                    seen_set.insert(rem_above);
                                }
                            
                            }

                        }

                    }
                    // whatever is left, just add to new
                    // new.push(y);
                }

                
            }

            // Assign old as new for new mapping iteration
            old = new;
        
        }

        let min = old.iter().map(|x| x.0).min().unwrap();
        min_per_range.push(min);
    }
    
    let min = min_per_range.iter().min().unwrap();
    println!("Part 2: {}", min);
}
