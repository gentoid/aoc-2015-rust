const INPUT: &str = "cqjxjnds";

pub fn part_1() -> String {
    "".to_owned()
}

fn increment(password: &str) -> String {
    let mut output = String::new();

    let mut increment = true;
    for char in password.chars().rev() {
        if increment {
            let next = next_char(char);
            increment = next.1;
            output.push(next.0);
        } else {
            output.push(char);
        }
    }

    output.chars().rev().collect()
}

fn next_char(c: char) -> (char, bool) {
    let shift = c >= 'z';

    if c < 'a' || shift {
        return ('a', shift)
    }

    (char::from_u32(c as u32 + 1).expect(&format!("Char must be incrementable: {c}")), shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_passwords() {
        let data = vec![
            ("aaaaaaaa", "aaaaaaab"),
            ("cdcorzzz", "cdcosaaa"),
            ("aaaaaaaz", "aaaaaaba"),
            ("azzzzzzz", "baaaaaaa"),
            ("zzzzzzzz", "aaaaaaaa"),
        ];

        for (input, output) in data {
            assert_eq!(&increment(input), output);
        }
    }
}
