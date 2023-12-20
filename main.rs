use std::fs::read_to_string;
use std::collections::{BTreeMap,VecDeque};
use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Signal {
    Low,
    High
}
use Signal::{Low,High};


#[derive(Clone,Debug)]
enum Module {
    Broadcast,
    FlipFlop(bool),
    Conj(BTreeMap<String,Signal>),
    Blank
}
use Module::{Broadcast,FlipFlop,Conj,Blank};


impl Module {
    fn add_prec(&mut self, s: String) {
        let Conj(h) = self else {return};
        h.insert(s,Low);
    }
    
    fn receive(&mut self, s: Signal, sender: String) -> Option<Signal> {
        match self {
            Blank => None,
            Broadcast => Some(s),
            FlipFlop(state) => match s {
                High => None,
                Low => {
                    *state = !*state;
                    if *state {
                        Some(High)
                    } else {
                        Some(Low)
                    }
                }
            },
            Conj(state) => {
                state.insert(sender,s);
                if state.iter().all(|(_,s)| *s==High) {
                    Some(Low)
                } else {
                    Some(High)
                }
            }
        }
    }
}


fn main(){
    //let mut total = 0;
    let text = read_input();
    let reg = Regex::new(r"^([%&]?)(\w+) -> (.+)$").unwrap();
    
    let mut mods = BTreeMap::new();
    let mut temp = vec![];
    for line in text.iter() {
        let cap = reg.captures(line).unwrap();
        let name = &cap[2];
        temp.push((name.to_string(),cap[3].to_string()));
        mods.insert(name.to_string(), match name {
            "broadcaster" => Broadcast,
            _ => match &cap[1] {
                "%" => FlipFlop(false),
                "&" => Conj(BTreeMap::new()),
                _ => panic!()
            }
        });
    }
    
    let mut links = BTreeMap::new();
    for (name,suivs) in temp.into_iter() {
        let mut l = vec![];
        for n in suivs.split(", ") {
            if !mods.contains_key(n) {
                mods.insert(n.to_string(),Blank);
            }
            l.push(n.to_string());
            mods.get_mut(n).unwrap().add_prec(name.clone());
        }
        links.insert(name,l);
    }
    
    
    let links = links;
    let mut htot = 0;
    let mut ltot = 0;
    
    let beacon = ["qq","fj","vm","jc"];
    
    for push in 0..10000 {
        let mut process = VecDeque::from([("button".to_string(),Low,"broadcaster".to_string())]);
        let mut lcount= 0;
        let mut hcount =0;
        
        while let Some((em,sig,rec)) = process.pop_front() {
            //println!("{} -{:?}-> {}", em,sig,rec);
            match sig {
                Low => lcount+=1,
                High => hcount += 1
            }
            if beacon.contains(&&em.as_str()) && sig==Low {
                println!("{} {}",em,push+1)
            }
            let Some(sig) = mods.get_mut(&rec).unwrap().receive(sig,em) else {continue};
            let em = rec;
            for rec in links[&em].iter() {
                process.push_back((em.clone(),sig,rec.clone()))
            }
        }
        ltot+=lcount;
        htot+=hcount;
    }
    
    
    println!("{}", htot*ltot)
}