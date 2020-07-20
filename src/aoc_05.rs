fn is_nice(input: &str) -> bool {
    let naugty_list = vec!["ab", "cd", "pq", "xy"];
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    
    for item in naugty_list {
        if input.contains(item) {
            return false;
        }
    }
    
    let mut num_of_vowels = 0;
    let mut has_double_letter = false;
    let mut prev_char = '_';

    for c in input.chars() {
        if vowels.contains(&c) {
            num_of_vowels += 1;
        }

        if prev_char == c {
            has_double_letter = true;
        }

        prev_char = c;
    }

    num_of_vowels >= 3 && has_double_letter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ugknbfddgicrmopn_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn aaa_is_nice() {
        assert!(is_nice("aaa"));
    }

    #[test]
    fn jchzalrnumimnmhp_is_not_nice() {
        assert!(!is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn haegwjzuvuyypxyu_is_not_nice() {
        assert!(!is_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn dvszwmarrgswjxmb_is_not_nice() {
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
