use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{} team score: {}", key, value);
    }
}