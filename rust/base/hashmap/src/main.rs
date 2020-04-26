//1、HashMap<K, V>
//2、创建HashMap
//3、读取
//4、遍历
//5、更新
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    println!("scores = {:#?}", scores);

    let keys = vec![String::from("Green"), String::from("Yellow")];
    let values = vec![30, 40];
    let scores: HashMap<String, i32> = keys.iter().zip(values.iter()).collect();
    // println!("scores = {:?}", scores);
}
