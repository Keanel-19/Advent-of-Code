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
    
    let erows: Vec<usize> = (0..grid.len()).filter(|i| (0..grid[*i].len()).all(|j| grid[*i][j]=='.')).collect();
    let ecols: Vec<usize> = (0..grid[0].len()).filter(|j| (0..grid.len()).all(|i| grid[i][*j]=='.')).collect();
    
    let mut stars = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j]=='#' {
                stars.push((i,j))
            }
        }
    }
    
    
    for s in 0..stars.len()-1 {
        for t in s..stars.len() {
            let (x,y) = stars[s];
            let (u,v) = stars[t];
            total += if x > u {
                x-u
            } else {
                u-x
            };
            
            total += if x > u {
                erows.iter().filter(|i| u<**i && **i<x).count()
            } else {
                erows.iter().filter(|i| x<**i && **i<u).count()
            }*(1000000-1);
            
            total += if y > v {
                y-v
            } else {
                v-y
            };
            
            total += if y > v {
                ecols.iter().filter(|j| v<**j && **j<y).count()
            } else {
                ecols.iter().filter(|j| y<**j && **j<v).count()
            }*(1000000 -1);
        }
    }
    
    println!("{}",total)
}