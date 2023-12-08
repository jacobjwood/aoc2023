use std::fs;
use std::collections::{HashSet, HashMap};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let instructions = contents.lines().nth(0).unwrap();
    let lr_map = contents.split("\n\n").nth(1).unwrap().lines().map(|l| (&l[0..3], (&l[7..10], &l[12..15]))).collect::<HashMap<_, _>>();

    println!("{:?}", lr_map);
    println!("{}", instructions);

    let mut step_count = 0;
    let mut current_position = "AAA";

    while current_position != "ZZZ" {
        let step = step_count % instructions.len();
        let step_to_take = &instructions[step..step+1];
        let options = lr_map.get(&current_position).unwrap();
        current_position = if step_to_take == "L" { options.0 } else { options.1 };
        step_count += 1;
    }

    println!("Part 1: {}", step_count);

    let mut positions = lr_map.keys().filter(|k| &k[2..] == "A").map(|k| String::from(*k)).collect::<Vec<_>>();

    // let mut step_counts = 0;
    // while positions.iter().filter(|k| &k[2..] == "Z").count() != positions.len() {
    //     let step = step_count % instructions.len();
    //     let step_to_take = &instructions[step..step+1];

    //     let mut new_positions = Vec::new();
    //     for pos in positions {
    //         let options = lr_map.get(&pos[..]).unwrap();
    //         let new_pos = if step_to_take == "L" { options.0 } else { options.1 };
    //         new_positions.push(String::from(new_pos));
    //     }
    //     positions = new_positions;
    //     step_count += 1; 
    // }
    
    let mut z_steps = Vec::new();
    for position in positions {
        let mut step_count = 0;
        let mut z_count = 0;
        println!("{}", position);
        let mut new_pos = "";
        loop {
            let step = step_count % instructions.len();
            let step_to_take = &instructions[step..step+1];

            let options = if step_count == 0 {
                lr_map.get(&position[..]).unwrap()
            } else {
                lr_map.get(&new_pos[..]).unwrap()
            };

            new_pos = if step_to_take == "L" { options.0 } else { options.1 };
            // println!("{} {} {} {:?} {:?}", position, new_pos, step_count, z_steps.len(), z_set.len());
            step_count += 1;

            if &new_pos[2..] == "Z" {
                z_steps.push(step_count);
                break;
            }
        }
    }

    z_steps.sort_by(|a, b| b.cmp(&a));
    println!("{:?}", z_steps);
    let mut z_max = z_steps[0];
    let lesser_z = &z_steps[1..];
    
    loop {
        let jeff = lesser_z.iter().map(|x| z_max % x).sum::<usize>();

        //println!("{}", jeff);
        if jeff == 0 { break; }
        // println!("{}", z_max);
        z_max += z_steps[0];
    }

    println!("Part 2: {}", z_max);

    



    // println!("Part 2: {}", step_count);
    

}
