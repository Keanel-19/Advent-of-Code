use std::fs::read_to_string;
use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Debug)]
struct Hand {
    bid: u32,
    t: u32,
    cards: [usize;5]
}

fn main(){
    //let mut total = 1;
    let cards = "J23456789TQKA";
    let text = read_input();
    let reg = Regex::new(r"(.+) (\d+)").unwrap();
    
    let mut hands:Vec<Hand> = Vec::new();
    for line in text.iter() {
        let cap = reg.captures(line).unwrap();
        //println!("{:?} {:?}",&cap[1],&cap[2]);
        
        let mut h = Hand{
            bid: cap[2].parse().unwrap(),
            t: 0,
            cards: cap[1].chars().map(|c| cards.find(c).unwrap()).collect::<Vec<usize>>().as_slice().try_into().unwrap()
        };
        
        let mut count = [0;13];
        for c in h.cards.iter() {
            count[*c] += 1;
        };
        let joker = count[0];
        let mut ncount = [0;12];
        ncount.copy_from_slice(&count[1..]);
        
        ncount.sort();
        
        h.t = match (ncount[11]+joker,ncount[10]) {
            (5,_) => 6,
            (4,_) => 5,
            (3,2) => 4,
            (3,_) => 3,
            (2,2) => 2,
            (2,_) => 1,
            _ => 0
        };
        
        hands.push(h);
        
    }
    //println!("{:?}",hands);
    
    hands.sort_by(|h,g| h.t.cmp(&g.t)
    .then(h.cards[0].cmp(&g.cards[0]))
    .then(h.cards[1].cmp(&g.cards[1]))
    .then(h.cards[2].cmp(&g.cards[2]))
    .then(h.cards[3].cmp(&g.cards[3]))
    .then(h.cards[4].cmp(&g.cards[4]))
    );
    
    let total:u32= hands.iter().enumerate().map(|(i,h)| (i as u32+1)*h.bid).sum();
    println!("{}", total)
    
}