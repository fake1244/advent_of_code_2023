use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let input = read_to_string("./input/day16.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    let mut grid = Vec::new();
    for line in input.lines(){
        let mut arr = Vec::new();
        for c in line.chars(){
            arr.push(c);
        }
        grid.push(arr);
    }
    
    let mut visited = HashSet::new();
    let mut energized = HashSet::new();
    calculate_ray((0,0), "right", &grid, &mut energized, &mut visited);

    let sol1 = energized.len();
    let mut sol2 = 0;
    for c in 0 .. grid.len() {
        let mut visited = HashSet::new();
        let mut energized = HashSet::new();
        calculate_ray((0,c), "down", &grid, &mut energized, &mut visited);
        let sol = energized.len();
        if sol > sol2 { sol2 = sol; }

        let mut visited = HashSet::new();
        let mut energized = HashSet::new();
        calculate_ray((grid.len()-1,c), "up", &grid, &mut energized, &mut visited);
        let sol = energized.len();
        if sol > sol2 { sol2 = sol; }
    }

    for r in 0 .. grid[0].len() {
        let mut visited = HashSet::new();
        let mut energized = HashSet::new();
        calculate_ray((r,0), "right", &grid, &mut energized, &mut visited);
        let sol = energized.len();
        if sol > sol2 { sol2 = sol; }

        let mut visited = HashSet::new();
        let mut energized = HashSet::new();
        calculate_ray((r,grid.len()-1), "left", &grid, &mut energized, &mut visited);
        let sol = energized.len();
        if sol > sol2 { sol2 = sol; }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn calculate_ray<'a>(start: (usize, usize), dir: &'a str, grid: &Vec<Vec<char>>, energized: &mut HashSet<(usize, usize)>, visited: &mut HashSet<(usize, usize, &'a str)>) {
    // println!("calculate {} {} dir: {}", start.0, start.1, dir);
    if start.0 >= grid.len() || start.1 >= grid.len() {
        return;
    }
    
    match dir{
        "left" => {
            for c in (0 .. start.1 + 1).rev() {
                if visited.contains(&(start.0, c, dir)){
                    return;
                }
                visited.insert((start.0, c, dir));
                energized.insert((start.0, c));

                match grid[start.0][c] {
                    '/' => {
                        return calculate_ray((start.0 + 1, c), "down", grid, energized, visited);
                    },
                    '\\' => {
                        return calculate_ray((start.0 - 1, c), "up", grid, energized, visited);
                    },
                    '|' => {
                        calculate_ray((start.0 - 1, c), "up", grid, energized, visited);
                        return calculate_ray((start.0 + 1, c), "down", grid, energized, visited);
                    },
                    '.' => {
                        continue;
                    },
                    '-' => {
                        continue;
                    },
                    _ => panic!("not a symbol"),
                }
            }
        },
        "right" => {
            for c in (start.1 .. grid[0].len()) {
                if visited.contains(&(start.0, c, dir)){
                    return;
                }
                visited.insert((start.0, c, dir));
                energized.insert((start.0, c));

                match grid[start.0][c] {
                    '/' => {
                        return calculate_ray((start.0 - 1, c), "up", grid, energized, visited);
                    },
                    '\\' => {
                        return calculate_ray((start.0 + 1, c), "down", grid, energized, visited);
                    },
                    '|' => {
                        calculate_ray((start.0 - 1, c), "up", grid, energized, visited);
                        return calculate_ray((start.0 + 1, c), "down", grid, energized, visited);
                    },
                    '.' => {
                        continue;
                    },
                    '-' => {
                        continue;
                    },
                    _ => panic!("not a symbol"),
                }
            }
        },
        "down" => {
            for r in (start.0 .. grid.len()) {
                if visited.contains(&(r, start.1, dir)){
                    return;
                }
                visited.insert((r, start.1, dir));
                energized.insert((r, start.1));

                match grid[r][start.1] {
                    '/' => {
                        return calculate_ray((r, start.1 - 1), "left", grid, energized, visited);
                    },
                    '\\' => {
                        return calculate_ray((r, start.1 + 1), "right", grid, energized, visited);
                    },
                    '-' => {
                        calculate_ray((r, start.1 - 1), "left", grid, energized, visited);
                        return calculate_ray((r, start.1 + 1), "right", grid, energized, visited);
                    },
                    '|' => {
                        continue;
                    },
                    '.' => {
                        continue;
                    },
                    _ => panic!("not a symbol"),
                }
            }
        }
        "up" => {
            for r in (0 .. start.0 + 1).rev() {
                if visited.contains(&(r, start.1, dir)){
                    return;
                }
                visited.insert((r, start.1, dir));
                energized.insert((r, start.1));

                match grid[r][start.1] {
                    '/' => {
                        // println!("hello?");
                        return calculate_ray((r, start.1 + 1), "right", grid, energized, visited);
                    },
                    '\\' => {
                        return calculate_ray((r, start.1 - 1), "left", grid, energized, visited);
                    },
                    '-' => {
                        calculate_ray((r, start.1 - 1), "left", grid, energized, visited);
                        return calculate_ray((r, start.1 + 1), "right", grid, energized, visited);
                    },
                    '|' => {
                        continue;
                    },
                    '.' => {
                        continue;
                    },
                    _ => panic!("not a symbol"),
                }
            }
        }
        _ => panic!("not a direction!")
    }

}
