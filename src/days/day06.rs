use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 1;

    let input = read_to_string("./input/day06.txt").expect("couldnt read file");
    let mut lines = input.lines();
    let mut races_rec: Vec<(u64, u64)> = vec![];

    let sec_arr = &lines.next().unwrap().split_ascii_whitespace().collect::<Vec<&str>>()[1..];
    let dist_arr = &lines.next().unwrap().split_ascii_whitespace().collect::<Vec<&str>>()[1..];
    for i in 0 .. sec_arr.len() {
        races_rec.push((sec_arr[i].parse::<u64>().unwrap(), dist_arr[i].parse::<u64>().unwrap()));
    }

    for (sec, dist_winning) in races_rec {
        let mut n_ways = 0;
        for i in 1 .. sec - 1 {
            let speed = i;
            let dist = speed * (sec - i);
            if dist > dist_winning {
                n_ways += 1;
            }
        }
        sol1 *= n_ways;
    }

    let mut sol2: u64 = 0;
    let real_sec = sec_arr.concat().parse::<u64>().unwrap();
    let real_dist = dist_arr.concat().parse::<u64>().unwrap();
    for i in 1 .. real_sec - 1 {
        let speed = i;
        let dist = speed * (real_sec - i);
        if dist > real_dist {
            sol2 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}
