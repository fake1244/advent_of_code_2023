use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    
    let input = read_to_string("./input/day02-1.txt").expect("couldnt read file");
    for line in input.lines(){
        let (left, right) = line.split_once(": ").unwrap();
        let (_, gameid) = left.split_once(" ").unwrap();
        let mut possible = true;
        for round in right.split("; "){
            for block in round.split(", "){
                let (count, color) = block.split_once(" ").unwrap();
                let count: u64 = count.parse().unwrap_or(0);
                if (color == "red" && count > 12) || (color == "green" && count > 13) || (color == "blue" && count > 14){
                    possible = false;
                }
            }
        }

        if possible {
            sol1 += gameid.parse().unwrap_or(0);
        }
    }
    // SOLUTION 2
    let mut sol2: u64 = 0;

    for line in input.lines(){
        let (left, right) = line.split_once(": ").unwrap();
        let (_, gameid) = left.split_once(" ").unwrap();
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        for round in right.split("; "){
            for block in round.split(", "){
                let (count, color) = block.split_once(" ").unwrap();
                let count: u64 = count.parse().unwrap_or(0);
                if color == "red" && count > min_red {
                    min_red = count;
                }
                if color == "green" && count > min_green {
                    min_green = count;
                }
                if color == "blue" && count > min_blue {
                    min_blue = count;
                }
            }
        }
        // println!("{}", line);
        // println!("{} {} {}", min_red, min_green, min_blue);
        sol2 += min_red * min_green * min_blue;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
