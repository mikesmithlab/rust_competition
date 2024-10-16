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

fn main() {
    //read in num tests
    let num_tests: usize= read().unwrap();
 
    for _j in 1..=num_tests{
       let g:Vec<u32> = read_vec().unwrap();
       let n:u32=g[0];
       let k:u32 = g[1];
       
       //if n is even k must be 1, or k must divide by 4 or k-1 must divide by 4
       //if n is odd k must divide by 4 or k+1 must divide by 4
       if n%2==0{
          if k==1 || k%4  ==0 || (k-1)%4 ==0 {
            println!("YES");
          }
          else{
            println!("NO");
          }
        }
        else{
            if k%4 == 0 || (k+1)%4 == 0{
                println!("YES");
            }
            else{
                println!("NO");
            }
        }
      
    }

}
