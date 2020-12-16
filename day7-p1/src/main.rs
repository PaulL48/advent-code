#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;
use petgraph::graphmap::DiGraphMap;
use petgraph::Outgoing;
use petgraph::visit::{Reversed, Dfs};
use regex::Regex;

const TARGET_BAG: &str = "shiny gold";

fn get_bounded_substring<'a>(text: &'a str, start_pattern: &Regex, end_pattern: &Regex) -> &'a str {
    lazy_static! {
        static ref BEGINNING_OF_TEXT: Regex = Regex::new(r"^").unwrap();
        static ref END_OF_TEXT: Regex = Regex::new(r"$").unwrap();
    }

    let start_byte = start_pattern.find(text).unwrap_or(BEGINNING_OF_TEXT.find(text).unwrap()).end();
    let end_byte = end_pattern.find(text).unwrap_or(END_OF_TEXT.find(text).unwrap()).start();
    
    &text[start_byte..end_byte]
}

// Return the color of bag this line is prescribing rules for
fn get_subject_color(line: &str) -> Option<&str> {
    Some(line.split(" bag").next()?)
}

// Return a vector of colors that this line's bag can contain
fn get_allowed_colors(line: &str) -> Vec<&str> {
    lazy_static! {
        static ref START_RE: Regex = Regex::new(r"\d{1} ").unwrap();
        static ref END_RE: Regex = Regex::new(r" bag").unwrap();
    }

    let mut result = vec![];
    let contains_rules = line.split("contain ").nth(1).expect("Nothing follows token 'contains'").split(",");
    for contains_rule in contains_rules {
        result.push(get_bounded_substring(contains_rule, &START_RE, &END_RE));
    }
    result
}

fn construct_graph<'a>(graph: &mut DiGraphMap<&'a str, ()>, rules: &'a str) {
    for line in rules.lines() {
        if let Some(color) = get_subject_color(line) {
            graph.add_node(color);
            for allowed_color in get_allowed_colors(line) {
                graph.add_node(allowed_color);
                graph.add_edge(color, allowed_color, ());
            }
        } else {
            println!("Error processing line, skipping: {}", line)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args.get(1).expect("file path expected as the first parameter")).expect("Failed to read file");
    let mut graph = DiGraphMap::<&str, ()>::new();
    construct_graph(&mut graph, contents.as_str());

    // Erase outgoing edges from target bag
    let neighbors: Vec<&str> = graph.neighbors_directed(TARGET_BAG, Outgoing).collect();
    for neighbor in neighbors {
        graph.remove_edge(TARGET_BAG, neighbor);
    }

    // Traverse starting from target node and count visited nodes
    let mut dfs = Dfs::new(Reversed(&graph), TARGET_BAG);
    let mut count = 0;
    while let Some(_) = dfs.next(Reversed(&graph)) {
        count += 1;
    }

    println!("{}", count - 1);
}
