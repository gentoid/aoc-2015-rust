use crate::read_input::read_lines;
use std::collections::HashSet;

pub fn aoc_03_01() -> usize {
    let lines = read_lines(3);
    houses(&lines[0])
}

pub fn aoc_03_02() -> usize {
    let lines = read_lines(3);
    houses_with_robo(&lines[0])
}

fn houses(input: &str) -> usize {
    uniq_houses(input).len()
}

fn houses_with_robo(input: &str) -> usize {
    let (input_santa, input_robo) = unzip_input(input);

    uniq_houses(&input_santa)
        .union(&uniq_houses(&input_robo))
        .collect::<HashSet<&(i32, i32)>>()
        .len()
}

fn unzip_input(input: &str) -> (String, String) {
    let mut input_santa = "".to_string();
    let mut input_robo = "".to_string();

    for (index, c) in input.char_indices() {
        if index % 2 == 0 {
            input_santa.push(c);
        } else {
            input_robo.push(c);
        }
    }

    (input_santa, input_robo)
}

fn uniq_houses(input: &str) -> HashSet<(i32, i32)> {
    let mut houses = HashSet::new();
    let mut house = (0, 0);
    houses.insert(house.clone());

    for c in input.chars() {
        house = calculate_next_house(house, &c);
        houses.insert(house.clone());
    }

    houses
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

    #[test]
    fn three_houses_with_robo() {
        assert_eq!(3, houses_with_robo("^v"));
    }

    #[test]
    fn three_houses_and_back_with_robo() {
        assert_eq!(3, houses_with_robo("^>v<"));
    }

    #[test]
    fn eleven_houses_with_robo() {
        assert_eq!(11, houses_with_robo("^v^v^v^v^v"));
    }
}
