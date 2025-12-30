
fn judge_score(score: i32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    }
}

fn main(){
    let score = 85;
    let grade = judge_score(score);
    println!("Score: {}, Grade: {}", score, grade);

    let score = 65;
    let grade = judge_score(score);
    println!("Score: {}, Grade: {}", score, grade);

}
