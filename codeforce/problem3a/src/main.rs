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
        
        
        let num_tests: i64 = read().expect("num_tests not read properly");
        
        for _i in 1..=num_tests{
            //default return value for case n= 1 or 2
            let mut x: i64 = -1;
            
            //read test input
            let n: f64 = read().expect("n not read properly");
            let ai: Vec<i64>=read_vec().expect("Vec not read properly");
            
            //Sum of all earnings
            let sum_val  = ai.iter().sum::<i64>();

            //sorted array of earnings
            let mut sort_ai = ai.clone();
            sort_ai.sort();

            //person in sorted array
            let n_2: usize = ((n + 0.0001)/2.0).ceil() as usize;
            
            if n as i64 > 2{           
                x = 2*n as i64 * sort_ai[n_2-1] - sum_val + 1;
            
                //No need RH already coming else add x to get him to come
                if x <= 0{x=0;};
            
            }

        
            println!("{:?}", x);

        }
    }
