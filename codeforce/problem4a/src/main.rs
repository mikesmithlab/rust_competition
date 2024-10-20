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

        //Create list of jobs
        let mut job_list: Vec<(usize,usize)> = Vec::new();     
        let mut job_list_finish: Vec<(usize,usize)> = Vec::new();
        
        for job_id in 1..=k{
            let job = read_vec().expect("n not read properly");
            job_list.push((job[0],job[1]));
            
        }

        //Sort jobs by LHS and then RHS
        job_list.sort_by(|x, y| x.partial_cmp(y).unwrap());

        let mut robert : usize = 0;
        let mut robert_day : usize = 1;
        let mut robert_count : usize = 0;
        let mut mum: usize = 100000000000000;
        let mut mum_day : usize = 1;
        let mut mum_count : usize = 0;

        
        for day in 1..=n-d+1{
            let mut robert_count = 0;
            let mut mum_count = 0;
            
            let mut idx_remove = Vec::new();
            for (idx, job) in job_list.iter().enumerate(){
                if day + 1 > job.1  {
                    //job ends before you arrive
                    idx_remove.push(idx);
                }
                
                if (day >= job.0) & (day <= job.1) {
                    // arrival during job
                    robert_count += 1;
                    mum_count += 1;
                }
                else if (day + d - 1 >= job.0) & (day + d - 1 <= job.1) {
                    //leaving during job
                    robert_count += 1;
                    mum_count += 1;
                }
                else if (day <= job.0) & (day + d - 1 >= job.1){
                    //stay spans job
                    robert_count += 1;
                    mum_count += 1;
                }
                else if day + d - 1 < job.0 {
                    //job starts after leave
                    break;
                }
            }

            for idx in idx_remove.iter().rev(){
                job_list.remove(*idx);
            }

            //update days
            if robert_count > robert{
                robert = robert_count;
                robert_day = day;
            }

            if mum_count < mum{
                mum = mum_count;
                mum_day = day;
            }   

        }
        println!{"{} {}", robert_day, mum_day};
        

    }
}
