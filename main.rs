use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Rel {
    from: u32,
    to: u32,
    len: u32
}

#[derive(Debug)]
struct Seeds {
    start: u32,
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
    
    let reg = Regex::new(r"(\d+) (\d+)").unwrap();
    let mut seeds : Vec<Seeds>= reg.captures_iter(&text[0]).map(|c| Seeds{start:c[1].parse().unwrap(), len:c[2].parse().unwrap()}).collect();
    seeds.sort_by(|s1,s2| s1.start.cmp(&s2.start));
    
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
        let mut newseed: Vec<Seeds> = Vec::new();
        
        while iseed < seeds.len() && irel < map.len() {
            let s = &seeds[iseed];
            let r = &map[irel];
            println!("{:?} {:?}",s,r );
            
            if s.start < r.from {
                println!("cat1");
                if s.start+s.len-1 < r.from {
                    newseed.push(Seeds{
                        start:s.start,
                        len:s.len
                    });
                    iseed+=1
                } else {
                    newseed.push(Seeds{
                        start:s.start,
                        len: r.from - s.start
                    });
                    seeds[iseed] = Seeds{
                        start:r.from,
                        len: s.start + s.len - r.from 
                    };
                }
            } else if s.start-r.from >= r.len {
                irel += 1;
                println!("cat2");
            } else {
                println!("cat3");
                if s.start-r.from+s.len > r.len {
                println!("scat1");
                    newseed.push(Seeds{
                        start: s.start-r.from+r.to,
                        len: r.from + r.len - s.start
                    });
                    seeds[iseed] = Seeds{
                        start:r.from+r.len,
                        len:s.start-r.from+s.len-r.len
                    }
                } else {
                    println!("scat2");
                    newseed.push(Seeds{
                        start: s.start-r.from+r.to,
                        len: s.len
                    });
                    iseed+=1;
                }
            }
        }
        if irel >= map.len() {
            for u in iseed..seeds.len() {
                let s = &seeds[u];
                newseed.push(Seeds{
                    start:s.start,
                    len:s.len
                })
            }
        }
        seeds = newseed;
        seeds.sort_by(|s1,s2| s1.start.cmp(&s2.start));
        
        println!("{:?}", seeds);
    }
    //println!("{:?}", seeds);
    let total = seeds[0].start;
    println!("{}", total)
    
}