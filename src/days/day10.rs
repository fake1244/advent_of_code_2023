use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashSet;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;

    let input = read_to_string("./input/day10.txt").expect("couldnt read file");
    // let input = read_to_string("./input/test.txt").expect("couldnt read file");

    let mut grid: Vec<Vec<Point>> = Vec::new();
    let mut start = None;
    for (y, line) in input.lines().enumerate(){
        let mut l = Vec::new();
        for (x, c) in line.chars().enumerate(){
            let point = Point{
                symbol: c,
                x: x,
                y: y,
            };
            if c == 'S' { start = Some(point); }
            l.push(point);
        }
        // println!("{:?}", l);
        grid.push(l);
    }
    let start = start.unwrap();
    // println!("{:?}", start);
    // println!("{:?}", grid);
    let mut fronteer = vec![start];
    let mut visited = HashSet::new();
    while let Some(node) = fronteer.pop() {
        if visited.contains(&node) {
            continue;
        }
        // println!("visit node: {:?}", node);
        visited.insert(node);
        for suc in node.successors(&grid){
            // println!("suc: {:?}", suc);
            if suc.successors(&grid).contains(&node){
                // println!("push suc: {:?}", suc);
                fronteer.push(suc);
            }
        }
    }
    // println!("at the end: {:?}", visited);
    sol1 = (visited.len() / 2).try_into().unwrap();
    
    let mut sol2: u64 = 0;
    let mut in_loop = false;
    let mut on_line = false;
    let mut line = vec![];
    for row in grid {
        for point in row {
            // println!("{:?} in_loop: {}", point, in_loop);
            if visited.contains(&point) {
                let mut symbol = point.symbol;
                if symbol == 'S' { symbol = '|';} // in my input S behaves like |
                if symbol == '|'{
                    in_loop = !in_loop;
                    continue;
                }
                if symbol == '-'{
                    continue;
                }
                
                // if in_loop {
                    line.push(symbol);
                    // check diff types of lines
                    if line.len() == 2 {
                        // F---7 and L--J dont change the face
                        if (line[0] == 'F' && line[1] == 'J') || (line[0] == 'L' && line[1] == '7') {
                            in_loop = !in_loop;
                        }
                        // println!("off line {:?} {}", line, in_loop);
                        line = vec![];
                    }
                    // on_line = !on_line;
                    // in_loop = false;
                    // if on_line {
                    //     in_loop = true;
                    // }
                // }
            } else {
                if in_loop {
                    // println!("count {:?}", point);
                    sol2 += 1;
                }
            }
        }  
    }


    (Solution::from(sol1), Solution::from(sol2))
}
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Point {
    symbol: char,
    x: usize,
    y: usize,
}

impl Point {
    fn successors(&self, grid: &Vec<Vec<Point>>) -> Vec<Point> {
        match self.symbol {
            'S' => {
                let mut res = Vec::new();
                if let Some(point) = grid.get(self.y - 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y + 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x - 1)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x + 1)){
                    res.push(*point);
                }
                res
            },
            '|' => {
                let mut res = Vec::new();
                if let Some(point) = grid.get(self.y - 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y + 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }
                res
            },
            '-' => {
                let mut res = Vec::new();
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x + 1)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x - 1)){
                    res.push(*point);
                }
                res
            },
            'L' => {
                let mut res = Vec::new();
                if let Some(point) = grid.get(self.y - 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }                
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x + 1)){
                    res.push(*point);
                }
                res
            },
            'J' => {
                let mut res = Vec::new();
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x - 1)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y - 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }
                res
            },
            '7' => {
                let mut res = Vec::new();
                if let Some(point) = grid.get(self.y + 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x - 1)){
                    res.push(*point);
                }
                res
            },
            'F' => {
                let mut res = Vec::new();
                if let Some(point) = grid.get(self.y + 1).map_or(None, |row| row.get(self.x)){
                    res.push(*point);
                }
                if let Some(point) = grid.get(self.y).map_or(None, |row| row.get(self.x + 1)){
                    res.push(*point);
                }
                res
            },

            _ => Vec::new()
        }
    }
}
