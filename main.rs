use std::fs::read_to_string;
use std::collections::VecDeque;
//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Case {
    Wall,
    Blank,
    Step(u32)
}
use Case::{Wall,Blank,Step};

fn next_steps(g: &Vec<Vec<Case>>, i: usize, j: usize) -> Vec<(usize,usize)> {
    let mut rep = vec!();
    
    if 0 < i {
        rep.push((i-1,j))
    }
    if 0 < j {
        rep.push((i,j-1))
    }
    if i+1 < g.len() {
        rep.push((i+1,j))
    }
    if j+1 < g[0].len() {
        rep.push((i,j+1))
    }
    rep
}

const NBSTEP:u32 = 64;

fn main(){
    let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"^([%&]?)(\w+) -> (.+)$").unwrap();
    
    let mut grid: Vec<Vec<Case>> = text.iter().map(|s| s.chars().map(|c| match c {
        '.' => Blank,
        '#' => Wall,
        'S' => Step(0),
        _ => panic!()
    }).collect()).collect();
    
    
    let mut work = VecDeque::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Step(0) {
                work.push_back((i,j))
            }
        }
    }
    
    while let Some((i,j)) = work.pop_front() {
        let Step(d) = grid[i][j] else {
            panic!("Step from unreached case")
        };
        if d < NBSTEP {
            for (i,j) in next_steps(&grid,i,j).into_iter() {
                if grid[i][j] == Blank {
                    grid[i][j] = Step(d+1);
                    work.push_back((i,j));
                }
            }
        }
    }
    
    let parity = NBSTEP%2;
    for l in grid.into_iter() {
        for c in l.into_iter() {
            if let Step(d) = c {
                if d%2 == parity {
                    total+=1;
                }
            }
        }
    }
    
    
    println!("{}", total)
}