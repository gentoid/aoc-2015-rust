use std::collections::HashSet;
use crate::read_input::read_lines;

pub fn aoc_03_01() -> usize {
    let lines = read_lines(3);
    houses(&lines[0])
}

fn houses(input: &str) -> usize {
    let mut houses = HashSet::new();
    let mut house = (0, 0);
    houses.insert(house.clone());    

    for c in input.chars() {
        house = calculate_next_house(house, &c);
        houses.insert(house.clone());
    }

    houses.len()
}

fn calculate_next_house(house: (i32, i32), c: &char) -> (i32, i32) {
    match c {
        '>' => (house.0 + 1, house.1),
        'v' | 'V' => (house.0, house.1 - 1),
        '<' => (house.0 - 1, house.1),
        '^' => (house.0, house.1 + 1),
        _ => house,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_houses() {
        assert_eq!(2, houses(">"));
    }

    #[test]
    fn four_houses_in_square() {
        assert_eq!(4, houses("^>v<"));
    }

    #[test]
    fn two_houses_multiple_times() {
        assert_eq!(2, houses("^v^v^v^v^v"));
    }
}
