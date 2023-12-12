use std::fs::read_to_string;
use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(PartialEq,Clone,Copy,Debug)]
enum State {
    Good,
    Bad
}

use State::{Good,Bad};

#[derive(PartialEq,Clone,Copy,Debug)]
enum Data {
    Fixed(State),
    Unfixed
}
use Data::{Fixed,Unfixed};

fn main(){
    let mut total = 0;
    let text = read_input();
    let reg = Regex::new(r"([.#?]+) ([\d,]+)").unwrap();
    
    for line in text.iter() {
        let cap = reg.captures(line).unwrap();
        
        let init_states: Vec<Data> = cap[1].chars().map(|c| match c {
            '.' => Fixed(Good),
            '#' => Fixed(Bad),
            '?' => Unfixed,
            _ => panic!()
        }).collect();
        //println!("{:?}",init_states);
        
        let groups: Vec<u32> = cap[2].split(',').map(|s| s.parse().unwrap()).chain([0]).collect();
        //println!("{:?}",groups);
        
        let mut total_bad = groups.iter().sum::<u32>() as i32 - init_states.iter().filter(|s| **s == Fixed(Bad)).count() as i32;
        
        let mut total_good = init_states.iter().filter(|s| **s == Unfixed).count() as i32 - total_bad;
        
        //println!("good :{} bad :{}",total_good,total_bad);
        
        let mut states = vec![];
        let mut count = vec![0;groups.len()];
        let mut i = 0;
        let mut gi = 0;
        let mut guess = Bad;
        let mut back = false;
        let mut subtot = 0;
        
        loop {
            //println!("{:?}",states);
            if !back {
                states.push(match init_states[i] {
                    Fixed(x) => x,
                    Unfixed => {
                        if guess == Good {
                            total_good -= 1
                        } else {
                            total_bad -= 1
                        }
                        guess
                    }
                });
                match states[i] {
                    Bad => count[gi]+=1,
                    Good => if count[gi] > 0 {
                        if count[gi]==groups[gi] {
                            gi +=1
                        } else {
                            back = true
                        }
                    }
                }
                
                if total_bad < 0 || total_good < 0 || count[gi]>groups[gi] {
                    back=true
                }
                if !back {
                    i+=1;
                    guess= Bad
                }
            } else {
                let state_tried = states.pop().unwrap();
                back = match (init_states[i],state_tried) {
                    (Fixed(_),_) => true,
                    (Unfixed,Good) => {
                        total_good+=1;
                        true
                    },
                    (Unfixed,Bad) => {
                        total_bad+=1;
                        guess=Good;
                        false
                    }
                };
                match state_tried {
                    Good => (),
                    Bad => {
                        if 0<count[gi] {
                            count[gi]-=1;
                        } else {
                            gi -= 1;
                            count[gi] -=1;
                        }
                    }
                }
                if back {
                    if 0 < i {
                        i-=1
                    } else {
                        break
                    }
                }
            }
            
            if i >= init_states.len() && !back {
                //println!("find : {:?}",states);
                subtot += 1;
                i -= 1;
                back = true;
            }
        }
        //println!("{}",subtot);
        total += subtot;
    }
    
    println!("{}",total)
}