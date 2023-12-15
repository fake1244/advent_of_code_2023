use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let input = read_to_string("./input/day15.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    let mut boxes: HashMap<u64, Box> = HashMap::new();
    (0..256).for_each(|i| {
        boxes.insert(i, Box::new());
    });
    for instr in input.split(',') {
        sol1 += hash_str(instr);
        let mut splitted = instr.split_inclusive(['=',',']);
        let l_and_s = splitted.next().unwrap();
        let label = l_and_s.get(0..l_and_s.len()-1).unwrap();
        let sign = l_and_s.get(l_and_s.len()-1..l_and_s.len()).unwrap();
        
        match sign {
            "=" => {
                let focal_lenght = splitted.next().unwrap().parse::<u64>().unwrap();
                let mut my_box = boxes.get_mut(&hash_str(label)).unwrap();
                if !my_box.values.contains_key(label){
                    my_box.order.push(label);
                }
                my_box.values.insert(label, focal_lenght);
                // let mut box_values = boxes.get_mut(&hash_str(label)).unwrap();
                // match box_values.values.get_mut(label) {
                //     Some(ref mut value) => println!("{:?}", value),
                //     None => {
                //         box_values.values.insert(label, (box_values.lenses, focal_lenght));
                //         box_values.lenses += 1;
                //     },
                // }
                // println!("{:?} {:?} {}", label, sign, focal_lenght);
                
                // println!("{:?}", my_box);
            },
            "-" => {
                let mut my_box = boxes.get_mut(&hash_str(label)).unwrap();
                my_box.values.remove(label).map(|_| {
                    let index = my_box.order.iter().position(|x| *x == label).unwrap();
                    my_box.order.remove(index);
                });
                
                // println!("{:?}", my_box);
            } ,
            _ => {
                println!("{}", sign);
                panic!("WTF")
            }
        }
    }
    let mut sol2: u64 = 0;
    for (i, my_box) in boxes.iter() {
        if !my_box.order.is_empty() {
            // println!("{} {:?}", i, my_box);
        }
        for (order, label) in my_box.order.iter().enumerate() {
            let length = my_box.values.get(label).unwrap();
            let power = (*i + 1) * ((order + 1) as u64) * length;
            // println!("{} {} {}", order, length, power);
            sol2 += power;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn hash_str(string: &str) -> u64 {
    let mut hash: u64 = 0;
    for c in string.to_string().chars() {
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }
    // println!("{}", hash);
    hash
}
#[derive(Debug)]
struct Box<'a> {
    // id: usize,
    order: Vec<&'a str>,
    // lenses: u64,
    values: HashMap<&'a str, u64>,
}

impl Box<'_> {
    fn new() -> Self {
        Box {
            // lenses: 0,
            // id: id,
            order: Vec::new(),
            values: HashMap::new()
        }
    }
}