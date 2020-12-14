use std::env;
use std::fs;

const ROW_UPPER: char = 'B';
const ROW_LOWER: char = 'F';

const COL_UPPER: char = 'R';
const COL_LOWER: char = 'L';

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
    seat_id(
        binary_space_partition(0, 127, ROW_UPPER, ROW_LOWER, &sequence[..7]),
        binary_space_partition(0, 7, COL_UPPER, COL_LOWER, &sequence[7..]),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("You gotta supply a file bro");
    let max = contents
        .split("\n")
        .map(|x| seat_id_from_sequence(x))
        .max()
        .unwrap();
    println!("{}", max);
}
