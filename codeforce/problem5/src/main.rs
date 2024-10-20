//Reading standard input

use std::io;
use std::str::FromStr;
use std::collections::HashMap;


#[allow(dead_code)]
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    buffer
}

#[allow(dead_code)]
fn read<T : FromStr>() -> Result<T, T::Err>{
    read_line().trim().parse::<T>()
}

#[allow(dead_code)]
fn read_vec<T : FromStr>() -> Result< Vec<T>, T::Err>{
    let line = read_line();
    
    line.split_whitespace().map(|x| x.parse::<T>()).collect()
}


#[derive(Debug, Clone)]
struct Node
{
    adj_nodes:Vec<usize>,
    wts:Vec<usize>,
    visited:Vec<bool>,
    horse:bool,
}

impl Node{
    fn new(horse: bool) -> Self{
        Node{
            adj_nodes : Vec::new(),
            wts : Vec::new(),
            visited : Vec::new(),
            horse : horse,
        }
    }

    fn add_edge(&mut self, neighbour: usize, weight: usize){
        self.adj_nodes.push(neighbour);
        if self.horse{
            self.wts.push(weight/2);
        }else{
            self.wts.push(weight);
        }
        self.visited.push(false);
    }

    fn find_closest(&self) -> usize{
        //find closest unvisited neighbour
        let mut closest: usize = 100000;
        let mut closest_neighbour : Option<usize>= None;
        for (idx, &neighbour) in self.adj_nodes.iter().enumerate(){
            if !(self.visited[idx]){
                if self.wts[idx] < closest{
                    closest=self.wts[idx];
                    closest_neighbour = Some(neighbour);
                }
            }
        }
        closest_neighbour.unwrap()
    }
}


fn main(){
    //Get num tests
    let num_tests: usize = read().expect("num_tests not read properly");
    let mut node_hash: HashMap<usize,Node> = HashMap::new();
    
    for _i in 1..=num_tests{     
        //read test input
        let input = read_vec().expect("n not read properly");
        let n:usize = input[0];//num nodes
        let m:usize = input[1];//num edges
        let h:usize = input[2];//num vertices single horse

        let single_h: Vec<usize> = read_vec().expect("n not read properly");//
        println!("horse nodes {:?}",single_h);

        let mut times_1_n: Vec<usize> = vec![200000;n-1];
        let mut times_n_1: Vec<usize> = vec![200000;n-1];
        
            
        //iterate over edges
        for _edge in 1..=m{
            let line: Vec<usize> = read_vec().expect("n not read properly");
            let u:usize = line[0];//start node
            let v:usize = line[1];//end node
            let w:usize = line[2];//even travel time

            //Create nodes with and without horses    
            node_hash.entry(u).or_insert(match single_h.contains(&u){
                true => Node::new(true),
                false => Node::new(false),
            });
            
            //Do same for second node      
            node_hash.entry(v).or_insert(match single_h.contains(&v){
                true => Node::new(true),
                false => Node::new(false),
            });
            
            //Add the edges. scaling weights done internally using self.horse
            match node_hash.get_mut(&u){
                Some(node)=> node.add_edge(v,w),
                None => ()
            }
            match node_hash.get_mut(&v){
                Some(node)=> node.add_edge(u,w),
                None => ()
            }
        }

        let start_node = 1;
        times_1_n[start_node] = 0;
        println!("neighbours = {:?}", node_hash[&start_node].adj_nodes);
        let mut closest_neighbour =  node_hash[&start_node].find_closest();
        times_1_n[closest_neighbour] = 
        
    }
    println!("network {:?}", node_hash);
    
}
