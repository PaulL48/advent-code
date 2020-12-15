use std::collections::HashSet;
use std::env;
use std::fs;

const ROW_UPPER: char = 'B';
const ROW_LOWER: char = 'F';

const COL_UPPER: char = 'R';
const COL_LOWER: char = 'L';

const ROWS: i32 = 127;
const COLS: i32 = 7;

fn binary_space_partition(
    mut min: i32,
    mut max: i32,
    upper_char: char,
    lower_char: char,
    sequence: &str,
) -> i32 {
    for c in sequence.chars() {
        let diff = ((max - min) as f32 / 2.0).ceil() as i32;
        if c == upper_char {
            min += diff;
        } else if c == lower_char {
            max -= diff;
        } else {
            panic!("Unexpected character in sequence");
        }
    }
    return min;
}

fn seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

fn seat_id_from_sequence(sequence: &str) -> i32 {
    let coordinate = seat_coordinate(sequence);
    seat_id(coordinate.0, coordinate.1)
}

fn seat_coordinate(sequence: &str) -> (i32, i32) {
    (
        binary_space_partition(0, ROWS, ROW_UPPER, ROW_LOWER, &sequence[..7]),
        binary_space_partition(0, COLS, COL_UPPER, COL_LOWER, &sequence[7..]),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("You gotta supply a file bro");

    let mut seat_ids = contents
        .split("\n")
        .map(|sequence| seat_id_from_sequence(sequence))
        .collect::<Vec<i32>>();
    seat_ids.sort();

    let mut iter = seat_ids.iter();
    let mut previous_value = iter.next().unwrap();
    for value in iter {
        if *value != previous_value + 1 {
            println!("Missing {}", previous_value + 1);
        }
        previous_value = value;
    }
}
