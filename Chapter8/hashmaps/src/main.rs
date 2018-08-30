use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let blue_score: i32 = match scores.get("Blue") {
        Some(value) => *value,
        None => 0
    };

    let red_score: i32 = match scores.get("Red") {
        Some(value) => *value,
        None => 0
    };

    println!("Team Blue score: {:?}", blue_score);
    println!("Team Red score: {:?}", red_score);

    
    {
        let blue_score = scores.entry(String::from("Blue")).or_insert(0);
        *blue_score += 20;
    }
    //scores.insert(String::from("Blue"), 70);
    scores.entry(String::from("Yellow")).or_insert(30);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
