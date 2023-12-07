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
    let mut total = 1;
    let text = read_input();
    let reg = Regex::new(r"\d+").unwrap();
    
    let times:Vec<f64> = reg.find_iter(&text[0]).map(|m| m.as_str().parse().unwrap()).collect();
    
    let dists:Vec<f64> = reg.find_iter(&text[1]).map(|m| m.as_str().parse().unwrap()).collect();
    println!("{:?}", times);
    println!("{:?}", dists);
    
    for i in 0..times.len() {
        let t = times[i];
        let d = dists[i];
        let delta = t*t -4.*d;
        
        let r1 = t/2.-delta.sqrt()/2.;
        let r2 = t/2.+delta.sqrt()/2.;
        
        let i1 : u32 = (r1+1.).floor() as u32;
        let i2 : u32 = (r2-1.).ceil() as u32;
        
        total *= i2 - i1 + 1;
    }
    
    println!("{}", total)
    
}