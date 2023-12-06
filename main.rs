use std::fs::read_to_string;

struct Digit {
    index: usize,
    value: u32
}

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn find_first(line: &String, pat: &str, value: u32, old: Option<Digit>) -> Option<Digit> {
    let index = line.find(pat);
    match (index,old) {
        (Some(i), Some(d)) => if i < d.index {Some(Digit{index:i,value})} else {Some(d)},
        (None, old) => old,
        (Some(i), None) => Some(Digit{index:i,value})
    }
}

fn find_last(line: &String, pat: &str, value: u32, old: Option<Digit>) -> Option<Digit> {
    let index = line.rfind(pat);
    match (index,old) {
        (Some(i), Some(d)) => if i > d.index {Some(Digit{index:i,value})} else {Some(d)},
        (None, old) => old,
        (Some(i), None) => Some(Digit{index:i,value})
    }
}

fn main(){
    let mut total = 0;
    let digits = [
        ("one",1),
        ("1",1),
        ("two",2),
        ("2",2),
        ("three",3),
        ("3",3),
        ("four",4),
        ("4",4),
        ("five",5),
        ("5",5),
        ("six",6),
        ("6",6),
        ("seven",7),
        ("7",7),
        ("eight",8),
        ("8",8),
        ("nine",9),
        ("9",9),
    ];
    
    for line in read_input().iter() {
        let mut first = None;
        let mut last = None;
        for t in digits.iter() {
            first = find_first(line, t.0, t.1, first);
            last = find_last(line, t.0, t.1, last);
        }
        total += first.unwrap().value*10 + last.unwrap().value
    }
    println!("{}", total)
}