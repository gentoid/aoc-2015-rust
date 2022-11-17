const INPUT: &str = "1321131112";

pub fn part_1() -> usize {
    do_n_times(INPUT, 40)
}

pub fn part_2() -> usize {
    do_n_times(INPUT, 50)
}

fn do_n_times(input: &str, n: usize) -> usize {
    let mut data = input.to_owned();

    for _ in 1..=n {
        data = look_and_say(&data);
    }

    data.len()
}

fn look_and_say(input: &str) -> String {
    let mut output = String::new();

    let mut chars = input.chars();

    let mut prev_char = chars.next().expect("There must be a char");
    let mut repetitions = 1;

    for char in chars {
        if char == prev_char {
            repetitions += 1;
        } else {
            output.push_str(&format!("{repetitions}{prev_char}"));

            prev_char = char;
            repetitions = 1;
        }
    }

    output.push_str(&format!("{repetitions}{prev_char}"));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let data = vec![
            ("1", "11"),
            ("11", "21"),
            ("21", "1211"),
            ("1211", "111221"),
            ("111221", "312211"),
        ];

        for (input, ouput) in data {
            assert_eq!(look_and_say(input), ouput.to_owned());
        }
    }
}
