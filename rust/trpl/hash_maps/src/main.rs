use std::collections::HashMap;

fn main() {
    test_new_hash_map();
    test_auth();
    test_get_value();
    test_update_value();
}

fn test_new_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores is {:#?}", scores);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores is {:#?}", scores);
}

fn test_auth() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

//    println!("field_name is {}, field_value is {}", field_name, field_value);
    println!("map is {:#?}", map);
}

fn test_get_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score is {:#?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn test_update_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let e = scores.entry(String::from("Yellow"));
    println!("e is {:?}", e);
    e.or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    let words = text.split_whitespace();
    println!("words is {:?}", words);

    for word in words {
        println!("word is {:?}", word);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map is : {:?}", map);
}
