use std::fs::read_to_string;
use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(PartialEq,Clone,Copy,Debug)]
enum Spring {
    Good,
    Bad,
    Both
}
use Spring::{Good,Bad,Both};

fn calculate(spr: &Vec<Spring>, pat: &Vec<usize>) -> u64 {
    let mut mem = vec![vec![None;pat.len()];spr.len()];
    
    fn can_pose(spr: &Vec<Spring>, from: usize, l: usize) -> bool {
        for i in from..from+l {
            if !(i < spr.len() && spr[i]!=Good) {
                return false
            }
        }
        if from+l < spr.len() && spr[from+l]==Bad {
            return false
        }
        true
    }
    
    fn walk(mem: &mut Vec<Vec<Option<u64>>> ,spr: &Vec<Spring>, pat: &Vec<usize>, i: usize, j: usize) -> u64 {
        if i < spr.len() && j < pat.len() {
            let m = mem[i][j];
            if m.is_some() {
                return m.unwrap()
            }
            let m = match spr[i] {
                Good => walk(mem,spr,pat,i+1,j),
                Bad => if can_pose(spr,i,pat[j]) {
                    walk(mem,spr,pat,i+pat[j]+1,j+1)
                } else {
                    0
                },
                Both => (if can_pose(spr,i,pat[j]) {
                    walk(mem,spr,pat,i+pat[j]+1,j+1)
                } else {
                    0
                }) + walk(mem,spr,pat,i+1,j)
            };
            mem[i][j] = Some(m);
            m
        } else if j >= pat.len() {
            for a in i..spr.len() {
                if spr[a] == Bad {
                    return 0
                }
            }
            1
        } else {
            0
        }
    }
    
    walk(&mut mem,&spr,&pat,0,0)
}

//format!("{0}#{0}#{0}#{0}#{0}",&cap[1])

fn main(){
    let mut total = 0;
    let text = read_input();
    let reg = Regex::new(r"([.#?]+) ([\d,]+)").unwrap();
    
    for line in text.iter() {
        let cap = reg.captures(line).unwrap();
        
        let init_states: Vec<Spring> = format!("{0}?{0}?{0}?{0}?{0}",&cap[1]).chars().map(|c| match c {
            '.' => Good,
            '#' => Bad,
            '?' => Both,
            _ => panic!()
        }).collect();
        //println!("{:?}",init_states);
        
        let groups: Vec<usize> = format!("{0},{0},{0},{0},{0}",&cap[2]).split(',').map(|s| s.parse().unwrap()).collect();
        //println!("{:?}",groups);
        
        let subtot = calculate(&init_states,&groups);
        total+=subtot;
        //println!("{}",subtot)
    }
    
    println!("{}",total)
}