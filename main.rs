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
    let text = read_input();
    let reg = Regex::new(r"-?\d+").unwrap();
    
    for line in text.iter() {
        let mut suite : Vec<i64> = reg.find_iter(line).map(|m| m.as_str().parse().unwrap()).collect();
        let mut new = suite[0];
        let mut s = 1;
        while !suite.iter().all(|x| *x==0) {
            suite = (0..suite.len()-1).map(|i| suite[i+1]-suite[i]).collect();
            new = suite[0]-new;
            s *= -1;
        }
        total += s*new;
    }
    println!("{:?}", total)
}