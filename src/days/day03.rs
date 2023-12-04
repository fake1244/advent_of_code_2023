use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let input = read_to_string("./input/day03.txt").expect("couldnt read file");

    for (i, line) in input.lines().enumerate(){
        let mut prev = "";
        if i != 0 {
            prev = input.lines().nth(i - 1).unwrap_or("");
        }
        let next = input.lines().nth(i + 1).unwrap_or("");

        let mut curr_number = String::from("");
        let mut connected = false;
        for (i, c) in line.chars().enumerate(){
            if c.is_digit(10) {
                curr_number.push(c);
            } else {
                if connected {
                    sol1 += curr_number.parse::<u64>().unwrap();
                }
                connected = false;
                curr_number = String::from("");
                continue;
            }

            if i == 0 {
                if prev != "" && (check_if_symbol(prev.chars().nth(i).unwrap()) || check_if_symbol(prev.chars().nth(i + 1).unwrap())) {
                    connected = true;
                }
                if next != "" && (check_if_symbol(next.chars().nth(i).unwrap()) || check_if_symbol(next.chars().nth(i + 1).unwrap())) {
                    connected = true;
                }
                if check_if_symbol(line.chars().nth(i + 1).unwrap()){
                    connected = true;
                }
            } else if i == line.chars().count() - 1 {
                if prev != "" && (check_if_symbol(prev.chars().nth(i).unwrap()) || check_if_symbol(prev.chars().nth(i - 1).unwrap())) {
                    connected = true;
                }
                if next != "" && (check_if_symbol(next.chars().nth(i).unwrap()) || check_if_symbol(next.chars().nth(i - 1).unwrap())) {
                    connected = true;
                }
                if check_if_symbol(line.chars().nth(i - 1).unwrap()){
                    connected = true;
                }
                if connected {
                    sol1 += curr_number.parse::<u64>().unwrap();
                    println!("{}", curr_number.parse::<u64>().unwrap());
                }
                connected = false;
                curr_number = String::from("");
            } else {
                if prev != "" && (check_if_symbol(prev.chars().nth(i).unwrap()) || check_if_symbol(prev.chars().nth(i + 1).unwrap()) || check_if_symbol(prev.chars().nth(i - 1).unwrap())) {
                    connected = true;
                }
                if next != "" && (check_if_symbol(next.chars().nth(i).unwrap()) || check_if_symbol(next.chars().nth(i + 1).unwrap()) || check_if_symbol(next.chars().nth(i - 1).unwrap())) {
                    connected = true;
                }
                if check_if_symbol(line.chars().nth(i - 1).unwrap()) || check_if_symbol(line.chars().nth(i + 1).unwrap()){
                    connected = true;
                }
            }
        }
    }

    // PART 2
    let mut sol2: u64 = 0;
    let mut gear_numbers: HashMap<(usize, usize), HashSet<u64>> = HashMap::new();

    // for (row, line) in input.lines().enumerate(){
    //     for (col, c) in line.chars().enumerate(){
    //         if c == '*' {
    //             gear_numbers.insert((row, col), HashSet::new());
    //         }
    //     }
    // }
    // println!("{:?}", gear_numbers);

    // for (row, line) in input.lines().enumerate(){
    //     let mut number = String::from("");
    //     let mut star_coord = Vec::new();

    //     for (col, c) in line.chars().enumerate(){
    //         if c.is_digit(10) {
    //             number.push(c);
    //             // save star coord
    //         } else {
    //             if number != String::from("") {
    //                 // parse number
    //                 // map star coord to this;
    //                 println!("{}", number);
    //                 number = String::from("");
    //                 let mut star_coord = Vec::new();
    //             }
    //         }
    //     }   

    // }


    for (row, line) in input.lines().enumerate(){
        let row = row.clone();
        let mut prev = "";
        if row != 0 {
            prev = input.lines().nth(row - 1).unwrap_or("");
        }
        let next = input.lines().nth(row + 1).unwrap_or("");

        let mut curr_number = String::from("");
        let mut star_coord: Vec<(usize, usize)> = Vec::new();
        let mut connected = false;
        for (col, c) in line.chars().enumerate(){
            let col = col.clone();
            if c.is_digit(10) {
                curr_number.push(c);
            } else {
                if connected {
                    for star in &star_coord {
                        match gear_numbers.get_mut(&star){
                            Some(v) => {v.insert(curr_number.parse::<u64>().unwrap());},
                            None    => {gear_numbers.insert(star.clone(), HashSet::from([curr_number.parse::<u64>().unwrap()]));}
                        }
                    }
                }
                connected = false;
                curr_number = String::from("");
                star_coord = Vec::new();
                continue;
            }

            if col == 0 {
                if prev != "" {
                    if check_star(prev.chars().nth(col).unwrap()){
                        star_coord.push((row - 1, col));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                    if check_star(prev.chars().nth(col + 1).unwrap()) {
                        star_coord.push((row - 1, col + 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                }
                if next != "" {
                    if check_star(next.chars().nth(col).unwrap()){
                        star_coord.push((row + 1, col));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                    if check_star(next.chars().nth(col + 1).unwrap()) {
                        star_coord.push((row + 1, col + 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }   
                }
                if check_star(line.chars().nth(col + 1).unwrap()){
                    star_coord.push((row, col + 1));
                    // println!("{} {}", row, col);
                    connected = true;
                }
            } else if col == line.chars().count() - 1 {
                if prev != "" {
                    if check_star(prev.chars().nth(col).unwrap()){
                        star_coord.push((row - 1, col));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                    if check_star(prev.chars().nth(col - 1).unwrap()) {
                        star_coord.push((row - 1, col - 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                }
                if next != "" {
                    if check_star(next.chars().nth(col).unwrap()){
                        star_coord.push((row + 1, col));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                    if check_star(next.chars().nth(col - 1).unwrap()) {
                        star_coord.push((row + 1, col - 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }   
                }
                if check_star(line.chars().nth(col - 1).unwrap()){
                    star_coord.push((row, col - 1));
                    // println!("{} {}", row, col);
                    connected = true;
                }
                if connected {
                    for star in &star_coord {
                        match gear_numbers.get_mut(&star){
                            Some(v) => {v.insert(curr_number.parse::<u64>().unwrap());},
                            None    => {gear_numbers.insert(star.clone(), HashSet::from([curr_number.parse::<u64>().unwrap()]));}
                        }
                    }
                }
                connected = false;
                curr_number = String::from("");
                
                star_coord = Vec::new();
            } else {
                if prev != "" {
                    if check_star(prev.chars().nth(col).unwrap()){
                        star_coord.push((row - 1, col));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                    if check_star(prev.chars().nth(col - 1).unwrap()) {
                        star_coord.push((row - 1, col - 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                    if check_star(prev.chars().nth(col + 1).unwrap()) {
                        star_coord.push((row - 1, col + 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                }
                if next != "" {
                    if check_star(next.chars().nth(col).unwrap()){
                        star_coord.push((row + 1, col));
                        // println!("{} {}", row, col);
                        connected = true;
                    }
                    if check_star(next.chars().nth(col - 1).unwrap()) {
                        star_coord.push((row + 1, col - 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }   
                    if check_star(next.chars().nth(col + 1).unwrap()) {
                        star_coord.push((row + 1, col + 1));
                        // println!("{} {}", row, col);
                        connected = true;
                    }   
                }
                if check_star(line.chars().nth(col - 1).unwrap()){
                    star_coord.push((row, col - 1));
                    // println!("{} {}", row, col);
                    connected = true;
                }
                if check_star(line.chars().nth(col + 1).unwrap()){
                    star_coord.push((row, col + 1));
                    // println!("{} {}", row, col);
                    connected = true;
                }
            }
        }
    }

    println!("{:?}", gear_numbers);
    for (_, numbers) in gear_numbers.into_iter() {
        if numbers.len() == 2 {
            let mut prod = 1;
            for v in numbers {
                prod *= v;
                print!("{v} ");
            }
            println!("");
            sol2 += prod; 
        }
    }


    (Solution::from(sol1), Solution::from(sol2))
}

fn check_if_symbol(c: char) -> bool{
    // c == '+' || c == '#' || c == '$' || c == '*' || c == '%' || c == '@' || c == '&' || c == '=' || c == '/'
    !(c.is_digit(10) || c == '.')
}

fn check_star(c: char) -> bool{
    // c == '+' || c == '#' || c == '$' || c == '*' || c == '%' || c == '@' || c == '&' || c == '=' || c == '/'
    c == '*'
}

