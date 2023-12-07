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
    let reg2 = Regex::new(r"\*").unwrap();
    
    let plan = read_input();
    let mut syms = vec![vec![(-1,0,0); plan[0].len()]; plan.len()];
    
    for i in 0..plan.len() {
        for j in reg2.find_iter(&plan[i]).map(|m| m.start()) {
            syms[i][j].0 = 0
        }
    }
    
    for i in 0..plan.len() {
        for m in reg.find_iter(&plan[i]) {
            let n : u32 = m.as_str().parse().unwrap();
            for k in if i==0 {0} else {i-1}..=if i+1>=syms.len() {i} else {i+1} {
                for l in if m.start()==0 {0} else {m.start()-1}..if m.end()+1>=syms.len() {m.end()} else {m.end()+1}{
                match syms[k][l].0 {
                    -1 => (),
                    0 => {syms[k][l].0 = 1; syms[k][l].1= n},
                    1 => {syms[k][l].0 = 2; syms[k][l].2 = n},
                    _ => syms[k][l].0 = -1
                }
                }
            }
        }
    }
    
    for l in syms.iter() {
        for t in l.iter() {
            if t.0 == 2 {
                total += t.1*t.2
            }
        }
    }
    
    println!("{}", total)
    
}