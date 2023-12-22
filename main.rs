use std::fs::read_to_string;
use std::collections::BTreeSet;
use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Clone,Copy,Debug)]
struct Coord{
    x: usize,
    y: usize,
    z: usize
}

#[derive(Clone,Copy,Debug)]
struct Brick(Coord,Coord);

fn main(){
    let mut total = 0;
    let text = read_input();
    let reg = Regex::new(r"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();
    
    let mut all_b = vec![];
    for line in text.iter() {
        let cap = reg.captures(line).unwrap();
        let c1 = Coord{
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            z: cap[3].parse().unwrap(),
        };
        let c2 = Coord{
            x: cap[4].parse().unwrap(),
            y: cap[5].parse().unwrap(),
            z: cap[6].parse().unwrap(),
        };
        if c2.z<c1.z {
            all_b.push(Brick(c2,c1));
        } else {
            all_b.push(Brick(c1,c2));
        }
    }
    
    all_b.sort_by(|a,b| a.0.z.cmp(&b.0.z));
    let mut up_b = vec![BTreeSet::new();all_b.len()];
    let mut down_b = vec![BTreeSet::new();all_b.len()];
    
    
    let xmax = all_b.iter().map(|b| if b.0.x < b.1.x {b.1.x} else {b.0.x}).max().unwrap();
    let ymax = all_b.iter().map(|b| if b.0.y < b.1.y {b.1.y} else {b.0.y}).max().unwrap();
    println!("{} {}",xmax, ymax);
    
    let mut gheight: Vec<Vec<usize>> = vec![vec![0;ymax+1];xmax+1];
    let mut gbrick: Vec<Vec<Option<usize>>> = vec![vec![None;ymax+1];xmax+1];
    
    for b in 0..all_b.len() {
        let Brick(u,v) = all_b[b];
        let rangex = if u.x<v.x {u.x..v.x+1} else {v.x..u.x+1};
        let rangey = if u.y<v.y {u.y..v.y+1} else {v.y..u.y+1};
        
        let mut zmax = 0;
        for i in rangex.clone() {
            for j in rangey.clone() {
                if zmax < gheight[i][j] {
                    zmax = gheight[i][j];
                }
            }
        }
        
        let newz = zmax + v.z-u.z+1;
        for i in rangex.clone() {
            for j in rangey.clone() {
                if gheight[i][j] == zmax {
                    if let Some(b2) = gbrick[i][j] {
                        up_b[b2].insert(b);
                        down_b[b].insert(b2);
                    }
                }
                gheight[i][j] = newz;
                gbrick[i][j] = Some(b);
            }
        }
    }
    
    for b in 0..all_b.len() {
        if up_b[b].len() == 0 {
            total += 1;
        } else if up_b[b].iter().all(|b| down_b[*b].len() > 1) {
            total += 1;
        }
    }
    
    let mut total2 = 0;
    for b in 0..all_b.len() {
        let mut state = vec![false;all_b.len()];
        state[b] = true;
        for b in 0..all_b.len() {
            state[b] = state[b] || down_b[b].len()>0 && down_b[b].iter().all(|b| state[*b]);
        }
        //println!("{:?}", state);
        total2 += state.into_iter().filter(|s| *s).count() -1;
    }
    
    println!("{}", total);
    println!("{}", total2)
}
