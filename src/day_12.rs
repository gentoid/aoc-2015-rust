use crate::read_input::read_input_to_string;

pub fn part_1() -> i32 {
    get_numbers_from_json(&read_input_to_string(12))
        .iter()
        .sum()
}

pub fn part_2() -> i32 {
    let (_, output) = get_numbers_without_red(&read_input_to_string(12), NestedType::Root);
    output.iter().sum()
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

#[derive(PartialEq)]
enum NestedType {
    Array,
    Object,
    Root,
}

fn get_numbers_without_red(input: &str, nested_type: NestedType) -> (String, Vec<i32>) {
    let mut tmp_string = input.to_owned();
    let mut chars = tmp_string.chars();
    let mut numbers = vec![];
    let mut number_value = String::new();
    let mut string_value = String::new();
    let mut is_inside_string = false;
    let mut contains_red = false;

    while let Some(char) = chars.next() {
        if is_inside_string && char != '"' {
            string_value.push(char);
            continue;
        }

        match char {
            '"' => {
                if is_inside_string {
                    contains_red = contains_red || string_value == "red";
                    string_value = String::new();
                }

                is_inside_string = !is_inside_string;
            }
            '-' => {
                if number_value.is_empty() {
                    number_value.push(char);
                }
            }
            '0'..='9' => number_value.push(char),
            '[' => {
                let (left_string, inner_numbers) =
                    get_numbers_without_red(chars.as_str(), NestedType::Array);
                tmp_string = left_string;
                chars = tmp_string.chars();
                numbers.extend(inner_numbers);
            }
            ']' => {
                assert!(
                    nested_type == NestedType::Array,
                    "Not expected closing bracket."
                );
                break;
            }
            '{' => {
                let (left_string, inner_numbers) =
                    get_numbers_without_red(chars.as_str(), NestedType::Object);
                tmp_string = left_string;
                chars = tmp_string.chars();
                numbers.extend(inner_numbers);
            }
            '}' => {
                assert!(
                    nested_type == NestedType::Object,
                    "Not expected closing bracket."
                );
                break;
            }
            ',' => {
                if !number_value.is_empty() {
                    numbers.push(number_value.parse::<i32>().unwrap());

                    number_value = String::new();
                }
            }
            _ => continue,
        }
    }

    if !number_value.is_empty() {
        numbers.push(number_value.parse::<i32>().unwrap());
    }

    let numbers = if nested_type == NestedType::Object && contains_red {
        vec![]
    } else {
        numbers
    };

    (chars.as_str().to_owned(), numbers)
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

    #[test]
    fn extract_numbers_without_red() {
        let data = vec![
            ("[1,2,3]", vec![1, 2, 3]),
            (r#"[1,{"c":"red","b":2},3]"#, vec![1, 3]),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, vec![]),
            (r#"[1,"red",5]"#, vec![1, 5]),
        ];

        for (input, expected) in data {
            let (_, actual) = get_numbers_without_red(input, NestedType::Root);
            assert_eq!(actual, expected);
        }
    }
}
