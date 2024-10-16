use std::str::FromStr;

#[allow(dead_code)]
fn read_file()->String{
    let input = include_str!("input.txt").to_string();
    input
}

#[allow(dead_code)]
fn read_vec<T: FromStr>(puzzle_input: String) -> Result<Vec<T>, T::Err>{
    let mut input = Vec::new();
    for line in puzzle_input.split("\r\n"){
        match line.parse::<T>() {
            Ok(value) => input.push(value),
            Err(e) => return Err(e),
        }   
    }

   
    Ok(input)
}




fn solution(puzzle_input: String)->u32{
   5_u32
}

fn main(){
    let input: String = read_file();
    println!("{:?}",input);
    solution(input);
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_code(){

        let test_input="Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();

        let test_answer=5_u32;
        assert_eq!(solution(test_input),test_answer);

    }


}
