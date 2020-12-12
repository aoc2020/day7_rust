use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::{Regex, Match};
use std::fmt;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

const SHINY_GOLD: &str = "shiny gold";

fn lookup(color: &String,
          checked: &mut HashMap<String, bool>,
          bag_map: &HashMap<String, Bag>) -> bool {
    if color == SHINY_GOLD {
        return true;
    }
    if checked.contains_key(&color.clone()) {
        checked[&color.clone()]
    } else {
        let bag: &Bag = &bag_map[&color.clone()];
        let found = bag.contains.iter()
            .find(|bag| {
                let color = &bag.clone().1;
                lookup(&color, checked, bag_map)
            })
            .is_some();
        checked.insert(color.clone(), found);
        found
    }
}

fn solve_task1(bags: Vec<Bag>, bag_map: &HashMap<String, Bag>) {
    let mut checked: HashMap<String, bool> = HashMap::new();

    let answer: Vec<bool> = bags.iter()
        .map(|bag|
            lookup(&bag.color, &mut checked, &bag_map))
        .filter(|x| x.clone())
        .collect();
    println!("Answer 1: {}", answer.len() - 1); // removing 1 for the shiny bag itself
}

fn count_bags(color: &String, counts: &mut HashMap<String, i32>, bag_map: &HashMap<String, Bag>) -> i32 {
    if counts.contains_key(&color.clone()) {
        counts[color].clone()
    } else {
        let bags = bag_map[&color.clone()].clone().contains;
        let content : i32 = bags.into_iter().map(|dep| {
            let n = dep.0;
            let color = dep.1.clone();
            n * count_bags(&dep.1.clone(), counts, &bag_map)
        }).sum();
        content + 1
    }
}

fn solve_task2(bags: Vec<Bag>, bag_map: &HashMap<String, Bag>) {
    let mut counts: HashMap<String, i32> = HashMap::new();
    let num = count_bags(&String::from(SHINY_GOLD), &mut counts, bag_map);
    println!("Answer 2: {}", num-1); // removing 1 for the shiny bag
}

fn to_map(bags: &Vec<Bag>) -> HashMap<String, Bag> {
    let mut bag_map: HashMap<String, Bag> = HashMap::new();
    for bag in bags {
        bag_map.insert(bag.color.clone(), bag.clone());
    }
    bag_map
}

fn main() {
    let lines = read_file("input.txt");
    let bags: Vec<Bag> = lines.iter()
        .map(|l| Bag::parse(l))
        .collect();
    let bag_map = to_map(&bags);

    solve_task1(bags.clone(), &bag_map);
    solve_task2(bags.clone(), &bag_map);
}

fn re(s: &str) -> Regex {
    Regex::new(s)
        .expect(format!("Failed to compile regex: {}", s).as_str())
}

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    contains: Vec<(i32, String)>,
}

impl Bag {
    fn new(color: String, contains: Vec<(i32, String)>) -> Bag {
        Bag { color, contains }
    }
    fn parse(line: &String) -> Bag {
        lazy_static! {
            static ref LINE_RE : Regex = re("(^[a-z ]+) bags? contain (.*)");
            static ref BAG_RE : Regex = re("([0-9]+) ([a-z ]*) bag");
        }
        let outer: Vec<&str> = (*LINE_RE).captures(line)
            .expect(format!("Failed to parse {}", line).as_str())
            .iter().map(|s| s.unwrap().as_str())
            .collect();
        let color: String = String::from(outer[1]);
        let bags: Vec<(i32, String)> = (*BAG_RE).captures_iter(outer[2])
            .map(|cap| {
                let num = cap[1].parse().unwrap();
                let color: String = String::from(&cap[2]);
                (num, color)
            })
            .collect();
        Bag { color, contains: bags }
    }
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bag({} : {:?})", self.color, self.contains)
    }
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect(format!("File not found: {}", filename).as_str());
    let reader = BufReader::new(file);
    reader.lines().map(|s| s.expect("Could not collect line")).collect()
}