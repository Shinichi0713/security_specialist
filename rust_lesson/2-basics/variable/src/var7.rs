use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(score) => println!("The score for team {} is {}", team_name, score),
        None => println!("Team {} not found", team_name),
    }

    scores.entry(String::from("Green")).or_insert(30);
    println!("Scores: {:?}", scores);

}


