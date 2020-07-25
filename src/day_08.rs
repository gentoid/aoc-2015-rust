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
        assert_eq!((6, 1), count_chars(line))
    }

    #[test]
    fn counts_double_slashes() {
        let line = r#""\\tr""#;
        assert_eq!((6, 3), count_chars(line))
    }
}
