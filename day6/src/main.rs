use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let times: Vec<usize> = contents.lines().nth(0).unwrap().split(':').nth(1).unwrap().split(' ').filter(|x| !x.is_empty()).map(|x| x.trim().parse::<usize>().unwrap()).collect();
    let distances: Vec<usize> = contents.lines().nth(1).unwrap().split(':').nth(1).unwrap().split(' ').filter(|x| !x.is_empty()).map(|x| x.trim().parse::<usize>().unwrap()).collect();
    let time_part_2 = contents.lines().nth(0).unwrap().split(':').nth(1).unwrap().chars().filter(|x| *x != ' ').collect::<String>().parse::<usize>().unwrap();
    let distance_part_2 = contents.lines().nth(1).unwrap().split(':').nth(1).unwrap().chars().filter(|x| *x != ' ').collect::<String>().parse::<usize>().unwrap();

    let mut ways_to_beat_product = 1;

    for (t, d) in std::iter::zip(times, distances) {
        let mut ways_to_win = 0;
        // There will be a range between times that we need to count
        // n * (t - n) > d
        for n in 0..t {
            if n * (t - n) > d {
                ways_to_win += 1;
            }
        }

        ways_to_beat_product *= ways_to_win;
    }

    println!("Part 1: {}", ways_to_beat_product);

    let mut ways_to_win = 0;
    for n in 0..time_part_2 {
        if n * (time_part_2 - n) > distance_part_2 {
            ways_to_win += 1;
        }
    }

    println!("Part 2: {}", ways_to_win);
}
