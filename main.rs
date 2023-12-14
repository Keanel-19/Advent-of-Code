use std::fs::read_to_string;
//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Clone,Copy)]
enum Rock {
    Ball,
    Wall,
    Space
}
use Rock::{Ball,Wall,Space};

#[derive(Clone,Copy)]
enum Orient {
    North,
    South,
    East,
    West
}
use Orient::{North,South,East,West};

fn len(grid: &Vec<Vec<Rock>>,dir: Orient) -> usize {
    match dir {
        North|South => grid.len(),
        East|West => grid[0].len()
    }
}

fn len_row(grid: &Vec<Vec<Rock>>,dir: Orient) -> usize {
    match dir {
        North|South => grid[0].len(),
        East|West => grid.len()
    }
}

fn get(grid: &Vec<Vec<Rock>>,dir: Orient,i: usize, j: usize) -> Rock {
    let l = len(grid,dir);
    let lr = len_row(grid,dir);
    match dir {
        North => grid[i][j],
        South => grid[l-i-1][lr-j-1],
        East => grid[lr-j-1][l-i-1],
        West => grid[j][i]
    }
}

fn set(grid: &mut Vec<Vec<Rock>>,dir: Orient,i: usize, j: usize,v: Rock) {
    let l = len(grid,dir);
    let lr = len_row(grid,dir);
    match dir {
        North => grid[i][j] = v,
        South => grid[l-i-1][lr-j-1] = v,
        East => grid[lr-j-1][l-i-1] = v,
        West => grid[j][i] = v
    }
}

fn tilt(grid: &mut Vec<Vec<Rock>>,dir: Orient) {
    let lr = len_row(grid,dir);
    let l = len(grid,dir);
    for j in 0..lr {
        let mut row = 0;
        for i in 0..l {
            row = match get(grid,dir,i,j) {
                Ball => {
                    set(grid,dir,i,j,Space);
                    set(grid,dir,row,j,Ball);
                    row+1
                },
                Wall => i+1,
                Space => row
            }
        }
    }
}

fn count(grid: & Vec<Vec<Rock>>,dir: Orient) -> usize{
    let lr = len_row(grid,dir);
    let l = len(grid,dir);
    let mut total = 0;
    for j in 0..lr {
        for i in 0..l {
            total += match get(grid,dir,i,j) {
                Ball => l-i,
                _ => 0
            }
        }
    }
    total
}

fn main(){
    //let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"([.#?]+) ([\d,]+)").unwrap();
    
    let mut grid: Vec<Vec<Rock>> = text.into_iter().map(|s| s.chars().map(|c| match c {
        'O'=> Ball,
        '#' => Wall,
        '.'=> Space,
        _=> panic!()
    }).collect()).collect();
    //println!("{}*{}={}",grid.len(),grid[0].len(),grid.len()*grid[0].len());
    
    let cycle = 1000000000;
    
    let mut hist = vec![];
    let mut start_loop = 0;
    let mut reminder = 0;
    for l in 0..cycle{
        for dir in [North,West,South,East].into_iter() {
            tilt(&mut grid, dir);
        }
        let h = (count(&grid,North),count(&grid,West));
        match hist.iter().position(|e| *e==h) {
            Some(i) => {
                start_loop = i;
                reminder = cycle-l-1;
                break
            },
            None => hist.push(h)
        }
        //println!("{}: {} {}",l,count(&grid,North));
    }
    let ifin = start_loop + reminder%(hist.len()- start_loop);
    let total = hist[ifin].0;
    
    //println!("{:?}",hist);
    //println!("{} {}",start_loop,ifin);
    println!("{}",total)
}