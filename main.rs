use std::fs::read_to_string;
use std::collections::BTreeMap;
use regex::Regex;


fn read_input() -> Vec<String> {
    read_to_string("input")
       .unwrap()
       .lines()
       .map(String::from)
       .collect()
}

fn ajout(d: &mut BTreeMap<String,Vec<String>>, k: &str, v: &str) {
    if let Some(l) = d.get_mut(k) {
        l.push(v.to_string());
    } else {
        d.insert(k.to_string(),vec![v.to_string()]);
    }
}


fn find_chemin(gv: &Vec<Vec<usize>>, ga: &Vec<Vec<bool>>, l: &mut Vec<usize>, end: usize, m: &mut Vec<bool>) -> bool {
    
    let Some(n) = l.last() else {panic!("liste noeud vide")};
    let n = n.clone();
    //println!("{:?} {}",l,end);
    if n == end {
        
        true
    } else {
        for v in gv[n].iter().cloned() {
            if !l.contains(&v) && ga[n][v] && m[v] {
                l.push(v);
                m[v] = false;
                if find_chemin(gv,ga,l,end,m) {
                    return true
                }
                l.pop();
            }
        }
        false
    }
}


fn max_flux(g: &Vec<Vec<usize>>, f: usize, t: usize) -> u32 {
    let mut avail: Vec<Vec<bool>> = vec![vec![false;g.len()];g.len()];
    for i in 0..g.len() {
        for j in g[i].iter().cloned() {
            avail[i][j]= true;
        }
    }
    
    let mut chemin = vec![f];
    
    let mut fmax = 0;
    let mut mark = vec![true;g.len()];
    while find_chemin(&g, &avail, &mut chemin, t,&mut mark) {
        mark = vec![true;g.len()];
        for i in 0..chemin.len()-1 {
            let a = chemin[i];
            let b = chemin[i+1];
            avail[a][b] = false;
            avail[b][a] = true;
        }
        
        chemin.clear();
        chemin.push(f);
        fmax+=1;
        if fmax > 3 {
            break
        }
    }
    
    fmax
} 


fn main(){
    let mut total = 0;
    let text = read_input();
    let reg = Regex::new(r"^(.+): (.+)$").unwrap();
    let mut input = BTreeMap::new();
    for line in text.into_iter() {
        let cap = reg.captures(&line).unwrap();
        let k = &cap[1];
        let values = cap[2].split(" ");
        for v in values {
            ajout(&mut input, k, v);
            ajout(&mut input, v, k);
        }
    }
    
    println!("len : {}",input.len());
    
    let a = input.keys().cloned().enumerate().map(|(i,k)| (k,i)).collect::<BTreeMap<String,usize>>();
    
    let graph = input.iter().map(|(_,v)| v.iter().map(|s| a[s]).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();
    
    
    for n in 1..graph.len() {
        //println!("0 -> {} : {}",n,max_flux(&graph,0,n))
        if max_flux(&graph,0,n) <= 3 {
            total+=1;
        }
        println!("etape {}",n);
    }
    /*
    for (n,l) in input.into_iter() {
        print!("{} :",n);
        for n in l.into_iter() {
            print!(" {}", n)
        }
        println!("");
    }*/
    
    println!("{:?}",total*(graph.len() as u32-total))
}
