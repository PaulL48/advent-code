use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::FromStr;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

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

#[derive(Debug)]
struct Passport {
    pub byr: i32,
    pub iyr: i32,
    pub eyr: i32,
    pub hgt: (i32, String),
    pub hcl: String,
    pub ecl: String,
    pub pid: String
}

fn has_passport_keys(passport: &HashMap<&str, &str>) -> bool {
    return passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid");
}

// Parse string data into any appropriate types
fn parse_passport(key_value_pairs: &HashMap<&str, &str>) -> Result<Passport, <i32 as FromStr>::Err> {
    Ok(Passport {
        byr: key_value_pairs.get("byr").unwrap().parse()?,
        iyr: key_value_pairs.get("iyr").unwrap().parse()?,
        eyr: key_value_pairs.get("eyr").unwrap().parse()?,
        hgt: (key_value_pairs.get("hgt").unwrap().chars().take_while(|c| c.is_numeric()).collect::<String>().parse()?, key_value_pairs.get("hgt").unwrap().chars().skip_while(|c| c.is_numeric()).collect()),
        hcl: String::from(*key_value_pairs.get("hcl").unwrap()),
        ecl: String::from(*key_value_pairs.get("ecl").unwrap()),
        pid: String::from(*key_value_pairs.get("pid").unwrap())
    })
}

fn passport_is_valid(passport: &Passport) -> bool {
    let mut valid = true;

    // Validate byr: Parse and range
    valid &= 1920 <= passport.byr && passport.byr <= 2002;
    // println!("byr: {} {}", passport.byr, 1920 <= passport.byr && passport.byr <= 2002);

    // Validate iyr: Parse and range
    valid &= 2010 <= passport.iyr && passport.iyr <= 2020;
    // println!("iyr: {} {}", passport.iyr, 2010 <= passport.iyr && passport.iyr <= 2020);
    
    // Validate eyr: Parse and range
    valid &= 2020 <= passport.eyr && passport.eyr <= 2030;
    // println!("eyr: {} {}", passport.eyr, 2020 <= passport.eyr && passport.eyr <= 2030);

    // hgt: Split, find units, parse, range
    
    valid &= passport.hgt.1 == "cm" || passport.hgt.1 == "in";
    if passport.hgt.1 == "cm" {
        valid &= 150 <= passport.hgt.0 && passport.hgt.0 <= 193;
        // println!("hgt: {:?} {}", passport.hgt, 150 <= passport.hgt.0 && passport.hgt.0 <= 193);
    } else if passport.hgt.1 == "in" {
        valid &= 59 <= passport.hgt.0 && passport.hgt.0 <= 76;
        // println!("hgt: {:?} {}", passport.hgt, 59 <= passport.hgt.0 && passport.hgt.0 <= 76);
    } else {
        // println!("hgt: {:?} {}", passport.hgt, false);
    }
    

    // hcl: regex match
    lazy_static! {
        static ref COLOR_RE: Regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
    }
    valid &= COLOR_RE.is_match(&passport.hcl);
    // println!("hcl: {} {}", passport.hcl, COLOR_RE.is_match(&passport.hcl));

    // ecl: set match
    let options: HashSet<&str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
    valid &= options.contains(passport.ecl.as_str());
    // println!("ecl: {} {}", passport.ecl, options.contains(passport.ecl.as_str()));

    // pid: regex
    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    valid &= PID_RE.is_match(&passport.pid);
    // println!("pid: {} {}", passport.pid, PID_RE.is_match(&passport.pid));
    // println!("");

    return valid;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("You gotta supply a file bro");

    let mut valid_passports = 0;
    for passport_text in contents.split("\n\n") {
        let passport_kvs = get_key_value_pairs(passport_text);
        if has_passport_keys(&passport_kvs) {
            if let Ok(passport) = parse_passport(&passport_kvs) {
                if passport_is_valid(&passport) {
                    valid_passports += 1;
                    println!("{:?}", passport);
                }
            }
        }
    }

    println!("Number of valid passports is {}", valid_passports);
}
