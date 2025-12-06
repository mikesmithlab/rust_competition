
//! cargo-script-edition:2021
//! regex = "1.6"

use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d+$").unwrap();
    println!("Is '123' numeric? {}", re.is_match("123"));
}
