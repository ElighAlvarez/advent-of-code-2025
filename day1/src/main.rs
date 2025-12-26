use std::fs;

fn main() {

    let mut val = 50;
    let mut count = 0;

    let file_path = "day1/input/day1.in";
    let contents = fs::read_to_string(file_path).expect("Failed to read from file.");
    let turns = 
        str::split(&contents, "\n")
        .map(|turn| parse_turn(turn));

    for turn in turns {
        count_zeroes(&mut val, &mut count, turn);
    }

    println!("Number of zeroes: {count}");
}

fn parse_turn(turn: &str) -> i32 {
    let direction = if &turn[0..1] == "R" { 1 } else { -1 };
    let size = &turn[1..].parse::<i32>().expect("Invalid Input");
    size * direction
}

// Can probably be simplified with some math.
fn count_zeroes(val: &mut i32, count: &mut i32, turn: i32) {
    if turn < 0 {
        for _ in 0..(turn * -1) {
            *val -= 1;
            if *val % 100 == 0 {
                *count += 1;
            }
        }
    } else {
        for _ in 0..turn {
            *val += 1;
            if *val % 100 == 0 {
                *count += 1;
            }
        }
    }
}