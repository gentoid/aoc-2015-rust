use crate::read_input::read_lines;
use std::collections::HashMap;

pub fn aoc_05_01() -> u32 {
    let mut num_of_nice_lines = 0;
    for line in read_lines(5) {
        if is_nice(&line) {
            num_of_nice_lines += 1;
        }
    }

    num_of_nice_lines
}

pub fn aoc_05_02() -> u32 {
    let mut num_of_nice_lines = 0;
    for line in read_lines(5) {
        if is_really_nice(&line) {
            num_of_nice_lines += 1;
        }
    }

    num_of_nice_lines
}

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

fn is_really_nice(line: &str) -> bool {
    contains_pair_twice(line) && contains_repeated_letters(line)
}

fn contains_repeated_letters(line: &str) -> bool {
    let mut letters = HashMap::new();

    line.char_indices().any(|(index, c)| {
        match letters.get_mut(&c) {
            None => {
                letters.insert(c, vec![index]);
            }
            Some(ids) => {
                if ids.iter().any(|id| index - id == 2) {
                    return true;
                }

                ids.push(index);
            }
        }

        return false
    })
}

fn contains_pair_twice(line: &str) -> bool {
    let mut pairs = HashMap::new();
    let mut prev_char = None;
    
    line.char_indices().any(|(index, c)| {
        if let Some(prev) = prev_char {
            let pair = (prev, c);
            match pairs.get(&pair) {
                None => {
                    pairs.insert(pair, index);
                }
                Some(id) => {
                    if index - id >= 2 {
                        return true;
                    }
                }
            }
        }

        prev_char = Some(c);
        false
    })
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

    #[test]
    fn qjhvhtzxzqqjkmpb_is_really_nice() {
        assert!(is_really_nice("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn xxyxx_is_really_nice() {
        assert!(is_really_nice("xxyxx"));
    }

    #[test]
    fn uurcxstgmygtbstg_is_really_not_nice() {
        assert!(!is_really_nice("uurcxstgmygtbstg"));
    }

    #[test]
    fn ieodomkazucvgmuy_is_really_not_nice() {
        assert!(!is_really_nice("ieodomkazucvgmuy"));
    }

    #[test]
    fn aaa_does_not_contain_pairs() {
        assert!(!contains_pair_twice("aaa"));
    }

    #[test]
    fn xyxy_contains_pairs() {
        assert!(contains_pair_twice("xyxy"));
    }

    #[test]
    fn aabcdefgaa_contains_pairs() {
        assert!(contains_pair_twice("aabcdefgaa"));
    }

    #[test]
    fn aaa_contains_repeated_letters() {
        assert!(contains_repeated_letters("aaa"));
    }
}
