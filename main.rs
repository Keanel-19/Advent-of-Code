use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Orient {
    North,
    South,
    East,
    West
}
use Orient::{North,South,East,West};

impl Orient {
    fn depl(&self, c: (usize,usize)) -> Option<(usize,usize)> {
        match self {
            North => if c.0 > 0 {
                Some((c.0-1,c.1))
            } else {
                None
            },
            South => Some((c.0+1,c.1)),
            West => if c.1 > 0 {
                Some((c.0,c.1-1))
            } else {
                None
            },
            East => Some((c.0,c.1+1))
        }
    }
    
    fn i(&self) -> usize {
        match self {
            North => 0,
            South => 1,
            West => 2,
            East => 3
        }
    }
    
    fn turn_back(&self, other: Self) -> bool {
        *self == North && other == South || *self == South && other == North || *self == East && other == West || *self == West && other == East
    }
}


#[derive(Clone,Copy,Debug,Eq,PartialEq)]
struct State {
    p: (usize,usize),
    o: Orient,
    n: usize
}


#[derive(Clone,Copy,Debug)]
struct HeapWrap {
    state: State,
    score: u32
}


fn weight(s: State, w: u32) -> i64 {
    w as i64 - s.p.0 as i64 - s.p.1 as i64
}


impl PartialEq for HeapWrap {
    fn eq(&self, other: &Self) -> bool {
        weight(self.state,self.score) == weight(other.state,other.score)
    }
}

impl Eq for HeapWrap {}

impl Ord for HeapWrap {
    fn cmp(&self, other: &Self) -> Ordering {
        weight(other.state,other.score).cmp(&weight(self.state,self.score))
    }
}

impl PartialOrd for HeapWrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main(){
    let mut total:u32 = 0;
    let text = read_input();
    //let reg = Regex::new(r"(\w+)(-|=)(\d?)").unwrap();
    
    let grid: Vec<Vec<u32>> = text.iter().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let goal = (grid.len()-1,grid[0].len()-1);
    
    let mut costs: Vec<Vec<[[Option<u32>;11];4]>> = vec![vec![[[None;11];4];grid[0].len()];grid.len()];
    
    let mut heap = BinaryHeap::new();
    
    for i in 0..4 {
        costs[0][0][i][0] = Some(0);
    }
    
    heap.push(HeapWrap{state:State{p:(0,0),o:East,n:0},score:0});
    
    while let Some(HeapWrap {state, score}) = heap.pop() {
        //println!("{:?} : {}",state, score);
        if state.p == goal && state.n >= 4 {
            total = score;
            break
        }
        
        if costs[state.p.0][state.p.1][state.o.i()][state.n].is_some() && score > costs[state.p.0][state.p.1][state.o.i()][state.n].unwrap() {
            continue
        }
        
        for o in [North,South,East,West] {
            if o == state.o && state.n == 10 || o.turn_back(state.o) || o != state.o && state.n < 4 {
                continue
            }
            
            let p = o.depl(state.p);
            if p.is_none() {
                continue
            }
            let p = p.unwrap();
            
            if p.0 < grid.len() && p.1 < grid[0].len() {
                let n = if o == state.o {state.n+1} else {1};
                let new_score = score + grid[p.0][p.1];
                let old_s = costs[p.0][p.1][o.i()][n];
                if old_s.is_none() || new_score < old_s.unwrap() {
                    costs[p.0][p.1][o.i()][n] = Some(new_score);
                    heap.push(HeapWrap{state:State{p,o,n},score:new_score});
                }
            }
        }
    }
    
    println!("{:?}",total)
}