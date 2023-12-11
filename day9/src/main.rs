use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //let contents = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
    let predictions = solve(&contents, false);

    println!("PREDS {:?}", predictions);
    println!("Part 1: {}", predictions.iter().sum::<i32>());

    let predictions = solve(&contents, true);
    println!("Part 2: {}", predictions.iter().sum::<i32>());


}

fn solve(contents: &str, part2: bool) -> Vec<i32> {

    let mut predictions: Vec<i32> = Vec::new();

    for line in contents.lines() {

        let mut seq_veq = line.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if part2 {
            seq_veq = seq_veq.into_iter().rev().collect();
        }

        let mut seq_stack: Vec<Vec<i32>> = Vec::from([seq_veq]);

        let mut depth = 0;
        let mut current_seq = &seq_stack[depth];

        while current_seq.iter().map(|x| *x).collect::<HashSet<i32>>().len() != 1 {
            depth += 1;
            let mut delta_seq = Vec::new();

            for i in 0..current_seq.len()-1 {
                let delta = current_seq[i+1] - current_seq[i];
                delta_seq.push(delta);
            }

            seq_stack.push(delta_seq);
            current_seq = &seq_stack[depth];
        }

        let prediction: i32 = seq_stack.iter().rev().map(|x| x.iter().rev().nth(0).unwrap()).sum();
        predictions.push(prediction);
    }

    predictions
}