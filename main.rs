use std::fs::read_to_string;
//use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn find_ref(v: &Vec<u64>) -> usize {
    for c in 1..v.len() {
        let len = if c < v.len()-c {c} else {v.len()-c};
        if (0..len).all(|i| v[c-1-i]==v[c+i]) {
            return c
        }
    }
    0
}

fn main(){
    let mut total = 0;
    let text = read_input();
    //let reg = Regex::new(r"([.#?]+) ([\d,]+)").unwrap();
    
    let mut maps: Vec<Vec<String>> = vec![vec![]];
    for line in text.into_iter() {
        if line.len() > 0 {
            let last = maps.len()-1;
            maps[last].push(line)
        } else {
            maps.push(vec![])
        }
    }
    
    for grid in maps.into_iter() {
        let mut rows: Vec<u64> = vec![0;grid.len()];
        let mut cols: Vec<u64> = vec![0;grid[0].len()];
        
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let digit = match grid[i].chars().nth(j) {
                    Some('.') => 0,
                    Some('#') => 1,
                    _ => panic!()
                };
                rows[i] = (rows[i] << 1) + digit;
                cols[j] = (cols[j] << 1) + digit;
            }
        }
        
        total += find_ref(&cols) + 100*find_ref(&rows);
        
        //println!("{:?} {:?}",rows,cols)
    }
    
    println!("{}",total)
}