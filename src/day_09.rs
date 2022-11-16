use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::read_input::read_lines;

pub fn part_1() -> u32 {
    let (routes, places) = prepare_data(read_lines(9));

    let mut lengths = HashSet::new();

    for from in &places {
        let nested_lengths = find_lengths_for_place(from.to_string(), &places, &routes, 0);
        println!("For [ {} ]: {:?}", from, nested_lengths);
        lengths.extend(nested_lengths);
    }

    return *(lengths.iter().min().expect("There must be minimum value"));
}

fn prepare_data(lines: Vec<String>) -> (HashMap<(String, String), u32>, HashSet<String>) {
    let mut routes = HashMap::new();
    let mut places = HashSet::new();

    for line in lines {
        let (from, to, lenth) = parse_line(&line);
        routes.insert((from.clone(), to.clone()), lenth);
        routes.insert((to, from.clone()), lenth);
        places.insert(from);
    }

    (routes, places)
}

fn parse_line(line: &String) -> (String, String, u32) {
    let route: Regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let captures = route
        .captures(line)
        .expect("Line should be in the defined format.");

    (
        captures[1].to_owned(),
        captures[2].to_owned(),
        captures[3].parse().expect("Cannot parse capture as u32"),
    )
}

fn find_lengths_for_place(
    from: String,
    places: &HashSet<String>,
    routes: &HashMap<(String, String), u32>,
    nest_level: usize,
) -> HashSet<u32> {
    let mut lengths = HashSet::new();

    let places: HashSet<String> = places
        .into_iter()
        // .map(|place| *place)
        .filter(|place| **place != from)
        .map(|place| place.clone())
        .collect();

    if places.is_empty() {
        return lengths;
    }

    // println!("{}From: {}", " ".repeat(nest_level), from);
    // println!("{}Places: {:?}", " ".repeat(nest_level), places);

    if places.len() == 1 {
        lengths.insert(0);
        // println!("{}Lengths: {:?}", " ".repeat(nest_level), lengths);
        return lengths;
    }

    for place in &places {
        let length = routes
            .get(&(from.clone(), place.to_string()))
            .expect("There must be such a combination of places");
        let nested_lengths = places
            .iter()
            .flat_map(|inner_place| {
                find_lengths_for_place(inner_place.to_string(), &places, routes, nest_level + 1)
            })
            .map(|inner_length| inner_length + length);
        lengths.extend(nested_lengths);
    }

    // println!("{}Lengths: {:?}", " ".repeat(nest_level), lengths);

    return lengths;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = r#"London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141"#;

        // TODO: write more test cases
    }
}
