fn lowest_sum_of_scores(scores: &Vec<i32>, rounds: u32) -> i32 {
    if scores.len() == 0 || rounds == 0 {
        return 0;
    }
    if rounds == 1 {
        return *scores.iter().max().unwrap();
    }
    let mut scores = scores.clone();
    scores.sort();
    if rounds == 2 {
        todo!(
            "sum scores with its reverse and then find the lowest. try to look for a one liner ;)"
        )
    }
    todo!("the other cases")
}

fn main() {
    todo!("get user input");
}
