use std::collections::HashMap;
use std::env;
use std::fs;

fn get_key_value_pairs(string: &str) -> HashMap<&str, &str> {
    let mut result = HashMap::new();
    for pair in string
        .split(&[' ', '\n'][..])
        .filter(|pair| !pair.is_empty())
    {
        let tokens: Vec<&str> = pair.split(':').collect();
        result.insert(tokens[0], tokens[1]);
    }
    return result;
}

fn passport_is_valid(passport: HashMap<&str, &str>) -> bool {
    return passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("You gotta supply a file bro");

    let mut valid_passports = 0;
    for passport_text in contents.split("\n\n") {
        println!("{}", passport_text);
        let passport = get_key_value_pairs(passport_text);
        if passport_is_valid(passport) {
            valid_passports += 1;
        } else {
            println!("Invalid password");
        }
    }

    println!("Number of valid passports is {}", valid_passports);
}
