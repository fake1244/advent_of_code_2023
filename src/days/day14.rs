use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    
    let input = read_to_string("./input/day14.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut arr = Vec::new();
        for c in line.chars() {
            arr.push(c);
        }
        grid.push(arr);
    }
    let mut visited = HashMap::new();
    visited.insert(grid.clone(), 0);

    move_north(&mut grid);

    let sol1 = grid
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x.iter().fold(0, |acc1, x1| acc1 + if *x1 == 'O' { 1 + i } else { 0 }));

    let mut sol2: usize = 0;
    move_west(&mut grid);
    move_south(&mut grid);
    move_east(&mut grid);
    let mut cycle = 0;
    visited.insert(grid.clone(), 1);
    for i in 2 .. 1_000_000_000 { // start from 2 because 0 and one have been done already
        move_north(&mut grid);
        move_west(&mut grid);
        move_south(&mut grid);
        move_east(&mut grid);
        if visited.contains_key(&grid) {
            // iteration i and start are eaxtly the same 
            let start = visited.get(&grid).unwrap();
            // difference of iterations between i and start
            let period = i - start;
            // calculate where is 1 000 000 000 in the cycle
            let offset = (1_000_000_000 - start) % period;
            // find the correct grid
            for (grid, iteration) in &visited {
                if *iteration == start + offset {
                    sol2 = grid
                        .iter()
                        .rev()
                        .enumerate()
                        .fold(0, |acc, (i, x)| acc + x.iter().fold(0, |acc1, x1| acc1 + if *x1 == 'O' { 1 + i } else { 0 }));
                    break;
                }
            }
            break;
            // continue;
        }
        visited.insert(grid.clone(), i);
        // println!("{:?}", line);
    }
    // for line in &grid {
    //     println!("{:?}", line);
    // }
    (Solution::from(sol1), Solution::from(sol2))
}

fn move_north(grid: &mut Vec<Vec<char>>){
    for r in 1 .. grid.len() {
        for c in 0 .. grid[0].len() {
            if grid[r][c] == 'O'{
                move_point_north(grid, r, c);
            }
        }
    }
}
fn move_east(grid: &mut Vec<Vec<char>>){
    for r in 0 .. grid.len() {
        for c in (0 .. grid[0].len()-1).rev() {
            if grid[r][c] == 'O'{
                move_point_east(grid, r, c);
            }
        }
    }
}

fn move_south(grid: &mut Vec<Vec<char>>){
    for r in (0..grid.len()-1).rev() {
        for c in 0 .. grid[0].len() {
            if grid[r][c] == 'O'{
                move_point_south(grid, r, c);
            }
        }
    }
}

fn move_west(grid: &mut Vec<Vec<char>>){
    for r in 0 .. grid.len() {
        for c in (1 .. grid[0].len()) {
            if grid[r][c] == 'O'{
                move_point_west(grid, r, c);
            }
        }
    }
}

fn move_point_north(grid: &mut Vec<Vec<char>>, mut r: usize, c: usize) {
    for row in (0..r).rev() {
        if grid[row][c] != '.' {
            break;
        }
        grid[row][c] = 'O';
        grid[r][c] = '.';
        r -= 1;
    }
}

fn move_point_south(grid: &mut Vec<Vec<char>>, mut r: usize, c: usize) {
    for row in r+1..grid.len() {
        if grid[row][c] != '.' {
            break;
        }
        grid[row][c] = 'O';
        grid[r][c] = '.';
        r += 1;
    }
}

fn move_point_east(grid: &mut Vec<Vec<char>>, r: usize, mut c: usize) {
    for col in c+1..grid.len() {
        if grid[r][col] != '.' {
            break;
        }
        grid[r][col] = 'O';
        grid[r][c] = '.';
        c += 1;
    }
}

fn move_point_west(grid: &mut Vec<Vec<char>>, r: usize, mut c: usize) {
    for col in (0..c).rev() {
        if grid[r][col] != '.' {
            break;
        }
        grid[r][col] = 'O';
        grid[r][c] = '.';
        c -= 1;
    }
}