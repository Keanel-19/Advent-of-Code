use std::fs::read_to_string;
//use std::collections::{BTreeSet,HashMap};
//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

const MIN: f64 = 200000000000000.;
const MAX: f64 = 400000000000000.;


#[derive(Clone,Copy,Debug)]
struct V2d{
    x: f64,
    y: f64
}


#[derive(Clone,Copy,Debug)]
enum Intersect{
    One(V2d),
    Inf,
    Zero
}
use Intersect::{One,Inf,Zero};

fn intersect((a,u): (V2d,V2d), (b,v): (V2d,V2d)) -> Intersect {
    let num = v.x*(a.y-b.y)-v.y*(a.x-b.x);
    let num2 = u.x*(a.y-b.y)-u.y*(a.x-b.x);
    let den = u.x*v.y-u.y*v.x;
    if den == 0. {
        if num == 0. {
            Inf
        } else {
            Zero
        }
    } else {
        let t = num / den;
        let t2 = num2 / den;
        //println!("t: {} t2: {}",t,t2);
        if t<=0. || t2<=0. {
            Zero
        } else {
            One(V2d{
                x: a.x+t*u.x,
                y: a.y+t*u.y
            })
        }
    }
}


fn main(){
    let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"^(\d+), (\d+), (\d+) @ (\d+), (\d+), (\d+)$").unwrap();
    
    let mut lines = vec![];
    for line in text.into_iter() {
        let mut vectors = line.split(" @ ");
        let mut pos = vectors.next().unwrap().split(", ").map(|s| s.trim().parse::<f64>().unwrap());
        let mut dir = vectors.next().unwrap().split(", ").map(|s| s.trim().parse::<f64>().unwrap());
        
        lines.push((V2d{
            x: pos.next().unwrap(),
            y: pos.next().unwrap()
        }, V2d{
            x: dir.next().unwrap(),
            y: dir.next().unwrap()
        }));
    }
    
    for i in 0..lines.len()-1 {
        for j in i+1..lines.len() {
            //println!("{:?} {:?} {:?}",lines[i],lines[j],intersect(lines[i],lines[j]));
            match intersect(lines[i],lines[j]) {
                Zero => (),
                Inf => total+=1,
                One(p) => if MIN <= p.x && p.x <= MAX && MIN <= p.y && p.y <= MAX {
                    total+=1
                }
            }
        }
    }
    
    println!("{}",total)
}

// A+tU
// B+sV

// t = (Vx(Ay-By)-Vy(Ax-Bx))/(UxVy-UyVx)