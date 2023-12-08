use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn main(){
    let mut total = 0;
    let text = read_input();
    let reg = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();
    
    let mut graph = HashMap::new();
    
    for line in text.iter() {
        if let Some(cap) = reg.captures(line) {
            graph.insert(cap[1].to_string(),(cap[2].to_string(),cap[3].to_string()));
        };
    }
    
    println!("len graph : {}",graph.len());
    
    let mut node = "AAA".to_string();
    let mut i = 0;
    let orders = &text[0];
    loop {
        match orders.chars().nth(i) {
            Some('R') => node = graph[&node].1.to_string(),
            Some('L') => node = graph[&node].0.to_string(),
            _ => panic!()
        }
        i += 1;
        total += 1;
        if i >= orders.len() {
            i = 0;
        }
        if node == "ZZZ" {
            break
        }
    }
    
    println!("{}", total)
}