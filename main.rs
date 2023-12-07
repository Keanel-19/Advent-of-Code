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
        
        let mut trip = (0,0,0);
        
        for c in reg2.captures_iter(line) {
            match (c.get(2).unwrap().as_str(), c.get(1).unwrap().as_str().parse::<u32>().unwrap()) {
            ("red",v) => if (v > trip.0) {trip.0 = v},
            ("green",v) => if (v > trip.1) {trip.1 = v},
            ("blue",v) => if (v > trip.2) {trip.2 = v},
            _ => ()
        }}
        total += trip.0*trip.1*trip.2
    }
    println!("{}",total)
}