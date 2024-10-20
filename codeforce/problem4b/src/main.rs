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
    let line = read_line();
    
    line.split_whitespace().map(|x| x.parse::<T>()).collect()
}



fn main(){
    //Get num tests
    let num_tests: usize = read().expect("num_tests not read properly");
    
    for _i in 1..=num_tests{     
        //read test input
        let input = read_vec().expect("n not read properly");
        let n:usize = input[0];
        let d:usize = input[1];
        let k:usize = input[2];
            
        let mut job_list: Vec<i64> = vec![0; n+2];      
        
        for job_id in 1..=k{
            let job: Vec<usize> = read_vec().expect("n not read properly");
            if job[0]+1<=d{
                job_list[1]+=1;
            }else{
                job_list[job[0]-d+1] += 1;
            }
            job_list[job[1]+1] -= 1;
        }

        let mut cumsum : i64= 0;
        let mut robert = 0;
        let mut robert_count = 0;
        let mut mum = 0;
        let mut mum_count = 1000000;
        for day in 1..n-d+2{
            cumsum += job_list[day];
            if cumsum > robert_count{
                robert_count = cumsum;
                robert = day;
            }
            if cumsum < mum_count{
                mum_count = cumsum;
                mum = day;
            }
        }
        println!("{} {}", robert, mum);
    }
}
