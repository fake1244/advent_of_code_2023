use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: i64 = 0;
    let input = read_to_string("./input/day09.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    for line in input.lines(){
        let seq = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        // println!("{:?}", seq);
        let expanded_pattern = expand(seq);
        let prediction = predict(expanded_pattern);
        sol1 += prediction;
    }
    let sol1: u64 = sol1.try_into().unwrap();

    let mut sol2: i64 = 0;
    for line in input.lines(){
        let mut seq = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        // println!("{:?}", seq);
        seq.reverse();
        let expanded_pattern = expand(seq);
        let prediction = predict(expanded_pattern);
        sol2 += prediction;
    }
    let sol2: u64 = sol2.try_into().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

fn expand(seq: Vec<i64>) -> Vec<Vec<i64>>{
    let mut expanded = vec![seq.clone()];

    let mut current_seq = seq;
    while !current_seq.iter().all(|x| *x == 0) {
        let mut diff: Vec<i64> = Vec::new();
        for w in current_seq.windows(2){
            let a = w[0];
            let b = w[1];
            diff.push(b - a);
        }
        expanded.push(diff.clone());
        current_seq = diff;
        // println!("{:?}", current_seq);
    }

    expanded
}

fn predict(exp_seq: Vec<Vec<i64>>) -> i64 {
    let mut res = 0;
    for seq in exp_seq {
        res += seq.last().unwrap();
    }
    res
    // unimplemented!();
}