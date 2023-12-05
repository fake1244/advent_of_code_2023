use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;

    let input = read_to_string("./input/day05.txt").expect("couldnt read file");

    let parts = input.split("\r\n\r\n");
    let mut seeds: Vec<u64> = Vec::new();
    let mut seeds2 = Vec::new();
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for (i, part) in parts.enumerate() {
        if i == 0 {
            // fill seeds vec
            seeds = part.split_once(": ").unwrap().1.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            seeds2 = seeds.clone();
            // println!("{:?}", seeds);
            continue;
        }
        maps.push(make_map(part));
        // println!("{} {:?}", i, maps);
    }
    // println!("{:?}", maps);
    let mut min_loc = u64::MAX;
    for seed in seeds {
        let mut value = seed;
        for map in &maps {
            // println!("{}", value);
            // println!("{:?}", map);
            for (source, dest, range) in map {
                if source <= &value && value < source + range {
                    value = dest + value - source;
                    break;
                }
            }
            // println!("new val {}", value);
        }
        // println!("location {}", value);
        if value < min_loc {
            min_loc = value;
        }
    }

    sol1 = min_loc;

    let mut sol2: u64 = 0;

    let mut seeds = Vec::new();
    seeds2.chunks(2).for_each(|x| seeds.push((x[0], x[1])));
    
    // println!("{:?}", seeds);
    min_loc = u64::MAX;
    for (start, range) in seeds {
        // println!("start {} range {}", start, range);
        for seed in start .. start + range {
            let mut value = seed;
            // println!("seed: {}", value);
            for map in &maps {
                // println!("value: {}", value);
                // println!("{:?}", map);
                for (source, dest, range) in map {
                    if source <= &value && value < source + range {
                        value = dest + value - source;
                        break;
                    }
                }
                // println!("new value: {}", value);
            }
            // println!("end value: {}", value);
            if value < min_loc {
                min_loc = value;
            }
        }
    }
    sol2 = min_loc;

    (Solution::from(sol1), Solution::from(sol2))
}


fn make_map(map_string: &str) -> Vec<(u64, u64, u64)> {
    let mut map = Vec::new();

    for (i, line) in map_string.lines().enumerate(){
        if i == 0 {continue;}

        let (dest, rest) = line.split_once(" ").unwrap();
        let (source, range) = rest.split_once(" ").unwrap();
        map.push((source.parse::<u64>().unwrap(), dest.parse::<u64>().unwrap(), range.parse::<u64>().unwrap()));

        // println!("{} {:?}", i, line);
    }

    return map;
}