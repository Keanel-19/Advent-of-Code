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



#[derive(Clone,Copy,Debug)]
struct GearRange {
    x: (u32,u32),
    m: (u32,u32),
    a: (u32,u32),
    s: (u32,u32)
}

impl GearRange {
    fn new(t:(u32,u32)) -> GearRange {
        GearRange {
            x:t,
            m:t,
            a:t,
            s:t
        }
    }

    fn count(&self) -> u64 {
        fn len(t:(u32,u32)) -> u64 {
            (t.1-t.0+1) as u64
        }
        len(self.x)*len(self.m)*len(self.a)*len(self.s)
    }
    
    fn split_x(&self,l:u32) -> (GearRange,GearRange) {
        let mut inf = self.clone();
        let mut sup = self.clone();
        inf.x.1 = l;
        sup.x.0 = l+1;
        (inf,sup)
    }
    
    fn split_m(&self,l:u32) -> (GearRange,GearRange) {
        let mut inf = self.clone();
        let mut sup = self.clone();
        inf.m.1 = l;
        sup.m.0 = l+1;
        (inf,sup)
    }
    
    fn split_a(&self,l:u32) -> (GearRange,GearRange) {
        let mut inf = self.clone();
        let mut sup = self.clone();
        inf.a.1 = l;
        sup.a.0 = l+1;
        (inf,sup)
    }
    
    fn split_s(&self,l:u32) -> (GearRange,GearRange) {
        let mut inf = self.clone();
        let mut sup = self.clone();
        inf.s.1 = l;
        sup.s.0 = l+1;
        (inf,sup)
    }
}


#[derive(Clone,Debug)]
enum Rule {
    Sup(String,u32),
    Inf(String,u32),
    Always
}
use Rule::{Sup,Inf,Always};

struct RuleRes<T> {
    accepted: Option<T>,
    rejected: Option<T>
}


impl<T> RuleRes<T> {
    fn new(a:T,r:T) -> RuleRes<T>{
        RuleRes {
            accepted: Some(a),
            rejected: Some(r)
        }
    }

    fn a(v:T) -> RuleRes<T> {
        RuleRes {
            accepted: Some(v),
            rejected: None
        }
    }
    
    fn r(v:T) -> RuleRes<T> {
        RuleRes {
            accepted: None,
            rejected: Some(v)
        }
    }
}


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
    
    
    fn test_range(&self,g:GearRange) -> RuleRes<GearRange> {
        match self {
            Sup(s,v) => match s.as_str() {
                "x" => if g.x.0>*v {
                    RuleRes::a(g)
                } else if g.x.1 >*v {
                    let (r,a) = g.split_x(*v);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                "m" => if g.m.0>*v {
                    RuleRes::a(g)
                } else if g.m.1 >*v {
                    let (r,a) = g.split_m(*v);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                "a" => if g.a.0>*v {
                    RuleRes::a(g)
                } else if g.a.1 >*v {
                    let (r,a) = g.split_a(*v);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                "s" => if g.s.0>*v {
                    RuleRes::a(g)
                } else if g.s.1 >*v {
                    let (r,a) = g.split_s(*v);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                _ => panic!()
            },
            Inf(s,v) => match s.as_str() {
                "x" => if g.x.1<*v {
                    RuleRes::a(g)
                } else if g.x.0 <*v {
                    let (a,r) = g.split_x(*v-1);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                "m" => if g.m.1<*v {
                    RuleRes::a(g)
                } else if g.m.0 <*v {
                    let (a,r) = g.split_m(*v-1);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                "a" => if g.a.1<*v {
                    RuleRes::a(g)
                } else if g.a.0 <*v {
                    let (a,r) = g.split_a(*v-1);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                "s" => if g.s.1<*v {
                    RuleRes::a(g)
                } else if g.s.0 <*v {
                    let (a,r) = g.split_s(*v-1);
                    RuleRes::new(a,r)
                } else {
                    RuleRes::r(g)
                },
                _ => panic!()
            },
            Always => RuleRes::a(g)
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
    //let mut gears = vec![];
    
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
        }/*
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
        }*/
    }
    
    /*
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
    }*/
    
    
    let mut list = vec![(GearRange::new((1,4000)),"in".to_string())];
    let mut acc = vec![];
    while let Some((g,n)) = list.pop() {
        match n.as_str() {
            "A" => acc.push(g),
            "R" => continue,
            _ => {
            let mut g = g;
            for (r,s) in rules[&n].iter() {
                let res = r.test_range(g);
                if let Some(a) = res.accepted {
                    list.push((a,s.clone()))
                }
                if let Some(r) = res.rejected {
                    g = r
                } else {
                    break
                }
            }}
        }
    }
    
    
    for g in acc.iter() {
        //println!("{} {:?}",g.count(),g);
        total+= g.count();
    }
    
    println!("{}", total)
}