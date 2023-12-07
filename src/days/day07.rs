use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use core::cmp::Ordering;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;

    let input = read_to_string("./input/day07.txt").expect("couldnt read file");
    let mut hand_bids = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        hand_bids.push((hand, bid.parse::<u64>().unwrap()));
    }
    hand_bids.sort_by(|(hand1, _), (hand2, _)| hand_compare(hand1, hand2));
    for (i, (_, bids)) in hand_bids.iter().enumerate() {
        sol1 += *bids * ((i + 1) as u64);
    }

    let mut sol2: u64 = 0;
    println!("{:?}", hand_bids);
    hand_bids.sort_by(|(hand1, _), (hand2, _)| hand_compare_2(hand1, hand2));
    println!("{:?}", hand_bids);
    for (i, (_, bids)) in hand_bids.iter().enumerate() {
        sol2 += *bids * ((i + 1) as u64);
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn hand_compare(hand1: &str, hand2: &str) -> Ordering {
    let order_of_cards = "AKQJT98765432".to_string();
    let cards: [usize; 12] = [0; 12];
    let type1 = type_hand(hand1);
    let type2 = type_hand(hand2);
    if type1 > type2 {
        Ordering::Greater
    } else if type1 < type2 {
        Ordering::Less
    } else {
        for (c1, c2) in hand1.chars().zip(hand2.chars()){
            if order_of_cards.find(c1).unwrap() < order_of_cards.find(c2).unwrap() {
                return Ordering::Greater;
            } else if order_of_cards.find(c1).unwrap() > order_of_cards.find(c2).unwrap() {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

fn type_hand(hand: &str) -> u64 {
    let order_of_cards = "AKQJT98765432".to_string();
    let mut cards: [usize; 13] = [0; 13];
    for c in hand.chars() {
        cards[order_of_cards.find(c).unwrap()] += 1;
    }
    if cards.contains(&5) {
        6
    } else if cards.contains(&4) {
        5
    } else if cards.contains(&3) & cards.contains(&2) {
        4
    } else if cards.contains(&3) {
        3
    } else if cards.contains(&2) && cards.iter().position(|&x| x == 2).unwrap() != cards.iter().rposition(|&x| x == 2).unwrap() { // 2 pairs
        2
    } else if cards.contains(&2) { // 1 pair
        1
    } else { // nothing
        0
    }
}

fn hand_compare_2(hand1: &str, hand2: &str) -> Ordering {
    let order_of_cards = "AKQT98765432J".to_string();
    let cards: [usize; 13] = [0; 13];
    let type1 = type_hand_2(hand1);
    // println!("{type1}");
    let type2 = type_hand_2(hand2);
    // println!("{type2}");
    if type1 > type2 {
        Ordering::Greater
    } else if type1 < type2 {
        Ordering::Less
    } else {
        for (c1, c2) in hand1.chars().zip(hand2.chars()){
            if order_of_cards.find(c1).unwrap() < order_of_cards.find(c2).unwrap() {
                return Ordering::Greater;
            } else if order_of_cards.find(c1).unwrap() > order_of_cards.find(c2).unwrap() {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

fn type_hand_2(hand: &str) -> u64 {
    let order_of_cards = "AKQT98765432J".to_string();
    let mut cards: [usize; 12] = [0; 12];
    let mut j = 0;
    for c in hand.chars() {
        if c == 'J' {
            j += 1;
            // cards = cards.map(|x| x + 1);
            continue;
        }
        cards[order_of_cards.find(c).unwrap()] += 1;
    }
    let mut max = 0;
    let mut index_max = 0;
    for (i, x) in cards.iter().enumerate() {
        if x > &max {
            index_max = i;
            max = *x;
        }
    }
    cards[index_max] += j;
    // println!("{:?}", cards);
    // println!("{}, {}", hand, j);

    if cards.contains(&5) {
        6
    } else if cards.contains(&4) {
        5
    } else if cards.contains(&3) & cards.contains(&2) {
        4
    } else if cards.contains(&3) {
        3
    } else if cards.contains(&2) && cards.iter().position(|&x| x == 2).unwrap() != cards.iter().rposition(|&x| x == 2).unwrap() { // 2 pairs
        2
    } else if cards.contains(&2) { // 1 pair
        1
    } else { // nothing
        0
    }
}