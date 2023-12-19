use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}


#[derive(Clone,Copy,Debug)]
struct Gear {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl Gear {
    fn sum(&self) -> u32 {
        self.x+self.m+self.a+self.s
    }
}

#[derive(Clone,Debug)]
enum Rule {
    Sup(String,u32),
    Inf(String,u32),
    Always
}
use Rule::{Sup,Inf,Always};


impl Rule {
    fn test(&self,g:Gear) -> bool {
        match self {
            Sup(s,v) => match s.as_str() {
                "x" => g.x>*v,
                "m" => g.m>*v,
                "a" => g.a>*v,
                "s" => g.s>*v,
                _ => panic!()
            },
            Inf(s,v) => match s.as_str() {
                "x" => g.x<*v,
                "m" => g.m<*v,
                "a" => g.a<*v,
                "s" => g.s<*v,
                _ => panic!()
            },
            Always => true
        }
    }
}


fn main(){
    let mut total = 0;
    let text = read_input();
    let reg_rule = Regex::new(r"^(\w+)\{(.+)\}$").unwrap();
    let reg_act = Regex::new(r"(?:(\w+)([<>])(\d+):)?(\w+)").unwrap();
    
    let reg_gear = Regex::new(r"^\{(.+)\}$").unwrap();
    let reg_att = Regex::new(r"([xmas])=(\d+)").unwrap();
    
    let mut rules = HashMap::new();
    let mut gears = vec![];
    
    for l in text.iter() {
        if let Some(cap) = reg_rule.captures(l) {
            let name = cap[1].to_string();
            rules.insert(cap[1].to_string(),vec![]);
            for c in reg_act.captures_iter(&cap[2]) {
                let r = match c.get(2).and_then(|m| Some(m.as_str())) {
                    Some("<") => Inf(c[1].to_string(),c[3].parse::<u32>().unwrap()),
                    Some(">") => Sup(c[1].to_string(),c[3].parse::<u32>().unwrap()),
                    _ => Always
                };
                rules.get_mut(&name).unwrap().push((r,c[4].to_string()));
            }
        }
        else if let Some(cap) = reg_gear.captures(l) {
            let mut g = Gear{x:0,m:0,a:0,s:0};
            for c in reg_att.captures_iter(&cap[1]) {
                let v = c[2].parse::<u32>().unwrap();
                match &c[1] {
                    "x" => g.x = v,
                    "m" => g.m = v,
                    "a" => g.a = v,
                    "s" => g.s = v,
                    _ => panic!()
                }
            }
            gears.push(g);
        }
    }
    
    
    for g in gears.into_iter() {
        let mut name = "in".to_string();
        'l:loop {
        for (r,s) in rules[&name].iter() {
            if r.test(g) {
                match s.as_str() {
                    "A" => {
                        total+=g.sum();
                        break 'l
                    },
                    "R" => break 'l,
                    _ => name = s.clone()
                }
                break
            }
        }}
    }
    println!("{}", total)
}