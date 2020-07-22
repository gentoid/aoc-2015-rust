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
    let mut pairs: HashMap<(char, char), Vec<usize>> = HashMap::new();
    let mut letters_cache: HashMap<char, usize> = HashMap::new();

    let mut prev_char = None;
    let mut contains_repeated_letter = false;

    for (index, c) in line.char_indices() {
        if !contains_repeated_letter {
            match letters_cache.get_mut(&c) {
                None => {
                    letters_cache.insert(c, index);
                }
                Some(cached_index) => {
                    if index - *cached_index == 2 {
                        contains_repeated_letter = true;
                    } else {
                        *cached_index = index;
                    }
                }
            }
        }

        if let Some(prev) = prev_char {
            let pair = (prev, c);
            match pairs.get_mut(&pair) {
                None => {
                    pairs.insert(pair, vec![index]);
                }
                Some(value) => value.push(index),
            }
        }
        prev_char = Some(c);
    }

    let contains_pair_twice = pairs.iter().any(|(_, indices)| {
        if indices.len() < 2 {
            return false;
        }
        let min = indices.iter().min();
        let max = indices.iter().max();

        if let Some(max) = max {
            if let Some(min) = min {
                return max - min >= 2;
            }
        }

        false
    });

    contains_pair_twice && contains_repeated_letter
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
}
