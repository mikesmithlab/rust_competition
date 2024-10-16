//Reading standard input

use std::io;
use std::str::FromStr;

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
    read_line().split_whitespace().map(|x| x.parse::<T>()).collect()
}

fn take_money(mut robin: u32, mut ai: u32, k: u32)->(u32, u32){
    if ai >= k{
        robin += ai;
        (robin, 0)
    }
    else if (ai == 0) & (robin > 0){
        ai = 1;
        robin -= 1;
        (robin, 1)
    }
    else {
     (robin, 0)   
    }
    
}

fn main() {        
    //read in num tests
    let num_tests: usize= read().unwrap();
    
    //Count number of people "poor" who get given gold in each test
    for i in 1..=num_tests{
        //create vars
        let mut robin=0;
        let mut poor=0;
        let mut add2count;
        
        //read in test data
        let n_k: Vec<u32> = read_vec().unwrap();
        let people: Vec<u32> = read_vec().unwrap();

        let n: u32=n_k[0];
        let k: u32=n_k[1];


        for person in people{
            let ai:u32=person;
            (robin, add2count)=take_money(robin, ai, k);
            poor += add2count;
        }
        println!("{}", poor);
        
    }

}
