use std::collections::HashMap;

use regex::Regex;

use crate::utils::without;

type HappinessOptions = HashMap<(String, String), i32>;
type Seating = (i32, String, i32);

fn parse_line(input: &str) -> (String, String, i32) {
    let temlate =
        Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)\.").unwrap();
    let captures = temlate.captures(input).unwrap();

    let sign = match &captures[2] {
        "gain" => 1,
        "lose" => -1,
        _ => unreachable!(),
    };

    (
        captures[1].to_owned(),
        captures[4].to_owned(),
        sign * captures[3].parse::<i32>().unwrap(),
    )
}

fn find_combinations(options: &HappinessOptions, guests: &[String]) -> Vec<Vec<Seating>> {
    let mut output = vec![];

    let first_guest = guests.first().unwrap();
    let filtered = without(guests, first_guest);

    if filtered.len() == 1 {
        let mut tmp = vec![];

        let last_guest = guests.last().unwrap();

        let happiness = options
            .get(&(first_guest.clone(), last_guest.clone()))
            .unwrap();
        let reverse_happiness = options
            .get(&(last_guest.clone(), first_guest.clone()))
            .unwrap();

        tmp.push((*happiness, first_guest.clone(), *happiness));
        tmp.push((*reverse_happiness, last_guest.clone(), *reverse_happiness));
        output.push(tmp);

        let mut tmp = vec![];
        tmp.push((*reverse_happiness, last_guest.clone(), *reverse_happiness));
        tmp.push((*happiness, first_guest.clone(), *happiness));
        output.push(tmp);
    } else {
        for mut inner in find_combinations(options, &filtered) {
            let happiness = options
                .get(&(first_guest.clone(), inner.last().unwrap().1.clone()))
                .unwrap();
            let inner_first = inner.first().unwrap();
            let inner_last = inner.last().unwrap();

            let mut tmp = vec![(*happiness, first_guest.clone(), *happiness)];
            tmp.extend(inner);
            output.push(tmp);
        }
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lines() {
        let data = vec![
            (
                "Alice would gain 54 happiness units by sitting next to Bob.",
                ("Alice", "Bob", 54),
            ),
            (
                "Alice would lose 79 happiness units by sitting next to Carol.",
                ("Alice", "Carol", -79),
            ),
            (
                "Bob would gain 83 happiness units by sitting next to Alice.",
                ("Bob", "Alice", 83),
            ),
        ];

        for (input, (person1, person2, happiness)) in data {
            assert_eq!(
                parse_line(input),
                (person1.to_owned(), person2.to_owned(), happiness)
            );
        }
    }

    fn options() -> HashMap<(String, String), i32> {
        let mut options = HashMap::new();

        options.insert(("Alice".to_owned(), "Bob".to_owned()), 57);
        options.insert(("Alice".to_owned(), "Clark".to_owned()), 4);
        options.insert(("Bob".to_owned(), "Alice".to_owned()), 10);
        options.insert(("Bob".to_owned(), "Clark".to_owned()), -1);
        options.insert(("Clark".to_owned(), "Alice".to_owned()), 6);
        options.insert(("Clark".to_owned(), "Bob".to_owned()), 3);

        options
    }

    fn guests(length: usize) -> Vec<String> {
        vec!["Alice".to_owned(), "Bob".to_owned(), "Clark".to_owned()].into_iter().take(length).collect()
    }

    #[test]
    fn finds_seating_for_two_persons() {
        let expected = vec![
            vec![(57, "Alice".to_owned(), 57), (10, "Bob".to_owned(), 10)],
            vec![(10, "Bob".to_owned(), 10), (57, "Alice".to_owned(), 57)],
        ];

        assert_eq!(find_combinations(&options(), &guests(2)), expected);
    }

    #[test]
    fn finds_seating_for_three_persons() {
        let expected = vec![
            vec![
                (4, "Alice".to_owned(), 57),
                (10, "Bob".to_owned(), -1),
                (3, "Clark".to_owned(), 6),
            ],
            vec![
                (57, "Alice".to_owned(), 4),
                (6, "Clark".to_owned(), 3),
                (-1, "Bob".to_owned(), 10),
            ],
        ];

        assert_eq!(find_combinations(&options(), &guests(3)), expected);
    }
}
