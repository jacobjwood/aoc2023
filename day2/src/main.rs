use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(contents: &str) -> usize {
    let mut total = 0;

    for game in contents.lines() {
        if let [id, draws] = &game.split(": ").collect::<Vec<&str>>()[..] {
            let possible = draws
                .split("; ")
                .map(|draw| {
                    let (mut r, mut g, mut b) = (0, 0, 0);

                    for cube_draw in draw.split(", ") {
                        if let [n, c] = &cube_draw.split(" ").collect::<Vec<&str>>()[..] {
                            let n_parsed = n.parse::<usize>().unwrap();
                            match *c {
                                "red" => r += n_parsed,
                                "green" => g += n_parsed,
                                "blue" => b += n_parsed,
                                _ => (),
                            }
                        }
                    }
                    r <= 12 && g <= 13 && b <= 14
                })
                .all(|x| x) as usize;

            total += possible * id.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        }
    }
    total
}

fn part2(contents: &str) -> usize {
    let mut total_power = 0;

    for game in contents.lines() {
        let draws = &game.split(": ").nth(1).unwrap();

        let (mut rmax, mut gmax, mut bmax) = (0, 0, 0);

        for draw in draws.split("; ") {
            let (mut r, mut g, mut b) = (0, 0, 0);

            for cube_draw in draw.split(", ") {
                if let [n, c] = &cube_draw.split(" ").collect::<Vec<&str>>()[..] {
                    let n_parsed = n.parse::<usize>().unwrap();
                    match *c {
                        "red" => r += n_parsed,
                        "green" => g += n_parsed,
                        "blue" => b += n_parsed,
                        _ => (),
                    }
                }
            }
            rmax = std::cmp::max(rmax, r);
            gmax = std::cmp::max(gmax, g);
            bmax = std::cmp::max(bmax, b);
        }

        total_power += rmax * gmax * bmax;
    }
    total_power
}
