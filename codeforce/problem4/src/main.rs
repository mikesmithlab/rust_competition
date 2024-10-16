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

#[derive(Debug, Copy, Clone)]
struct Job{
    l:u64,
    r:u64,
    count:u64,
}


impl Job{
    fn add_other(&mut self, other: Job)->Vec<Job>{    
        //Create default jobs to be modified
        let mut job_left = Job{..*self};
        let mut job_centre = Job{..*self};
        let mut job_right = Job{..*self};

        if other.l > self.r {
            //non-overlapping
            vec![*self, other]
        }
        else if self.l==other.l 
        {
            job_left.count += 1;
            if self.r < other.r {
            job_centre.l=self.r + 1;
            job_centre.r = other.r;
            job_centre.count = 1;
            vec![job_left, job_centre]
            }
            else if self.r==other.r{
            //Both ranges the same
            vec![job_left]
            }else{
            //self.r > other.r which only happens for first run where self covers whole range.
            job_left.r = other.r;
            job_centre.l=other.r+1;
            job_centre.r = self.r;
            vec![job_left, job_centre]
            }
        }
        else{
            //self.l < other.l
            job_centre.count += 1;
            if self.r==other.r{
            //rhs match
            job_left.r=other.l-1;
            job_centre.l=other.l;
            job_centre.r=self.r;
            //Both ranges the same
            vec![job_left, job_centre]
            }
            else if self.r < other.r {
            //self rhs below other
            job_left.r = other.l-1;
            job_centre.l=other.l;
            job_centre.r = self.r;
            job_right.l = self.r + 1;
            job_right.r = other.r;
            vec![job_left, job_centre, job_right]
            }else{
            //self.r > other.r which only happens for first run where self covers whole range.
            job_left.r = other.l-1;
            job_centre.l=other.l;
            job_centre.r = other.r;
            job_right.l = other.r + 1;
            job_right.r = self.r;
            vec![job_left, job_centre, job_right]
            }
        }
    }
}       




fn main(){
    //Get num tests
    let num_tests: usize = read().expect("num_tests not read properly");
    
    for _i in 1..=num_tests{     
        //read test input
        let input = read_vec().expect("n not read properly");
        let n:u64 = input[0];
        let d:u64 = input[1];
        let k:u64 = input[2];

        //Create list of jobs
        let mut job_list: Vec<Job> = Vec::new();     
        
        for job_id in 1..=k{
            let job = read_vec().expect("n not read properly");
            job_list.push(Job{l:job[0], r:job[1], count:1});
        }

        //Sort jobs by LHS and then RHS
        job_list.sort_by(|a, b| a.l.cmp(&b.l).then_with(|| a.r.cmp(&b.r)));

        //Initialise with empty spanning range
        let mut condensed_job_list: Vec<Job> = vec![job_list[0]];
        //work left to right splitting repeated ranges and updating counts
        for job in &job_list[1..]{
            let last_index = condensed_job_list.len()-1;
            let new_jobs = condensed_job_list[last_index].add_other(*job);
            condensed_job_list.pop();
            condensed_job_list.extend(new_jobs);
        }
        
        println!("{:?}", condensed_job_list);

        //Find day to visit
        let mut robert_n=0;
        let mut robert_idx=0;
        let mut mum_n=1e6;
        let mut mum_idx=0;
        for (idx, job) in &condensed_job_list.iter().enumerate(){
            if job.n > robert_n{
                robert_idx = idx;
                robert_n = job.n;
            }
            if job.n < mum_n{
                mum_idx = idx;
                mum_n = job.n;
            }
        }
        let robert_day = condensed_job_list[robert_idx].l;
        let mum_day = condensed_job_list[mum_idx].l;

        //Output answers
        println!("{}", robert_day);
        println!("{}", mum_day);*/

    }
}
