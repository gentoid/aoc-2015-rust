use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::read_input::read_lines;

pub fn part_1() -> u32 {
    let (routes, places) = prepare_data(read_lines(9));

    find_shortest(&routes, &places)
}

fn without(list: &[String], element: &String) -> Vec<String> {
    list.iter()
        .cloned()
        .filter(|item| item != element)
        .collect()
}

fn concat(one: &[String], another: &[String]) -> Vec<String> {
    one.iter().cloned().chain(another.iter().cloned()).collect()
}

fn find_shortest(routes: &HashMap<(String, String), u32>, places: &[String]) -> u32 {
    *(find_all_lengths(routes, &places)
        .iter()
        .min()
        .expect("There must be minimum value"))
}

fn find_lengths_for_place(
    routes: &HashMap<(String, String), u32>,
    path: &[String],
    places: &[String],
    current: &String,
    lvl: usize,
) -> Vec<u32> {
    let mut lengths = vec![];

    // println!("{}Path: {:?}", "  ".repeat(lvl), path);
    // println!("{}Places: {:?}", "  ".repeat(lvl), places);
    // println!("{}Current: {}", "  ".repeat(lvl), current);

    let current = current.to_owned();

    for place in places.iter().cloned() {
        let path = concat(path, &[place.to_string()]);
        let filtered_out = &without(places, &place);

        let Some(length) = routes
            .get(&(current.clone(), place.clone())) else {
                continue;
            };

        if filtered_out.is_empty() {
            return vec![*length];
        }

        let found: Vec<u32> = find_lengths_for_place(routes, &path, filtered_out, &place, lvl + 1)
            .into_iter()
            .map(|found| found + length)
            .collect();

        lengths.extend(&found);
    }

    // println!("{}Lengths: {:?}", "  ".repeat(lvl), lengths);

    lengths.into_iter().unique().collect()
}

fn find_all_lengths(routes: &HashMap<(String, String), u32>, places: &[String]) -> Vec<u32> {
    let mut lengths = vec![];

    for place in places {
        let filtered_out = &without(places, &place);
        let found = find_lengths_for_place(&routes, &[place.to_owned()], &filtered_out, place, 0);

        lengths.extend(found);
    }

    lengths.into_iter().unique().collect()
}

fn prepare_data(lines: Vec<String>) -> (HashMap<(String, String), u32>, Vec<String>) {
    let mut routes = HashMap::new();
    let mut places = vec![];

    for line in lines {
        let (from, to, lenth) = parse_line(&line);
        routes.insert((from.clone(), to.clone()), lenth);
        routes.insert((to.clone(), from.clone()), lenth);
        places.push(from);
        places.push(to);
    }

    (routes, places.into_iter().unique().collect())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> Vec<String> {
        vec![
            "London to Dublin = 464".to_string(),
            "London to Belfast = 518".to_string(),
            "Dublin to Belfast = 141".to_string(),
        ]
    }

    #[test]
    fn data_is_prepared_correctly() {
        let actual = prepare_data(example_data());

        let mut combinations = HashMap::new();
        combinations.insert(("London".to_string(), "Dublin".to_string()), 464);
        combinations.insert(("Dublin".to_string(), "London".to_string()), 464);
        combinations.insert(("London".to_string(), "Belfast".to_string()), 518);
        combinations.insert(("Belfast".to_string(), "London".to_string()), 518);
        combinations.insert(("Dublin".to_string(), "Belfast".to_string()), 141);
        combinations.insert(("Belfast".to_string(), "Dublin".to_string()), 141);

        let mut places = vec![];
        places.push("London".to_string());
        places.push("Dublin".to_string());
        places.push("Belfast".to_string());

        assert_eq!(combinations, actual.0);
        assert_eq!(places, actual.1);
    }

    #[test]
    fn finds_shortest_path() {
        let data = prepare_data(example_data());
        let actual = find_shortest(&data.0, &data.1);

        assert_eq!(605, actual);
    }
}
