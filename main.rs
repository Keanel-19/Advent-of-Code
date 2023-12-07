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
let mut total = 0;
    let reg = Regex::new(r"Game (\d+):").unwrap();
    let reg2 = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    
    for line in read_input().iter() {
        let caps = reg.captures(line).unwrap();
        let game: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        
        if reg2.captures_iter(line)
        .all(|c| match (c.get(2).unwrap().as_str(), c.get(1).unwrap().as_str().parse::<u32>().unwrap()) {
            ("red",v) => v<=12,
            ("green",v) => v<=13,
            ("blue",v) => v<=14,
            _ => true
        }) {
            total += game
        }
    }
    println!("{}",total)
}