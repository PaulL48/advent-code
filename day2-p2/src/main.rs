use std::env;
use std::fs;

fn password_is_valid(line: &str) -> bool {
    let tokens: Vec<&str> = line
        .split(&['-', ' ', ':'][..])
        .filter(|token| !token.is_empty())
        .collect();
    let (first_position, second_position, character, password) = (
        tokens[0].parse::<usize>().unwrap(),
        tokens[1].parse::<usize>().unwrap(),
        &tokens[2].chars().next().unwrap(),
        tokens[3],
    );
    let in_first_pos = password.chars().nth(first_position - 1).unwrap() == *character;
    let in_second_pos = password.chars().nth(second_position - 1).unwrap() == *character;
    println!("{} {}", in_first_pos, in_second_pos);

    // XOR operation
    return (in_first_pos || in_second_pos) && !(in_first_pos && in_second_pos);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("You gotta supply a file bro");

    let mut good_passwords = 0;
    for line in contents.lines() {
        if password_is_valid(line) {
            good_passwords += 1;
            
        } else {
            println!("{}", line);
        }
    }

    println!("Good passwords: {}", good_passwords);
}
