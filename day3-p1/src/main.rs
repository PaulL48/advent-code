use std::env;
use std::fs;

const TREE: char = '#';

fn trees_encountered(input: &str, horizontal_step: usize, vertical_step: usize) -> i32 {
    let mut x = 0;
    let mut trees = 0;
    for line in input.lines().step_by(vertical_step) {
        if line.chars().nth(x).unwrap() == TREE {
            trees += 1;
        }
        x += horizontal_step;
        x %= line.len();
    }

    return trees;
}

fn main() {
    // Forgot to separate this into p1 and p2 so this is the p2 solution that can easily be used for p1
    // ¯\_(ツ)_/¯

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("You gotta supply a file bro");

    let first_pass = trees_encountered(&contents, 1, 1);
    let second_pass = trees_encountered(&contents, 3, 1);
    let third_pass = trees_encountered(&contents, 5, 1);
    let fourth_pass = trees_encountered(&contents, 7, 1);
    let fifth_pass = trees_encountered(&contents, 1, 2);

    let result: i128 = i128::from(first_pass) * i128::from(second_pass) * i128::from(third_pass) * i128::from(fourth_pass) * i128::from(fifth_pass);

    println!("{} {} {} {} {}", first_pass, second_pass, third_pass, fourth_pass, fifth_pass);
    println!("product {}", result)
}
