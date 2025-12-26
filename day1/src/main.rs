use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Failed to read from file.");
    let turns = 
        str::split(&contents, "\n")
        .map(|turn| parse_turn(turn));

    let mut val = 50;
    let mut count = 0;
    for turn in turns {
        (val, count) = count_zeroes(val, count, turn);
    }

    println!("Number of zeroes: {count}");
}

fn parse_turn(turn: &str) -> i32 {
    let direction = if &turn[0..1] == "R" { 1 } else { -1 };
    let size = &turn[1..].parse::<i32>().expect("Invalid Input");
    size * direction
}

// Can probably be simplified with some math.
fn count_zeroes(mut val: i32, mut count: i32, turn: i32) -> (i32, i32) {
    if turn < 0 {
        for _ in 0..(turn * -1) {
            val -= 1;
            if val % 100 == 0 {
                count += 1;
            }
        }
    } else {
        for _ in 0..turn {
            val += 1;
            if val % 100 == 0 {
                count += 1;
            }
        }
    }
    (val, count)
}