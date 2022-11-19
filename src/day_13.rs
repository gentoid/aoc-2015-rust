use regex::Regex;

type Happiness = (String, String, i32);
type Seating = (i32, String, i32);

fn parse_line(input: &str) -> Happiness {
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

fn find_combinations(data: &Vec<Happiness>) -> Vec<Vec<Seating>> {
    let mut output = vec![];

    let first = data.first().unwrap();
    let (filtered_first, filtered_second, filtered_out) = split_by_person(&first.0, data);

    if filtered_out.is_empty() {
        let last = data.get(1).unwrap();
        let mut tmp = vec![];
        tmp.push((last.2, last.0.clone(), last.2));
        tmp.push((first.2, first.0.clone(), first.2));
        output.push(tmp);

        let mut tmp = vec![];
        tmp.push((first.2, first.0.clone(), first.2));
        tmp.push((last.2, last.0.clone(), last.2));
        output.push(tmp);
    } else {
        for mut inner in find_combinations(&filtered_out) {
            let inner_first = inner.first().unwrap();
            let inner_last = inner.last().unwrap();

            let mut tmp = vec![(first.2, first.0.clone(), first.2)];
            tmp.extend(inner);
            output.push(tmp);
        }
    }

    return output;
}

fn split_by_person(
    name: &str,
    list: &Vec<Happiness>,
) -> (Vec<Happiness>, Vec<Happiness>, Vec<Happiness>) {
    let mut filtered_first = vec![];
    let mut filtered_second = vec![];
    let mut filtered_out = vec![];

    for item in list.iter().cloned() {
        if item.0 == name {
            filtered_first.push(item);
        } else if item.1 == name {
            filtered_second.push(item);
        } else {
            filtered_out.push(item);
        }
    }

    (filtered_first, filtered_second, filtered_out)
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

    #[test]
    fn finds_seating_for_two_persons() {
        let data = vec![
            ("Alice".to_owned(), "Bob".to_owned(), 57),
            ("Bob".to_owned(), "Alice".to_owned(), 10),
        ];

        let expected = vec![
            vec![(10, "Bob".to_owned(), 10), (57, "Alice".to_owned(), 57)],
            vec![(57, "Alice".to_owned(), 57), (10, "Bob".to_owned(), 10)],
        ];

        assert_eq!(find_combinations(&data), expected);
    }

    #[test]
    fn finds_seating_for_three_persons() {
        let data = vec![
            ("Alice".to_owned(), "Bob".to_owned(), 57),
            ("Alice".to_owned(), "Clark".to_owned(), 4),
            ("Bob".to_owned(), "Alice".to_owned(), 10),
            ("Bob".to_owned(), "Clark".to_owned(), -1),
            ("Clark".to_owned(), "Alice".to_owned(), 6),
            ("Clark".to_owned(), "Bob".to_owned(), 3),
        ];

        let expected = vec![
            vec![
                (57, "Alice".to_owned(), 4),
                (6, "Clark".to_owned(), 3),
                (-1, "Bob".to_owned(), 10),
            ],
            vec![
                (4, "Alice".to_owned(), 57),
                (10, "Bob".to_owned(), -1),
                (3, "Clark".to_owned(), 6),
            ],
        ];

        assert_eq!(find_combinations(&data), expected);
    }
}
