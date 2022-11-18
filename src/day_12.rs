pub fn part_1() {
    //
}

fn get_numbers_from_json(input: &str) -> Vec<i32> {
    let mut is_inside_string = false;
    let mut number_value = String::new();
    let mut numbers = vec![];

    for char in input.chars() {
        if is_inside_string && char != '"' {
            continue;
        }

        if char == '"' {
            is_inside_string = !is_inside_string;
            continue;
        }

        if char == '-' && number_value.is_empty() {
            number_value.push(char);
        } else if char >= '0' && char <= '9' {
            number_value.push(char);
        } else if !number_value.is_empty() {
            numbers.push(number_value.parse().expect("It must be parsable"));
            number_value = String::new();
        }
    }

    return numbers;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_numbers_from_json() {
        let data = vec![
            ("[1,2,3]", vec![1, 2, 3]),
            (r#"{"a":2,"b":4}"#, vec![2, 4]),
            ("[[[3]]]", vec![3]),
            (r#"{"a":{"b":4},"c":-1}"#, vec![4, -1]),
            (r#"{"a":[-1,1]}"#, vec![-1, 1]),
            (r#"[-1,{"a":1}]"#, vec![-1, 1]),
            ("[]", vec![]),
            ("{}", vec![]),
        ];

        for (input, output) in data {
            assert_eq!(get_numbers_from_json(input), output);
        }
    }
}
