use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    
    let input = read_to_string("./input/day12.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    for line in input.lines() {
        let (line, conditions) = line.split_once(" ").unwrap();
        let conditions = conditions.split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let line = line.chars().collect::<Vec<char>>();
        // println!("{:?} {:?}", line, conditions);
        let mut saved = HashMap::new();
        let ways = calculate_ways(line, conditions, false, &mut saved);
        // println!("{}", ways);
        // println!("{:?}", saved);
        sol1 += ways;
    }

    let mut sol2: u64 = 0;

    for line in input.lines() {
        let (line, conditions) = line.split_once(" ").unwrap();
        let conditions = conditions.split(",").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let line = [line.clone(), line.clone(), line.clone(), line.clone(), line.clone()].join("?");
        let conditions = [conditions.clone(),conditions.clone(),conditions.clone(),conditions.clone(),conditions.clone()].concat();
        let line = line.chars().collect::<Vec<char>>();
        // println!("{:?} {:?}", line, conditions);
        let mut saved = HashMap::new();
        let ways = calculate_ways(line, conditions, false, &mut saved);
        // println!("{:?}", saved);
        sol2 += ways;
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn calculate_ways(mut line: Vec<char>, mut conditions: Vec<u64>, continuous: bool, saved: &mut HashMap<(Vec<char>,Vec<u64>), u64>) -> u64 {
    if conditions.is_empty(){
        if !line.iter().any(|x| *x == '#') {
            return 1;
        } else {
            return 0;
        }
    }
    if line.is_empty() {
        return 0;
    }
    if let Some(res) = saved.get(&(line.clone(), conditions.clone())){
        // println!("{:?} {:?} {}", line, conditions, res);
        return *res;
    }
    let line_save = line.clone();
    let conditions_save = conditions.clone();
    let mut res = 0;
    // println!("line {:?} {:?}", line, conditions);
    if let Some(ch) = line.pop() {
        match ch {
            '.' => {
                if continuous { return 0; }
                res = calculate_ways(line, conditions, false, saved);
            },
            '#' => {
                let mut cond = conditions.iter_mut().last().unwrap();
                let mut continuous = true;
                *cond -= 1;
                if *cond == 0 {
                    conditions.pop();
                    if let Some(ch) = line.pop() {
                        if ch == '#'{
                            return 0;
                        }
                    }
                    continuous = false;
                } 
                res = calculate_ways(line, conditions, continuous, saved);
            },
            '?' => {
                let mut line_clone = line.clone();
                line_clone.push('.');
                line.push('#');
                // println!("try line {:?} {:?}", line_clone, conditions);
                res += calculate_ways(line_clone, conditions.clone(), continuous, saved);
                // println!("after {}", res);
                // println!("try line {:?} {:?}", line, conditions);
                res += calculate_ways(line, conditions, continuous, saved);
                // println!("after{}", res);
            },
            _ => {
                panic!();
            },
        }
    }
    saved.insert((line_save, conditions_save), res);
    res
}
