use std::fs::read_to_string;
use std::collections::VecDeque;

//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Clone,Copy,Debug)]
enum Orient {
    North,
    South,
    East,
    West
}
use Orient::{North,South,East,West};

impl Orient {
    fn depl(&self, c: (usize,usize)) -> Option<(usize,usize)> {
        match self {
            North => if c.0 > 0 {
                Some((c.0-1,c.1))
            } else {
                None
            },
            South => Some((c.0+1,c.1)),
            West => if c.1 > 0 {
                Some((c.0,c.1-1))
            } else {
                None
            },
            East => Some((c.0,c.1+1))
        }
    }
    
    fn get_mark(&self, m: &(bool,bool,bool,bool)) -> bool {
        match self {
            North => m.0,
            South => m.1,
            East => m.2,
            West => m.3
        }
    }
    
    fn set_mark(&self, m: &mut (bool,bool,bool,bool)) {
        match self {
            North => m.0 = true,
            South => m.1 = true,
            East => m.2 = true,
            West => m.3 = true
        }
    }
}


fn act(c: char, o: Orient) -> Vec<Orient>{
    match c {
        '-' => match o {
            North|South => vec![East,West],
            o => vec![o]
        },
        '|' => match o {
            East|West => vec![North,South],
            o => vec![o]
        },
        '/' => match o {
            North => vec![East],
            South => vec![West],
            East => vec![North],
            West => vec![South],
        },
        '\\' => match o {
            North => vec![West],
            South => vec![East],
            East => vec![South],
            West => vec![North],
        },
        '.' => vec![o],
        _ => panic!()
    }
}


fn main(){
    let mut total:u32 = 0;
    let text = read_input();
    //let reg = Regex::new(r"(\w+)(-|=)(\d?)").unwrap();
    
    let layout: Vec<Vec<char>> = text.iter().map(|s| s.chars().collect()).collect();
    
    for first_o in [North,South,East,West] {
        let b_sup = match first_o {
            North|South => layout[0].len(),
            _ => layout.len()
        };
        for first_c in (0..b_sup).map(|i| match first_o {
            South => (0,i),
            North => (layout.len()-1,i),
            East => (i,0),
            West => (i,layout[0].len()-1),
        }) {
            let mut mark = vec![vec![(false,false,false,false);layout[0].len()];layout.len()];
            let mut front = VecDeque::from([(first_c,first_o)]);
            East.set_mark(&mut mark[first_c.0][first_c.1]);
            //println!("{:?}",front[0]);
            
            while front.len() > 0 {
                let (coor,orient) = front.pop_front().unwrap();
                let orients = act(layout[coor.0][coor.1],orient);
                
                for o in orients.into_iter() {
                    if let Some(c) = o.depl(coor) {
                        if c.0 < layout.len() && c.1 < layout[0].len() && !o.get_mark(&mark[c.0][c.1]) {
                            o.set_mark(&mut mark[c.0][c.1]);
                            front.push_back((c,o));
                        }
                    }
                }
            }
            
            /*
            for r in mark.iter() {
                let mut s = String::new();
                for t in r.iter() {
                    s.push(if t.0||t.1||t.2||t.3 {'#'} else {'.'});
                }
                println!("{}",&s);
            }*/
            let score = mark.iter().map(|r| r.iter().map(|t| if t.0||t.1||t.2||t.3 {1} else {0}).sum::<u32>()).sum();
            
            total = if total<score {score} else {total};
        }
    }
    
    println!("{:?}",total)
}