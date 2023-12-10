use std::fs::read_to_string;
//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn main(){
    let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"-?\d+").unwrap();
    
    let grid: Vec<Vec<char>> = text.iter().map(|s| s.chars().collect()).collect();
    
    let s_point = (0..grid.len()).flat_map(|i| (0..grid[i].len()).map(move |j| (i,j))).filter(|t| grid[t.0][t.1]=='S').next().unwrap();
    
    let starts = [
        if 0 < s_point.0 {Some((s_point.0-1,s_point.1))} else {None},
        if 0 < s_point.1 {Some((s_point.0,s_point.1-1))} else {None},
        if s_point.0+1 < grid.len() {Some((s_point.0+1,s_point.1))} else {None},
        if s_point.1+1 < grid[s_point.0].len() {Some((s_point.0,s_point.1+1))} else {None}
    ].into_iter().filter(|x| x.is_some()).map(|x| x.unwrap());
    
    for p in starts {
        let mut actuel = s_point;
        let mut next = p;
        let mut len = 0;
        let mut found = false;
        
        loop {
            if grid[next.0][next.1] == 'S' {
                found = true;
                break
            }
            (actuel,next) = (next,match (next.0 as isize-actuel.0 as isize,next.1 as isize-actuel.1 as isize) {
                ( 1,0) => match grid[next.0][next.1] {
                    '|' => (next.0+1,next.1),
                    'L' => (next.0,next.1+1),
                    'J' => (next.0,next.1-1),
                    _ => break
                },
                (-1,0) => match grid[next.0][next.1] {
                    '|' => (next.0-1,next.1),
                    'F' => (next.0,next.1+1),
                    '7' => (next.0,next.1-1),
                    _ => break
                },
                (0, 1) => match grid[next.0][next.1] {
                    '-' => (next.0,next.1+1),
                    '7' => (next.0+1,next.1),
                    'J' => (next.0-1,next.1),
                    _ => break
                },
                (0,-1) => match grid[next.0][next.1] {
                    '-' => (next.0,next.1-1),
                    'L' => (next.0-1,next.1),
                    'F' => (next.0+1,next.1),
                    _ => break
                },
                _ => break
            });
            len +=1;
        }
        if found {
        total = (len+1)/2;
            break
        }
    }
    
    println!("{:?}", total)
}