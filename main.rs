use std::fs::read_to_string;
use std::collections::{BTreeSet,HashMap};
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
    Wall
}
use Cell::{Blank,Wall};


fn is_free(g: &Vec<Vec<Cell>>, (i,j): (usize,usize)) -> bool{
    i < g.len() && j < g[0].len() && match g[i][j] {
        Blank => true,
        Wall => false
    }
}


fn nexts(g: &Vec<Vec<Cell>>, (i,j): (usize,usize)) -> impl Iterator<Item=(usize,usize)>+'_ {
     [Up,Down,Right,Left].into_iter().filter_map(move |d| d.depl((i,j))).filter(|p| is_free(g,*p))
}


fn parcours(g: &Vec<Vec<Cell>>, m: &BTreeSet<(usize,usize)>, from: (usize,usize), p: (usize,usize), len: u32) -> ((usize,usize),u32) {
    if m.contains(&p) {
        return (p,len)
    }
    let mut v = nexts(g,p);
    let Some(mut to) = v.next() else {
        panic!("Pas de voisins")
    };
    while to == from {
        to = v.next().unwrap();
    }
    return parcours(g,m,p,to,len+1)
}


fn f(g: &HashMap<(usize,usize),Vec<((usize,usize),u32)>>, m: &mut BTreeSet<(usize,usize)>, p: (usize,usize), len: u32, end: (usize,usize)) -> Option<u32> {
    if m.contains(&p) {
        None
    } else if p==end {
        Some(len)
    } else {
        m.insert(p);
        let rep= g[&p].iter().filter_map(|(p,l)| f(g,m,*p,len+*l,end)).max();
        m.remove(&p);
        rep
    }
}


fn main(){
    //let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();
    
    let grid: Vec<Vec<Cell>> = text.into_iter().map(|s| s.chars().map(|c| match c {
        '#' => Wall,
        _ => Blank
    }).collect()).collect();
    
    let j = (0..grid[0].len()).filter(|j| match grid[0][*j] {Blank=> true, Wall=> false}).next().unwrap();
    let start = (0,j);
    let j = (0..grid[0].len()).filter(|j| match grid[grid.len()-1][*j] {Blank=> true, Wall=> false}).next().unwrap();
    let end = (grid.len()-1,j);
    
    
    let mut nodes = BTreeSet::new();
    nodes.insert(start);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_free(&grid,(i,j)) && nexts(&grid,(i,j)).count() > 2 {
                nodes.insert((i,j));
                
            }
        }
    }
    nodes.insert(end);
    
    let mut graph = HashMap::new();
    for p in nodes.clone().into_iter() {
        let mut voisins = vec![];
        for v in nexts(&grid,p) {
            voisins.push(parcours(&grid,&nodes,p,v,1))
        }
        graph.insert(p,voisins);
    }
     ;
    
    let mut mark=BTreeSet::new();
    let total = f(&graph,&mut mark, start, 0,end).unwrap();
    
    println!("{}",total)
}
6682