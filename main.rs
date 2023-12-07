use std::fs::read_to_string;
use regex::Regex;


fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn main(){
let mut total =0;
    let reg = Regex::new(r"\d+").unwrap();
    let reg2 = Regex::new(r"[^.\d]").unwrap();
    
    let plan = read_input();
    let mut syms = vec![vec![false; plan[0].len()]; plan.len()];
    
    for i in 0..plan.len() {
        for j in reg2.find_iter(&plan[i]).map(|m| m.start()) {
            syms[i][j] = true
        }
    }
    
    for i in 0..plan.len() {
        for m in reg.find_iter(&plan[i]) {
            let mut n : u32 = m.as_str().parse().unwrap();
            for k in if i==0 {0} else {i-1}..=if i+1>=syms.len() {i} else {i+1} {
                for l in if m.start()==0 {0} else {m.start()-1}..if m.end()+1>=syms.len() {m.end()} else {m.end()+1}{
                    if syms[k][l] {
                        total += n;
                        n = 0;
                    }
                }
            }
        }
    }
    
    println!("{}", total)
    
}