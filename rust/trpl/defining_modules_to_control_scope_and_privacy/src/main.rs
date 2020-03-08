use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map is {:#?}", map);

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number is {}", secret_number);
    
}
