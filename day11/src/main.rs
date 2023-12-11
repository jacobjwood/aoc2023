use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let file_path = "input.txt"; //"example.txt"; 
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    parse_input(&contents);
    parse_input_pt2(&contents, 1000000);
    

}

fn parse_input_pt2(contents: &str, insertion_size: usize) {

    // let mut space_grid = Vec::new();
    let mut galaxies = Vec::new();
    let mut rows_to_expand = Vec::new();
    let mut cols_to_expand = Vec::new();
    let mut space_grid = Vec::new();

    for (i, l) in contents.lines().enumerate() {
        let mut line_vec = Vec::new();
        let mut galaxy_row_count = 0;
        for (j, c) in l.chars().enumerate() {
            line_vec.push(c);
            if c == '#' {
                galaxy_row_count += 1;
                galaxies.push((i, j));
            }
        }
        space_grid.push(line_vec);
        if galaxy_row_count == 0 {
            rows_to_expand.push(i);
        }
    }

    // now columns to expand
    for j in 0..space_grid[0].len() {
        let mut galaxy_column_count = 0;
        for i in 0..space_grid.len() {
            if space_grid[i][j] == '#' {
                galaxy_column_count += 1;
            }
        }
        if galaxy_column_count == 0 {
            cols_to_expand.push(j);
        }
    }

    // for row in &space_grid {
    //     let mut row_string = String::new();
    //     for j in row {
    //         row_string.push(*j);
    //     }
    //     println!("{}", row_string);
    // }

    // println!("RTE {:?}", rows_to_expand);
    // println!("CTE {:?}", cols_to_expand);

    for g in 0..galaxies.len() {
        let mut row_add = 0;
        let mut col_add = 0;
        for r in &rows_to_expand {
            if *r <= galaxies[g].0 {
                row_add += std::cmp::max(1, insertion_size - 1);
            }
        }

        for c in &cols_to_expand {
            if *c <= galaxies[g].1 {
                col_add += std::cmp::max(1, insertion_size - 1);
            }
        }

        galaxies[g] = (galaxies[g].0 + row_add, galaxies[g].1 + col_add);
    }
    
    // println!("Galaxies: {:?}", galaxies);

    let mut distance_tot = 0;
    while galaxies.len() != 1 {
        let g = galaxies.pop().unwrap();
        for remaining in galaxies.iter() {
            let md = manhattan_distance(g, *remaining);
            distance_tot += md;
            // min_dist = std::cmp::min(md, min_dist);
        }
        
    } 
    println!("Part 2: {}", distance_tot);
    // galaxies
}

fn parse_input(contents: &str) {

    let mut space_grid = Vec::new();

    for (i, l) in contents.lines().enumerate() {
        let mut line_vec = Vec::new();
        let mut galaxy_row_count = 0;
        for (j, c) in l.chars().enumerate() {
            line_vec.push(c);
            if c == '#' {
                galaxy_row_count += 1;
            }
        }
        space_grid.push(line_vec);
        if galaxy_row_count == 0 {
            space_grid.push(l.chars().map(|c| '.').collect::<Vec<_>>());
        }
    }

    // now expand columns 
    let mut columns_to_expand = Vec::new();
    for j in 0..space_grid[0].len() {
        let mut galaxy_column_count = 0;
        for i in 0..space_grid.len() {
            if space_grid[i][j] == '#' {
                galaxy_column_count += 1;
            }
        }
        if galaxy_column_count == 0 {
            columns_to_expand.push(j + columns_to_expand.len());
        }
    }

    for j in columns_to_expand {
        for i in 0..space_grid.len() {
            space_grid[i].insert(j, '.');
        }
    }

    // println!("EXPANDED SPACE: ");
    // for row in &space_grid {
    //     let mut row_string = String::new();
    //     for j in row {
    //         row_string.push(*j);
    //     }
    //     println!("{}", row_string);
    // }

    // collect positions of galaxies
    let mut galaxies = Vec::new();
    for (i, line) in space_grid.iter().enumerate() {
        for (j, pos) in line.iter().enumerate() {
            if *pos == '#' {
                galaxies.push((i, j));
            }
        }
    }

    println!("Galaxies: {:?}", galaxies);

    let mut distance_tot = 0;
    while galaxies.len() != 1 {
        let g = galaxies.pop().unwrap();
        let mut min_dist = 10000000;
        for remaining in galaxies.iter() {
            let md = manhattan_distance(g, *remaining);
            distance_tot += md;
            // min_dist = std::cmp::min(md, min_dist);
        }
        
    } 

    println!("{}", manhattan_distance((2, 0), (7, 12)));
    println!("Part 1: {}", distance_tot);
    // galaxies
}

fn manhattan_distance(g1: (usize, usize), g2: (usize, usize)) -> usize {
    let i_dist = std::cmp::max(g1.0, g2.0) - std::cmp::min(g1.0, g2.0);
    let j_dist = std::cmp::max(g1.1, g2.1) - std::cmp::min(g1.1, g2.1);

    i_dist + j_dist
}
