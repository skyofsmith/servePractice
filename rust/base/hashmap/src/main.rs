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

    // let keys = vec![String::from("Green"), String::from("Yellow")];
    // let values = vec![30, 40];
    // let scores: HashMap<String, i32> = keys.iter().zip(values.iter()).collect();
    // println!("scores = {:?}", scores);

    let k = String::from("Blue");
    if let Some(v) = scores.get(&k) {
        println!("v = {}", v);
    } else {
        println!("None");
    }
    let kk = String::from("Black");
    let vv = scores.get(&kk);
    match vv {
        Some(value) => println!("v = {}", value),
        None => println!("None"),
    }

    for (key, value) in &scores {
        println!("key = {}, value = {}", key, value);
    }

    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.insert(String::from("one"), 11);
    ss.entry(String::from("two")).or_insert(22);
    ss.entry(String::from("four")).or_insert(44);
    println!("ss = {:#?}", ss);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:#?}", map);
}
