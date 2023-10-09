use std::fs;
use std::env;
use csv::ReaderBuilder;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = parse_configs(&args);
    println!("Hello, world!");
    let mut total_score = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        println!("{}", line);
        total_score = total_score + shoot(&line[..1], &line[2..]);
        println!("{}", total_score);
    }
}

fn parse_configs(args: &[String]) -> &str {
    let file_path = &args[1];
    file_path
}

fn shoot(opponent_hand: &str, my_hand: &str) -> i32 {
    let mut opponent_value = 0;
    let mut my_value = 0;
    let mut score=0;
    // Rock
    if opponent_hand == "A" {
        opponent_value=1;
    // Paper
    } else if opponent_hand == "B" {
        opponent_value=2;
    // Scissors
    } else if opponent_hand == "C" {
        opponent_value=3;
    }
    // Rock
    if my_hand == "X" {
        my_value=1;
    // Paper
    } else if my_hand == "Y" {
        my_value=2;
    // Scissors
    } else if my_hand == "Z" {
        my_value=3;
    }

    if my_value==opponent_value {
        score=score+3+my_value;
    } else if my_value == 3 && opponent_value == 1 {
        score=score+my_value;
    } else if (my_value>opponent_value) || (my_value==1 && opponent_value ==3) {
        score=score+6+my_value;
    } else {
        score=score+my_value;
    }
    return score;
}
