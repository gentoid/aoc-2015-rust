use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::read_input::read_lines;

pub fn part_1() -> u32 {
    let known_aunt_sue = mfcsam_output();

    for aunt in all_aunts() {
        if known_aunt_sue.is_this_1(&aunt) {
            return aunt.number;
        }
    }

    unreachable!("No aunt found!");
}

fn all_aunts() -> Vec<Aunt> {
    read_lines(16).iter().map(parse_line).collect_vec()
}

struct Aunt {
    number: u32,
    properties: HashMap<String, u32>,
}

enum PropertyType {
    GreaterThan,
    FewerThan,
    Exactly,
}

struct KnownAunt {
    properties: HashMap<String, (u32, PropertyType)>,
}

impl KnownAunt {
    fn is_this_1(&self, aunt: &Aunt) -> bool {
        for (property, value) in aunt.properties.iter() {
            if let Some((self_value, _)) = self.properties.get(property.as_str()) {
                if self_value != value {
                    return false;
                }
            }
        }

        true
    }
}

fn mfcsam_output() -> KnownAunt {
    use PropertyType::*;

    let mut properties = HashMap::new();
    properties.insert("children".to_owned(), (3, Exactly));
    properties.insert("cats".to_owned(), (7, GreaterThan));
    properties.insert("samoyeds".to_owned(), (2, Exactly));
    properties.insert("pomeranians".to_owned(), (3, FewerThan));
    properties.insert("akitas".to_owned(), (0, Exactly));
    properties.insert("vizslas".to_owned(), (0, Exactly));
    properties.insert("goldfish".to_owned(), (5, FewerThan));
    properties.insert("trees".to_owned(), (3, GreaterThan));
    properties.insert("cars".to_owned(), (2, Exactly));
    properties.insert("perfumes".to_owned(), (1, Exactly));
    KnownAunt { properties }
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
