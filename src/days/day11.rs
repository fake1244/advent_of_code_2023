use crate::{Solution, SolutionPair};
use std::fs::read_to_string;


///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: usize = 0;
    let input = read_to_string("./input/day11.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    let mut grid = Vec::new();
    let mut add_line = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut l = Vec::new();
        for (x, c) in line.chars().enumerate() {
            l.push(c);
        }
        if l.iter().all(|&x| x == '.') {
            add_line.push(y);
        }
        grid.push(l);
    }
    let mut add_col = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    for col in 0..cols {
        if (0..rows).map(|row| grid[row][col]).all(|x| x == '.') {
            add_col.push(col)
        }
    }
    let mut galaxies = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((x, y));
            }
        }
    }
    let mut galaxies2 = galaxies.clone();
    let added_space = 1;
    for mut galaxy in &mut galaxies {
        let mut add_x = 0;
        let mut add_y = 0;
        for r in &add_line {
            if *r < galaxy.1 {
                add_y += added_space;
            }
        }
        for c in &add_col {
            if *c < galaxy.0 {
                add_x += added_space;
            }
        }
        galaxy.0 += add_x;
        galaxy.1 += add_y;
    }
    // println!("{:?}", galaxies);
    for i in 0..galaxies.len() {
        for j in i + 1 .. galaxies.len() {
            let galaxy = galaxies[i];
            let galaxy2 = galaxies[j];
            let diff = galaxy.0.abs_diff(galaxy2.0) + galaxy.1.abs_diff(galaxy2.1);
            // println!("{:?} - {:?} = {}", galaxy, galaxy2, diff);
            sol1 += diff;
        }
    }
    // for row in grid {
    //     println!("{:?}", row);
    // }
    // println!("{:?}", galaxies);

    let mut sol2: usize = 0;
    let added_space = 999999;
    for mut galaxy in &mut galaxies2 {
        let mut add_x = 0;
        let mut add_y = 0;
        for r in &add_line {
            if *r < galaxy.1 {
                add_y += added_space;
            }
        }
        for c in &add_col {
            if *c < galaxy.0 {
                add_x += added_space;
            }
        }
        galaxy.0 += add_x;
        galaxy.1 += add_y;
    }
    // println!("{:?}", galaxies);
    for i in 0..galaxies2.len() {
        for j in i + 1 .. galaxies2.len() {
            let galaxy = galaxies2[i];
            let galaxy2 = galaxies2[j];
            let diff = galaxy.0.abs_diff(galaxy2.0) + galaxy.1.abs_diff(galaxy2.1);
            // println!("{:?} - {:?} = {}", galaxy, galaxy2, diff);
            sol2 += diff;
        }
    }
    (Solution::from(sol1), Solution::from(sol2))
}
