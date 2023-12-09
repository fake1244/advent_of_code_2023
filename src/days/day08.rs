use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let input = read_to_string("./input/day08.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");
    let (instructions, map) = input.split_once("\r\n\r\n").unwrap();
    let mut hashmap = HashMap::<String, (String, String)>::new();
    // println!("{:?}", instructions);
    
    for def in map.lines(){
        let (node, assignment) = def.split_once(" = ").unwrap();
        // println!("{:?}", node);
        let (left, right) = assignment.split_once(", ").unwrap();
        let left = left.strip_prefix("(").unwrap().to_string();
        let right = right.strip_suffix(")").unwrap().to_string();
        hashmap.insert(node.to_string(), (left, right));
    }

    let mut current_loc = "AAA";
    let mut step: u64 = 0;
    while current_loc != "ZZZ" {
        if instructions.chars().nth((step % instructions.len() as u64).try_into().unwrap()).unwrap() == 'L' {
            current_loc = &hashmap.get(current_loc).unwrap().0;
            step += 1;
        } else {
            current_loc = &hashmap.get(current_loc).unwrap().1;
            step += 1;
        }
    }

    sol1 = step;

    let mut sol2: u64 = 0;
    let mut start_nodes = hashmap.keys().map(|x| x.as_str()).filter(|x| x.ends_with("A")).collect::<Vec<&str>>();
    sol2 = start_nodes.into_iter().map(|x| {
        let mut step: u64 = 0;
        let mut current_loc = x;
        while !current_loc.ends_with("Z") {
            if instructions.chars().nth((step % instructions.len() as u64).try_into().unwrap()).unwrap() == 'L' {
                current_loc = &hashmap.get(current_loc).unwrap().0;
                step += 1;
            } else {
                current_loc = &hashmap.get(current_loc).unwrap().1;
                step += 1;
            }
        }
        step
    }).fold(1, |acc, x| lcm(acc, x));

    (Solution::from(sol1), Solution::from(sol2))
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					
