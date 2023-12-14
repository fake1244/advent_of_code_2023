use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::iter::zip;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: usize = 0;
    let mut sol2: usize = 0;
    let input = read_to_string("./input/day13.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    for pattern in input.split("\r\n\r\n") {
        let pattern = pattern.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let (r, c) = solve_pattern(&pattern, false, (0,0));
        sol1 += r * 100 + c;
        let (r, c) = fix_and_solve_pattern(pattern, (r, c));
        sol2 += r * 100 + c;
    }

    (Solution::from(sol1), Solution::from(sol2))
}


fn solve_pattern(pattern: &Vec<Vec<char>>, part2: bool, last_val: (usize, usize)) -> (usize, usize) {
    for line_refl in 1..pattern.len() {
        if zip((0..line_refl).rev(), line_refl..pattern.len()).all(|(l, h)| pattern[l] == pattern[h]){
            if part2 && (line_refl, 0) == last_val{
                continue;
            }
            return (line_refl, 0);
        }
    }

    let pattern: Vec<Vec<char>> = (0..pattern[0].len()).map(|col| {
        (0..pattern.len())
            .map(|row| pattern[row][col])
            .collect()
    }).collect();

    for line_refl in 1..pattern.len() {
        if zip((0..line_refl).rev(), line_refl..pattern.len()).all(|(l, h)| pattern[l] == pattern[h]){
            if part2 && (0, line_refl) == last_val{
                continue;
            }
            return (0, line_refl);
        }
    }

    (0, 0)
}


fn fix_and_solve_pattern(pattern: Vec<Vec<char>>, last_res: (usize, usize)) -> (usize, usize) {
    let mut new_pattern = pattern.clone();
    for r in 0..pattern.len() {
        for c in 0..pattern[r].len() {
            if pattern[r][c] == '#' {
                new_pattern[r][c] = '.';
                let new_res = solve_pattern(&new_pattern, true, last_res);
                if new_res != (0, 0) && new_res != last_res {
                    return new_res;
                }
                new_pattern[r][c] = '#';
            } 
            else if pattern[r][c] == '.' {
                new_pattern[r][c] = '#';
                let new_res = solve_pattern(&new_pattern, true, last_res);
                if new_res != (0, 0) && new_res != last_res {
                    return new_res;
                }
                new_pattern[r][c] = '.';
            }
        }
    }
    (0, 0)
}
