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

fn contains_tree_sequential_letters(password: &str) -> bool {
    let mut sequencial_letters = 0;
    let mut chars = password.chars();
    let mut prev_char = chars.next().expect("There must be at least one char in a password");

    for char in chars {
        let (incremented, shift) = next_char(prev_char);

        if incremented == char && !shift {
            sequencial_letters += 1;

            if sequencial_letters >= 3 {
                return true;
            }
        } else {
            sequencial_letters = 0;
        }

        prev_char = char;
    }

    false
}

fn contains_ambiguous_letters(password: &str) -> bool {
    password.contains(['i', 'o', 'l'])
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

    #[test]
    fn correcly_detects_three_letters() {
        let data = vec![
            ("abcdefgh", true),
            ("abbccdde", false),
            ("edefgkor", true),
            ("abdeghkl", false),
        ];

        for (input, output) in data {
            assert_eq!(contains_tree_sequential_letters(input), output, "For \"{}\" it expected to be {}", input, output);
        }
    }

    #[test]
    fn detects_ambiguous_letters() {
        let data = vec![
            ("abcdefgh", false),
            ("ikjfythd", true),
            ("jklmnpdd", true),
            ("opqrstvw", true),
        ];

        for (input, output) in data {
            assert_eq!(contains_ambiguous_letters(input), output);
        }
    }
}
