use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp::min;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = u64::MAX;
    
    let input = read_to_string("./input/day17.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    let mut grid = Vec::new();
    for line in input.lines(){
        let mut arr = Vec::new();
        for c in line.chars(){
            arr.push(c.to_digit(10).unwrap() as u64);
        }
        grid.push(arr);
    }

    let mut visited = HashMap::new();
    let mut to_visit = Vec::new();
    to_visit.push((0, 0, 0, "", 0));
    
    while let Some((x, y, cost, dir, in_a_row)) = to_visit.pop() {
        // println!("visit {} {} cost: {} dir: {} in_row: {}", x, y, cost, dir, in_a_row);
        if in_a_row > 3 {
            continue;
        }
        if visited.contains_key(&(x, y, dir, in_a_row)) {
            continue;
        }
        if (x, y) == (grid.len() - 1, grid[0].len() - 1) {
            sol1 = cost;
            break;
        }

        visited.insert((x, y, dir, in_a_row), cost);

        if x - 1 < grid.len() && dir != "right" {
            let cost = cost + grid[x-1][y];
            if dir == "left" { to_visit.push((x - 1, y, cost, dir, in_a_row + 1)); } else { to_visit.push((x-1, y, cost, "left", 1)); }
        }
        if x + 1 < grid.len() && dir != "left" {
            let cost = cost + grid[x+1][y];
            if dir == "right" { to_visit.push((x + 1, y, cost, dir, in_a_row + 1)); } else { to_visit.push((x+1, y, cost, "right", 1)); }
        }
        if y - 1 < grid.len() && dir != "down" {
            let cost = cost + grid[x][y-1];
            if dir == "up" { to_visit.push((x, y - 1, cost, dir, in_a_row + 1)); } else { to_visit.push((x, y-1, cost, "up", 1)); }
        }
        if y + 1 < grid.len() && dir != "up" {
            let cost = cost + grid[x][y+1];
            if dir == "down" { to_visit.push((x, y + 1, cost, dir, in_a_row + 1)); } else { to_visit.push((x, y+1, cost, "down", 1)); }
        }
        to_visit.sort_by_key(|x| x.2);
        to_visit = to_visit.into_iter().rev().collect::<Vec<(_)>>();
        // println!("{:?}", to_visit);
    }

    // sol1 = min(sol1, *visited.get(&(grid.len() - 1, grid[0].len() - 1, "right", 1)).unwrap_or(&u64::MAX));
    // sol1 = min(sol1, *visited.get(&(grid.len() - 1, grid[0].len() - 1, "right", 2)).unwrap_or(&u64::MAX));
    // sol1 = min(sol1, *visited.get(&(grid.len() - 1, grid[0].len() - 1, "right", 3)).unwrap_or(&u64::MAX));
    // sol1 = min(sol1, *visited.get(&(grid.len() - 1, grid[0].len() - 1, "down", 1)).unwrap_or(&u64::MAX));
    // sol1 = min(sol1, *visited.get(&(grid.len() - 1, grid[0].len() - 1, "down", 2)).unwrap_or(&u64::MAX));
    // sol1 = min(sol1, *visited.get(&(grid.len() - 1, grid[0].len() - 1, "down", 3)).unwrap_or(&u64::MAX));

    let mut sol2: u64 = 0;

    let mut visited = HashMap::new();
    let mut to_visit = Vec::new();
    to_visit.push((0, 0, 0, "right", 0));
    to_visit.push((0, 0, 0, "down", 0));
    
    while let Some((x, y, cost, dir, in_a_row)) = to_visit.pop() {
        // println!("visit {} {} cost: {} dir: {} in_row: {}", x, y, cost, dir, in_a_row);
        if in_a_row > 10 {
            continue;
        }
        if visited.contains_key(&(x, y, dir, in_a_row)) {
            continue;
        }
        if (x, y) == (grid.len() - 1, grid[0].len() - 1) {
            sol2 = cost;
            break;
        }

        visited.insert((x, y, dir, in_a_row), cost);

        if x - 1 < grid.len() && dir != "right" {
            let cost = cost + grid[x-1][y];
            if dir == "left" { to_visit.push((x - 1, y, cost, dir, in_a_row + 1)); } else if in_a_row >= 4 { to_visit.push((x-1, y, cost, "left", 1)); }
        }
        if x + 1 < grid.len() && dir != "left" {
            let cost = cost + grid[x+1][y];
            if dir == "right" { to_visit.push((x + 1, y, cost, dir, in_a_row + 1)); } else if in_a_row >= 4 { to_visit.push((x+1, y, cost, "right", 1)); }
        }
        if y - 1 < grid.len() && dir != "down" {
            let cost = cost + grid[x][y-1];
            if dir == "up" { to_visit.push((x, y - 1, cost, dir, in_a_row + 1)); } else if in_a_row >= 4 { to_visit.push((x, y-1, cost, "up", 1)); }
        }
        if y + 1 < grid.len() && dir != "up" {
            let cost = cost + grid[x][y+1];
            if dir == "down" { to_visit.push((x, y + 1, cost, dir, in_a_row + 1)); } else if in_a_row >= 4  { to_visit.push((x, y+1, cost, "down", 1)); }
        }
        to_visit.sort_by_key(|x| x.2);
        to_visit = to_visit.into_iter().rev().collect::<Vec<(_)>>();
        // println!("{:?}", to_visit);
    }

    (Solution::from(sol1), Solution::from(sol2))
}
