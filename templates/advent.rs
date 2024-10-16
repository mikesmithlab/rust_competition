use std::fs

#[allow(dead_code)]
fn read_file(filename: String)->String{
    let input = include_str!("./input.txt");
    println!(input);
    input
}

#[allow(dead_code)]
fn read_vec<T : FromStr>() -> Result< Vec<T>, T::Err>{
    read_line().split_whitespace().map(|x| x.parse::<T>()).collect()
}


fn solution(){
   println!("");
}

fn main(){

    
    solution();
}



#[cfg(test)]
mod tests{
    #[test]
    fn test_code(){

        solution()




    }


}
