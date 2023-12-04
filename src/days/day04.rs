use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    let input = read_to_string("./input/day04.txt").expect("couldnt read file");

    for line in input.lines(){
        let (game, card_numbers) = line.split_once(": ").unwrap();
        let (win_n, my_n) = card_numbers.split_once(" | ").unwrap();
        let win_numbers = win_n.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();
        let my_numbers = my_n.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();

        // println!("{:?} | {:?}", win_numbers, my_numbers);

        let mut points = 1;
        for number in my_numbers {
            if win_numbers.contains(&number){
                points *= 2;
            }
        }
        points /= 2;
        // println!("{}", points);
        sol1 += points;

    }

    let mut sol2: u64 = 0;
    let mut scratchcards: [u64; 200] = [0; 200];

    println!("{:?}", scratchcards);
    for line in input.lines(){
        let (game, card_numbers) = line.split_once(": ").unwrap();
        let mut iter = game.split_whitespace();
        iter.next();
        let game_num = iter.next().unwrap().parse::<usize>().unwrap();
        let (win_n, my_n) = card_numbers.split_once(" | ").unwrap();
        let win_numbers = win_n.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();
        let my_numbers = my_n.split(" ").filter(|x| x != &"").collect::<Vec<&str>>();

        println!("{} {:?} | {:?}", game_num, win_numbers, my_numbers);
        scratchcards[game_num] += 1;
        let mut matches = 0;
        for number in my_numbers {
            if win_numbers.contains(&number){
                matches += 1;
            }
        }
        println!("{}", matches);

        for i in game_num + 1..game_num + 1 + matches{
            scratchcards[i] += scratchcards[game_num];
            println!("add {} to {}",scratchcards[game_num], i);
        }
    }
    sol2 = scratchcards.iter().sum();

    (Solution::from(sol1), Solution::from(sol2))
}
