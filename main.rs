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
    
    let mut grid: Vec<Vec<char>> = text.iter().map(|s| s.chars().collect()).collect();
    
    let s_point = (0..grid.len()).flat_map(|i| (0..grid[i].len()).map(move |j| (i,j))).filter(|t| grid[t.0][t.1]=='S').next().unwrap();
    
    let starts = [
        if 0 < s_point.0 {Some((s_point.0-1,s_point.1))} else {None},
        if 0 < s_point.1 {Some((s_point.0,s_point.1-1))} else {None},
        if s_point.0+1 < grid.len() {Some((s_point.0+1,s_point.1))} else {None},
        if s_point.1+1 < grid[s_point.0].len() {Some((s_point.0,s_point.1+1))} else {None}
    ].into_iter().filter(|x| x.is_some()).map(|x| x.unwrap());
    
    let mut path = vec![];
    
    for p in starts {
        let mut actuel = s_point;
        let mut next = p;
        path = vec![p];
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
            path.push(next);
        }
        if found {
            break
        }
    }
    
    let mut g = vec![vec![false;grid[0].len()];grid.len()];
    for (i,j) in path.iter() {
        g[*i][*j] = true
    }
    
    let a = path[0];
    let b = path[path.len()-2];
    grid[s_point.0][s_point.1] = match (
        s_point.0 as isize-a.0 as isize,
        s_point.1 as isize-a.1 as isize,
        s_point.0 as isize-b.0 as isize,
        s_point.1 as isize-b.1 as isize
    ) {
        (1,0,-1,0) => '|',
        (-1,0,1,0) => '|',
        (0,1,0,-1) => '-',
        (0,-1,0,1) => '-',
        (0,1,1,0) => 'J',
        (1,0,0,1) => 'J',
        (0,-1,-1,0) => 'F',
        (-1,0,0,-1) => 'F',
        (0,-1,1,0) => 'L',
        (1,0,0,-1) => 'L',
        (0,1,-1,0) => '7',
        (-1,0,0,1) => '7',
        _ => panic!()
    };
    
    
    for i in 0..g.len() {
        let mut inner = false;
        let mut o = 0;
        for j in 0..g[i].len() {
            if g[i][j] {
                o += match grid[i][j] {
                    'L'|'J' => 1,
                    '7'|'F' => -1,
                    _ => 0
                };
                if o == 0 {
                    inner = !inner
                } else if o == 2 || o == -2 {
                    o = 0
                }
                
            } else if inner {
                total+=1
            }
        }
    }
    
    println!("{:?}", total)
}