const INPUT: &str = "cqjxjnds";

pub fn part_1() -> String {
    "".to_owned()
}

fn next_allowed(password: &str) -> String {
    let mut next_password = password.to_owned();

    while !is_allowed(&next_password) {
        next_password = increment(&next_password);
    }

    next_password
}

fn is_allowed(password: &str) -> bool {
    password.len() == 8
        && contains_tree_sequential_letters(&password)
        && !contains_ambiguous_letters(&password)
        && contains_two_pairs_of_same_letters(&password)
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
    let mut sequencial_letters = 1;
    let mut chars = password.chars();
    let mut prev_char = chars
        .next()
        .expect("There must be at least one char in a password");

    for char in chars {
        let (incremented, shift) = next_char(prev_char);

        if incremented == char && !shift {
            sequencial_letters += 1;

            if sequencial_letters >= 3 {
                return true;
            }
        } else {
            sequencial_letters = 1;
        }

        prev_char = char;
    }

    sequencial_letters >= 3
}

fn contains_ambiguous_letters(password: &str) -> bool {
    password.contains(['i', 'o', 'l'])
}

fn contains_two_pairs_of_same_letters(password: &str) -> bool {
    let mut pairs = 0;
    let mut prev_char = None;

    for char in password.chars() {
        if let Some(prev_char_value) = prev_char {
            if prev_char_value == char {
                pairs += 1;

                if pairs >= 2 {
                    return true;
                }

                prev_char = None;
                continue;
            }
        };

        prev_char = Some(char);
    }

    pairs >= 2
}

fn next_char(c: char) -> (char, bool) {
    let shift = c >= 'z';

    if c < 'a' || shift {
        return ('a', shift);
    }

    (
        char::from_u32(c as u32 + 1).expect(&format!("Char must be incrementable: {c}")),
        shift,
    )
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
            assert_eq!(
                contains_tree_sequential_letters(input),
                output,
                "For \"{}\" it expected to be {}",
                input,
                output
            );
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

    #[test]
    fn detects_two_pairs_of_same_letters() {
        let data = vec![
            ("aaacvdsr", false),
            ("aaacvddd", true),
            ("aabberwe", true),
            ("erffddef", true),
        ];

        for (input, output) in data {
            assert_eq!(contains_two_pairs_of_same_letters(input), output);
        }
    }

    #[test]
    fn failed_password() {
        let data = vec!["hijklmmn", "abbceffg", "abbcegjk", "abcdefgh", "ghijklmn"];

        for password in data {
            assert!(!is_allowed(password));
        }
    }

    #[test]
    fn allowed_password() {
        let data = vec!["abcdffaa", "ghjaabcc"];

        for password in data {
            assert!(contains_tree_sequential_letters(password), "{} contais 3 sequential letters", password);
            assert!(!contains_ambiguous_letters(password), "{} does not contain ambiguous letters", password);
            assert!(contains_two_pairs_of_same_letters(password), "{} contains 2 pairs of same letters", password);
            assert!(is_allowed(password), "{} should be allowed", password);
        }
    }

    #[test]
    fn finds_next_allowed() {
        assert_eq!(next_allowed("abcdefgh"), "abcdffaa");
        assert_eq!(next_allowed("ghijklmn"), "ghjaabcc");
    }
}
