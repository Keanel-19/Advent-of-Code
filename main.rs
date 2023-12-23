use std::fs::read_to_string;
//use std::collections::BTreeSet;
//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

#[derive(Clone,Copy,Debug)]
enum Dir{
    Up,
    Right,
    Left,
    Down
}
use Dir::{Up,Right,Left,Down};

impl Dir {
    fn depl(&self, (i,j): (usize,usize)) -> Option<(usize,usize)> {
        match self {
            Up => if i > 0 {
                Some((i-1,j))
            } else {
                None
            },
            Left => if j > 0 {
                Some((i,j-1))
            } else {
                None
            },
            Right => Some((i,j+1)),
            Down => Some((i+1,j))
        }
    }
}

#[derive(Clone,Copy,Debug)]
enum Cell{
    Blank,
    Wall,
    Slop(Dir)
}
use Cell::{Blank,Wall,Slop};


fn path(g: &Vec<Vec<Cell>>, m: &mut Vec<Vec<bool>>, (i,j): (usize,usize)) -> Option<usize> {
    if i < g.len() && j < g[0].len() && !m[i][j] {
        m[i][j]=true;
        let rep = match g[i][j] {
            Wall => None,
            Slop(d) => d.depl((i,j)).and_then(|p| path(g,m,p)),
            Blank => if i == g.len()-1 {
                Some(0)
            } else {
                [Up,Down,Right,Left].into_iter().filter_map(|d| d.depl((i,j)).and_then(|p| path(g,m,p))).max()
            }
        }.and_then(|x| Some(x+1));
        m[i][j] = false;
        return rep;
    }
    None
}


fn main(){
    //let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();
    
    let grid: Vec<Vec<Cell>> = text.into_iter().map(|s| s.chars().map(|c| match c {
        '.' => Blank,
        '#' => Wall,
        '>' => Slop(Right),
        '<' => Slop(Left),
        '^' => Slop(Up),
        'v' => Slop(Down),
        _ => panic!()
    }).collect()).collect();
    let mut mark = vec![vec![false;grid[0].len()];grid.len()];
    
    let len = (0..grid[0].len()).filter_map(|j| path(&grid, &mut mark, (0,j))).next().unwrap()-1;
    
    println!("{}", len)
}
