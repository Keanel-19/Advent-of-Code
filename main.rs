use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Rel {
    from: u32,
    to: u32,
    len: u32
}

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn main(){
    //let mut total =0;
    let text = read_input();
    let reg = Regex::new(r"^.+:$").unwrap();
    
    let i_maps:Vec<usize> = (0..text.len()).filter(|i| reg.is_match(&text[*i])).map(|i| i+1).collect();
    
    //println!("{:?}", i_maps);
    
    let reg = Regex::new(r"\d+").unwrap();
    let mut seeds : Vec<u32>= reg.find_iter(&text[0]).map(|m| m.as_str().parse().unwrap()).collect();
    seeds.sort();
    
    //println!("{:?}", seeds);
    
    let reg2 = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    for i in i_maps.iter() {
        let mut j = 0;
        let mut map : Vec<Rel> = Vec::new();
        while i+j < text.len() && reg2.is_match(&text[i+j]) {
            let cap = reg2.captures(&text[i+j]).unwrap();
            let (_, rel): (&str, [&str; 3]) = cap.extract();
            let rel: Vec<u32> = rel.iter().map(|s| s.parse().unwrap()).collect();
            map.push(Rel{from:rel[1],to:rel[0],len:rel[2]});
            j += 1;
        }
        map.sort_by(|r1,r2| r1.from.cmp(&r2.from));
        
        //println!("{:?}",map);
        
        let mut iseed = 0;
        let mut irel = 0;
        
        while iseed < seeds.len() && irel < map.len() {
            let s = seeds[iseed];
            let r = &map[irel];
            
            if s < r.from {
                iseed+=1;
            } else if s-r.from >= r.len {
                irel += 1;
            } else {
                seeds[iseed] = s-r.from+r.to;
                iseed +=1;
            }
        }
        seeds.sort();
        
        //println!("{:?}", seeds);
    }
    //println!("{:?}", seeds);
    let total = seeds[0];
    println!("{}", total)
    
}