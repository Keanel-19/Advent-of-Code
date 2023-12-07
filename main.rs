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
    let reg = Regex::new(r"^Card +\d+:([ \d]+)\|([ \d]+)$").unwrap();
    let reg2 = Regex::new(r"\d+").unwrap();
    
    let cards = read_input();
    let mut quantity = vec![1; cards.len()];
    
    for i in 0..cards.len() {
        let line = &cards[i];
        let caps = reg.captures(line).unwrap();
        let targets : Vec<u32> = reg2.find_iter(&caps[1]).map(|s| s.as_str().parse().unwrap()).collect();
        let winning_count = reg2.find_iter(&caps[2]).map(|s| s.as_str().parse::<u32>().unwrap()).filter(|n| targets.contains(n)).count();
        for j in 0..winning_count {
            quantity[i+1+j] += quantity[i]
        }
    }
    
    total = quantity.iter().sum();
    println!("{}", total)
    
}