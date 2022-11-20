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

fn full_combination(guests: &[String]) -> Vec<Vec<String>> {
    match guests {
        [] => vec![],
        [guest] => vec![vec![guest.clone()]],
        _ => {
            let mut output = vec![];

            for guest in guests {
                output.extend(combinations_with_first(&without(guests, guest), guest));
            }

            output
        }
    }
}

fn find_combinations(guests: &[String]) -> Vec<Vec<String>> {
    match guests {
        [] | [_] => vec![],
        _ => {
            let first = guests.first().unwrap();
            let filtered = without(guests, &first);

            combinations_with_first(&filtered, &first)
        }
    }
}

fn combinations_with_first(guests: &[String], first_guest: &str) -> Vec<Vec<String>> {
    let mut output = vec![];

    for mut combination in full_combination(guests) {
        combination.insert(0, first_guest.to_string());
        output.push(combination);
    }

    output
}

fn fill_in_happiness(
    options: &HappinessOptions,
    combinations: &Vec<Vec<String>>,
) -> Vec<Vec<Seating>> {
    let mut output = vec![];

    for combination in combinations {
        let mut inner = vec![];

        for (index, guest) in combination.iter().enumerate() {
            let left_guest = if index == 0 {
                combination.last()
            } else {
                combination.get(index - 1)
            }
            .unwrap()
            .clone();

            let right_guest = if index < combination.len() - 1 {
                combination.get(index + 1)
            } else {
                combination.first()
            }
            .unwrap()
            .clone();

            inner.push((
                *options.get(&(guest.clone(), left_guest)).unwrap(),
                guest.clone(),
                *options.get(&(guest.clone(), right_guest)).unwrap(),
            ));
        }

        output.push(inner);
    }

    output
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
        options.insert(("Alice".to_owned(), "Dan".to_owned()), 9);
        options.insert(("Bob".to_owned(), "Alice".to_owned()), 10);
        options.insert(("Bob".to_owned(), "Clark".to_owned()), -1);
        options.insert(("Bob".to_owned(), "Dan".to_owned()), -3);
        options.insert(("Clark".to_owned(), "Alice".to_owned()), 6);
        options.insert(("Clark".to_owned(), "Bob".to_owned()), 3);
        options.insert(("Clark".to_owned(), "Dan".to_owned()), 12);
        options.insert(("Dan".to_owned(), "Alice".to_owned()), -12);
        options.insert(("Dan".to_owned(), "Bob".to_owned()), 32);
        options.insert(("Dan".to_owned(), "Clark".to_owned()), 7);

        options
    }

    fn guests(length: usize) -> Vec<String> {
        vec![
            "Alice".to_owned(),
            "Bob".to_owned(),
            "Clark".to_owned(),
            "Dan".to_owned(),
        ]
        .into_iter()
        .take(length)
        .collect()
    }

    #[test]
    fn finds_combinations_for_two_persons() {
        let expected = vec![vec!["Alice".to_owned(), "Bob".to_owned()]];

        assert_eq!(find_combinations(&guests(2)), expected);
    }

    #[test]
    fn finds_combinations_for_three_persons() {
        let expected = vec![
            vec!["Alice".to_owned(), "Bob".to_owned(), "Clark".to_owned()],
            vec!["Alice".to_owned(), "Clark".to_owned(), "Bob".to_owned()],
        ];

        assert_eq!(find_combinations(&guests(3)), expected);
    }

    #[test]
    fn finds_combinations_for_four_persons() {
        let expected = vec![
            vec![
                "Alice".to_owned(),
                "Bob".to_owned(),
                "Clark".to_owned(),
                "Dan".to_owned(),
            ],
            vec![
                "Alice".to_owned(),
                "Bob".to_owned(),
                "Dan".to_owned(),
                "Clark".to_owned(),
            ],
            vec![
                "Alice".to_owned(),
                "Clark".to_owned(),
                "Bob".to_owned(),
                "Dan".to_owned(),
            ],
            vec![
                "Alice".to_owned(),
                "Clark".to_owned(),
                "Dan".to_owned(),
                "Bob".to_owned(),
            ],
            vec![
                "Alice".to_owned(),
                "Dan".to_owned(),
                "Bob".to_owned(),
                "Clark".to_owned(),
            ],
            vec![
                "Alice".to_owned(),
                "Dan".to_owned(),
                "Clark".to_owned(),
                "Bob".to_owned(),
            ],
        ];

        assert_eq!(find_combinations(&guests(4)), expected);
    }

    #[test]
    fn fills_in_happiness_for_2_guests() {
        let expected = vec![vec![
            (57, "Alice".to_owned(), 57),
            (10, "Bob".to_owned(), 10),
        ]];

        let actual = fill_in_happiness(&options(), &find_combinations(&guests(2)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fills_in_happiness_for_3_guests() {
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

        let actual = fill_in_happiness(&options(), &find_combinations(&guests(3)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn fills_in_happiness_for_4_guests() {
        let expected = vec![
            vec![
                (9, "Alice".to_owned(), 57),
                (10, "Bob".to_owned(), -1),
                (3, "Clark".to_owned(), 12),
                (7, "Dan".to_owned(), -12),
            ],
            vec![
                (4, "Alice".to_owned(), 57),
                (10, "Bob".to_owned(), -3),
                (32, "Dan".to_owned(), 7),
                (12, "Clark".to_owned(), 6),
            ],
            vec![
                (9, "Alice".to_owned(), 4),
                (6, "Clark".to_owned(), 3),
                (-1, "Bob".to_owned(), -3),
                (32, "Dan".to_owned(), -12),
            ],
            vec![
                (57, "Alice".to_owned(), 4),
                (6, "Clark".to_owned(), 12),
                (7, "Dan".to_owned(), 32),
                (-3, "Bob".to_owned(), 10),
            ],
            vec![
                (4, "Alice".to_owned(), 9),
                (-12, "Dan".to_owned(), 32),
                (-3, "Bob".to_owned(), -1),
                (3, "Clark".to_owned(), 6),
            ],
            vec![
                (57, "Alice".to_owned(), 9),
                (-12, "Dan".to_owned(), 7),
                (12, "Clark".to_owned(), 3),
                (-1, "Bob".to_owned(), 10),
            ],
        ];

        let actual = fill_in_happiness(&options(), &find_combinations(&guests(4)));

        assert_eq!(expected, actual);
    }
}
