use std::collections::HashSet;
use std::env;
use std::fs;

fn to_set(sequence: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    sequence.chars().fold(&mut set, |acc, x| {
        acc.insert(x);
        acc
    });
    set
}

fn in_place_intersect<T>(set1: &mut HashSet<T>, set2: &HashSet<T>)
where
    T: std::cmp::Eq + std::hash::Hash,
{
    set1.retain(|k| set2.contains(k));
}

fn count_unique_characters(group: &str) -> i32 {
    let mut iter = group.split("\n");
    let mut questions = to_set(iter.next().expect("Group has size 0"));
    iter.fold(&mut questions, |acc, x| {
        in_place_intersect(acc, &to_set(x));
        acc
    });
    questions.len() as i32
}

fn main() {
    // Forgot to duplicate code for day 6 part 2 so this is day 6 part 2
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("You gotta supply a file bro");

    let total = contents
        .split("\n\n")
        .fold(0, |acc, x| acc + count_unique_characters(x));

    println!("{}", total);
}
