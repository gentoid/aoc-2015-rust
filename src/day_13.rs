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
}
