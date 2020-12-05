use std::collections::HashMap;
use std::env;
use std::fs;

fn character_counts(string: &str) -> HashMap<char, i32> {
    let mut character_counts = HashMap::new();
    for c in string.chars() {
        let entry = character_counts.entry(c).or_insert(0);
        *entry += 1;
    }
    return character_counts;
}

fn is_password_valid(line: &str) -> bool {
    let tokens: Vec<&str> = line
        .split(&['-', ' ', ':'][..])
        .filter(|token| !token.is_empty())
        .collect();
    let (min, max, character, password) = (
        tokens[0].parse::<i32>().unwrap(),
        tokens[1].parse::<i32>().unwrap(),
        &tokens[2].chars().next().unwrap(),
        tokens[3],
    );
    let character_counts = character_counts(password);
    println!(
        "min {}, max {}, char {}, pwd {}, valid: {}",
        min,
        max,
        character,
        password,
        *character_counts.get(&character).unwrap_or(&0) >= min
            && *character_counts.get(&character).unwrap_or(&0) <= max
    );
    return min <= *character_counts.get(&character).unwrap_or(&0)
        && *character_counts.get(&character).unwrap_or(&0) <= max;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("You gotta supply a file bro");

    let mut bad_passwords = 0;
    for line in contents.lines() {
        if !is_password_valid(line) {
            bad_passwords += 1;
        }
    }

    println!("The number of bad passwords is {}", bad_passwords);
}
