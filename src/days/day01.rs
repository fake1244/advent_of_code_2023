use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::env;
use std::char;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let input1 = read_to_string("./input/day01-1.txt").expect("couldnt read file");
    for line in input1.lines(){
        let number1: u64 = line.to_owned().chars().find(|&x| x.is_numeric()).unwrap().to_digit(10).unwrap().into();
        let number2: u64 = line.to_owned().chars().rfind(|&x| x.is_numeric()).unwrap().to_digit(10).unwrap().into();
        sol1 += number1 * 10 + number2;
    }

    let mut sol2: u64 = 0;
    let input1 = read_to_string("./input/day01-2.txt").expect("couldnt read file");
    for line in input1.lines(){
        let mut number1 = "";
        let mut index1 = usize::MAX;
        let mut number2 = "";
        let mut index2 = 0;
        for pattern in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]{
            let index = line.to_owned().find(pattern).unwrap_or(usize::MAX);
            if index < index1 {
                index1 = index;
                number1 = pattern;
            }
            let index = line.to_owned().rfind(pattern).unwrap_or(usize::MAX);
            if index >= index2 && index < usize::MAX {
                index2 = index;
                number2 = pattern;
            }
        }
        
        let num1;
        match number1 {
            "0" => num1 = 0,
            "1" => num1 = 1,
            "2" => num1 = 2,
            "3" => num1 = 3,
            "4" => num1 = 4, 
            "5" => num1 = 5, 
            "6" => num1 = 6, 
            "7" => num1 = 7, 
            "8" => num1 = 8, 
            "9" => num1 = 9, 
            "zeor" => num1 = 0,
            "one" => num1 = 1, 
            "two" => num1 = 2, 
            "three" => num1 = 3, 
            "four" => num1 = 4, 
            "five" => num1 = 5, 
            "six" => num1 = 6, 
            "seven" => num1 = 7, 
            "eight" => num1 = 8, 
            "nine" => num1 = 9,
            _ => num1 = 0
        }

        let num2;
        match number2 {
            "0" => num2 = 0,
            "1" => num2 = 1,
            "2" => num2 = 2,
            "3" => num2 = 3,
            "4" => num2 = 4, 
            "5" => num2 = 5, 
            "6" => num2 = 6, 
            "7" => num2 = 7, 
            "8" => num2 = 8, 
            "9" => num2 = 9, 
            "zero" => num2 = 0,
            "one" => num2 = 1, 
            "two" => num2 = 2, 
            "three" => num2 = 3, 
            "four" => num2 = 4, 
            "five" => num2 = 5, 
            "six" => num2 = 6, 
            "seven" => num2 = 7, 
            "eight" => num2 = 8, 
            "nine" => num2 = 9,
            _ => num2 = 0
        }

        sol2 += num1 * 10 + num2;
    }

    (Solution::from(sol1), Solution::from(sol2))
}
