use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map is {:#?}", map)

    
}
