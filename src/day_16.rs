use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::read_input::read_lines;

pub fn part_1() -> u32 {
    let all_aunts = read_lines(16).iter().map(parse_line).collect_vec();
    let known_aunt_sue = mfcsam_output();

    for aunt in all_aunts {
        if known_aunt_sue.is_this(&aunt) {
            return aunt.number;
        }
    }

    unreachable!("No aunt found!");
}

#[derive(Default)]
struct Aunt {
    number: u32,
    properties: HashMap<String, u32>,
}

impl Aunt {
    fn is_this(&self, other: &Self) -> bool {
        for (property, value) in other.properties.iter() {
            if self.properties.get(property.as_str()) != Some(&value) {
                return false
            }
        }

        true
    }
}

fn mfcsam_output() -> Aunt {
    let mut properties = HashMap::new();
    properties.insert("children".to_owned(), 3);
    properties.insert("cats".to_owned(), 7);
    properties.insert("samoyeds".to_owned(), 2);
    properties.insert("pomeranians".to_owned(), 3);
    properties.insert("akitas".to_owned(), 0);
    properties.insert("vizslas".to_owned(), 0);
    properties.insert("goldfish".to_owned(), 5);
    properties.insert("trees".to_owned(), 3);
    properties.insert("cars".to_owned(), 2);
    properties.insert("perfumes".to_owned(), 1);
    Aunt {
        number: 0,
        properties,
    }
}

fn parse_line(line: &String) -> Aunt {
    let temlate = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    let captures = temlate.captures(line).unwrap();

    let mut aunt = Aunt {
        number: captures[1].parse::<u32>().unwrap(),
        properties: HashMap::with_capacity(3),
    };

    for index in [2, 4, 6] {
        let property = captures[index].to_owned();
        let value = captures[index + 1].parse::<u32>().unwrap();
        aunt.properties.insert(property, value);
    }

    aunt
}
