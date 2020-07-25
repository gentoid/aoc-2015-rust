use crate::read_input::read_lines;

pub fn part_1() -> usize {
    let lines = read_lines(8);
    lines
        .iter()
        .map(|line| {
            let counts = count_chars(line);
            counts.0 - counts.1
        })
        .sum()
}

pub fn part_2() -> usize {
    let lines = read_lines(8);
    lines
        .iter()
        .map(|line| {
            let counts = count_chars_encoded(line);
            counts.1 - counts.0
        })
        .sum()
}

fn count_chars(line: &str) -> (usize, usize) {
    let escaped = line.len();
    let mut cleaned_up = 0;
    let mut escape = false;

    for c in line.chars() {
        if c == '\\' && !escape {
            escape = true;
            continue;
        }

        cleaned_up += 1;

        if escape {
            escape = false;
            if c == 'x' {
                cleaned_up -= 2;
            }
        }
    }
    (escaped, cleaned_up - 2)
}

fn count_chars_encoded(line: &str) -> (usize, usize) {
    let escaped = line.len();
    let mut encoded = 0;

    for c in line.chars() {
        if c == '"' || c == '\\' {
            encoded += 1;
        }
        encoded += 1;
    }

    (escaped, encoded + 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_empty_line() {
        let line = r#""""#;
        assert_eq!((2, 0), count_chars(line));
    }

    #[test]
    fn counts_aaa_with_quote_escape() {
        let line = r#""aaa\"aaa""#;
        assert_eq!((10, 7), count_chars(line));
    }

    #[test]
    fn counts_hex_notation() {
        let line = r#""\x27""#;
        assert_eq!((6, 1), count_chars(line));
    }

    #[test]
    fn counts_double_slashes() {
        let line = r#""\\tr""#;
        assert_eq!((6, 3), count_chars(line));
    }

    #[test]
    fn double_encoded_empty_line() {
        let line = r#""""#;
        assert_eq!((2, 6), count_chars_encoded(line));
    }

    #[test]
    fn double_encoded_aaa_with_quote() {
        let line = r#""aaa\"aaa""#;
        assert_eq!((10, 16), count_chars_encoded(line));
    }

    #[test]
    fn double_encoded_hex_value() {
        let line = r#""\x27""#;
        assert_eq!((6, 11), count_chars_encoded(line));
    }
}
