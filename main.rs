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

/*
fn next_steps_wrap(g: &Vec<Vec<Case>>, i: usize, j: usize) -> Vec<(usize,usize)> {
    let mut rep = vec!();
    
    if 0 < i {
        rep.push((i-1,j))
    } else {
        rep.push((g.len()-1,j))
    }
    if 0 < j {
        rep.push((i,j-1))
    } else {
        rep.push((i,g[0].len()-1))
    }
    if i+1 < g.len() {
        rep.push((i+1,j))
    } else {
        rep.push((0,j))
    }
    if j+1 < g[0].len() {
        rep.push((i,j+1))
    } else {
        rep.push((i,0))
    }
    rep
}*/

const NBSTEP:u32 = 26501365;

fn main(){
    let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"^([%&]?)(\w+) -> (.+)$").unwrap();
    
    let init_grid: Vec<Vec<Case>> = text.iter().map(|s| s.chars().map(|c| match c {
        '.' => Blank,
        '#' => Wall,
        'S' => Step(0),
        _ => panic!()
    }).collect()).collect();
    let ilen = init_grid.len();
    let jlen = init_grid[0].len();
    
    let mul = 5;
    let mut grid = vec![vec![Blank;jlen*mul];ilen*mul];
    
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let u = i%ilen;
            let v = j%jlen;
            if init_grid[u][v] == Wall {
                grid[i][j]=Wall
            }
        }
    }
    
    let mut work = VecDeque::new();
    for i in 0..init_grid.len() {
        for j in 0..init_grid[i].len() {
            if init_grid[i][j] == Step(0) {
                let i = init_grid.len()*(mul/2)+i;
                let j = init_grid[0].len()*(mul/2)+j;
                grid[i][j] = Step(0);
                work.push_back((i,j));
            }
        }
    }
    
    let k = NBSTEP/ilen as u32;
    let r = NBSTEP%ilen as u32;
    
    let max_step = 1+r+2*ilen as u32;
    while let Some((i,j)) = work.pop_front() {
        let Step(d) = grid[i][j] else {
            panic!("Step from unreached case")
        };
        if d < max_step {
            for (i,j) in next_steps(&grid,i,j).into_iter() {
                if grid[i][j] == Blank {
                    grid[i][j] = Step(d+1);
                    work.push_back((i,j));
                }
            }
        }
    }
    
    let mut r2ntot = 0;
    let mut rntot = 0;
    let mut rtot = 0;
    
    let n = ilen as u32;
    let parity = NBSTEP%2;
    for l in grid.into_iter() {
        for c in l.into_iter() {
            if let Step(d) = c {
                if d%2 == parity {
                    total+=1;
                }
                    if d%2==(r+2*n)%2 && d <= r+2*n {
                        r2ntot+=1;
                    }
                    if d%2==(r+n)%2 && d <= r+n {
                        rntot+=1;
                    }
                    if d%2==(r)%2 && d <= r {
                        rtot+=1;
                    }
            }
        }
    }
    
    let r = r as u128;
    let k = k as u128;
    let n = ilen as u128;
    let b0 = rtot;
    let b1 = rntot - rtot;
    let b2 = r2ntot-rntot;
    
    println!("k {}",k);
    println!("{} {} {}",r2ntot,rntot,rtot);
    println!("{}", b0 + b1*k + (k*(k-1)/2)*(b2-b1));
    println!("{}", total)
}