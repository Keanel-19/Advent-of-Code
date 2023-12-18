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
    let mut total:i128 = 0;
    let text = read_input();
    let reg = Regex::new(r"#([0-9a-f]{5})(\d)").unwrap();
    let reg2 = Regex::new(r"(\w) (\d+) .+").unwrap();
    
    let mut points = vec![(0,0)];
    
    for line in text.iter() {
        let cap = reg.captures(line).unwrap();
        
        let dir = (match &cap[2] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!()
        }).to_string();
        let len: i128 = i128::from_str_radix(&cap[1],16).unwrap();
        /*
        let dir = cap[1].to_string();
        let len: i128 = i128::from_str_radix(&cap[2],10).unwrap();
        
        println!("{} {}",dir,len);
        */
        
        total += len;
        let p = points[points.len()-1];
        points.push((match dir.as_str() {
            "U" => p.0 - len,
            "D" => p.0 + len,
            _ => p.0
        },match dir.as_str() {
            "R" => p.1 + len,
            "L" => p.1 - len,
            _ => p.1
        }));
    }
    //println!("{:?}",points);
    points.pop();
    
    for i in 0..points.len() {
        let j = (i+1)%points.len();
        total -= points[i].0*points[j].1 - points[j].0*points[i].1;
    }
    
    println!("{}", total/2 +1)
}