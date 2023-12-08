use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;
use gcd::Gcd;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn main(){
    //let mut total = 0;
    let text = read_input();
    let reg = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();
    
    let mut graph = HashMap::new();
    
    let mut nodes = Vec::new();
    
    for line in text.iter() {
        if let Some(cap) = reg.captures(line) {
            graph.insert(cap[1].to_string(),(cap[2].to_string(),cap[3].to_string()));
            if cap[1].chars().last() == Some('A') {
                nodes.push(cap[1].to_string());
            }
        };
    }
    
    println!("len graph : {}",graph.len());
    
    
    let orders = &text[0];
    let mut total = vec![0;nodes.len()];
    println!("{:?}",nodes);
    for j in 0..nodes.len() {
        let mut i = 0;
        loop {
            match orders.chars().nth(i) {
                Some('R') => nodes[j] = graph[&nodes[j]].1.to_string(),
                Some('L') => nodes[j] = graph[&nodes[j]].0.to_string(),
                _ => panic!()
            }
        
            i += 1;
            total[j] += 1;
            if i >= orders.len() {
                i = 0;
            }
            if nodes[j].chars().last() == Some('Z') {
                break
            }
        }
        println!("{:?}", total)
    }
    let mut ppcm : u64 = 1;
    for n in total.iter() {
        ppcm = ppcm*n / ppcm.gcd(*n)
    }
    println!("{:?}", ppcm)
}