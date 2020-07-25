use regex::Regex;
use std::collections::HashMap;
use crate::read_input::read_lines;

pub fn part_1() -> isize {
    let lines = read_lines(7);
    let circuit = parse_and_build(&lines);
    solve_circuit(&circuit)
}

pub fn part_2() -> isize {
    let lines = read_lines(7);
    let mut circuit = parse_and_build(&lines);
    let first_run = solve_circuit(&circuit);
    circuit.insert("b".into(), Operation::PassNum(first_run));
    solve_circuit(&circuit)
}

#[derive(PartialEq, Debug)]
enum Operation {
    Pass(String),
    PassNum(isize),
    Not(String),
    And(String, String),
    AndNum(isize, String),
    Or(String, String),
    Rshift(String, isize),
    Lshift(String, isize),
}

type Circuit = HashMap<String, Operation>;

fn parse_line(line: &str) -> (String, Operation) {
    let pass_line = Regex::new(r"^([a-z]+) -> ([a-z]+)$").unwrap();
    let pass_num_line = Regex::new(r"^(\d+) -> ([a-z]+)$").unwrap();
    let not_line = Regex::new(r"^NOT ([a-z]+) -> ([a-z]+)$").unwrap();
    let and_line = Regex::new(r"^([a-z]+) AND ([a-z]+) -> ([a-z]+)$").unwrap();
    let and_num_line = Regex::new(r"^(\d+) AND ([a-z]+) -> ([a-z]+)$").unwrap();
    let or_line = Regex::new(r"^([a-z]+) OR ([a-z]+) -> ([a-z]+)$").unwrap();
    let rshift_line = Regex::new(r"^([a-z]+) RSHIFT (\d+) -> ([a-z]+)$").unwrap();
    let lshift_line = Regex::new(r"^([a-z]+) LSHIFT (\d+) -> ([a-z]+)$").unwrap();

    if let Some(captures) = pass_line.captures(line) {
        return (captures[2].into(), Operation::Pass(captures[1].into()));
    }

    if let Some(captures) = pass_num_line.captures(line) {
        return (
            captures[2].into(),
            Operation::PassNum(captures[1].parse().unwrap()),
        );
    }

    if let Some(captures) = not_line.captures(line) {
        return (captures[2].into(), Operation::Not(captures[1].into()));
    }

    if let Some(captures) = and_line.captures(line) {
        return (
            captures[3].into(),
            Operation::And(captures[1].into(), captures[2].into()),
        );
    }

    if let Some(captures) = and_num_line.captures(line) {
        return (
            captures[3].into(),
            Operation::AndNum(captures[1].parse().unwrap(), captures[2].into()),
        );
    }

    if let Some(captures) = or_line.captures(line) {
        return (
            captures[3].into(),
            Operation::Or(captures[1].into(), captures[2].into()),
        );
    }

    if let Some(captures) = rshift_line.captures(line) {
        return (
            captures[3].into(),
            Operation::Rshift(captures[1].into(), captures[2].parse().unwrap()),
        );
    }

    if let Some(captures) = lshift_line.captures(line) {
        return (
            captures[3].into(),
            Operation::Lshift(captures[1].into(), captures[2].parse().unwrap()),
        );
    }

    unreachable!("Seems we've got wrong operations: {}", line);
}

fn parse_and_build(lines: &Vec<String>) -> Circuit {
    let ops = lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let mut circuit = HashMap::new();

    for (string, op) in ops {
        circuit.insert(string, op);
    }

    circuit
}

enum Point<'p> {
    Calc(&'p String),
    Calc2(&'p String, &'p String),
    Value(isize),
}

fn solve_circuit(circuit: &Circuit) -> isize {
    let end_point = "a".to_string();
    let mut to_calculate = vec![end_point.clone()];

    let mut cache: HashMap<String, isize> = HashMap::new();

    while !to_calculate.is_empty() {
        let key = to_calculate.pop().unwrap();
        match cache.get(&key) {
            None => {
                let op = circuit.get(&key).unwrap();

                let res = match op {
                    Operation::Pass(x) => cache
                        .get(x)
                        .map_or(Point::Calc(x), |val| Point::Value(*val)),
                    Operation::PassNum(num) => Point::Value(*num),
                    Operation::Not(x) => cache
                        .get(x)
                        .map_or(Point::Calc(x), |val| Point::Value(!val)),
                    Operation::And(x, y) => match (cache.get(x), cache.get(y)) {
                        (None, None) => Point::Calc2(x, y),
                        (None, Some(_)) => Point::Calc(x),
                        (Some(_), None) => Point::Calc(y),
                        (Some(x_value), Some(y_value)) => Point::Value(x_value & y_value),
                    },
                    Operation::AndNum(num, x) => cache
                        .get(x)
                        .map_or(Point::Calc(x), |val| Point::Value(num & val)),
                    Operation::Or(x, y) => match (cache.get(x), cache.get(y)) {
                        (None, None) => Point::Calc2(x, y),
                        (None, Some(_)) => Point::Calc(x),
                        (Some(_), None) => Point::Calc(y),
                        (Some(x_value), Some(y_value)) => Point::Value(x_value | y_value),
                    },
                    Operation::Rshift(x, num) => cache
                        .get(x)
                        .map_or(Point::Calc(x), |val| Point::Value(val >> num)),
                    Operation::Lshift(x, num) => cache
                        .get(x)
                        .map_or(Point::Calc(x), |val| Point::Value(val << num)),
                };

                match res {
                    Point::Calc(x) => {
                        to_calculate.push(key.clone());
                        to_calculate.push(x.clone());
                    }
                    Point::Calc2(x, y) => {
                        to_calculate.push(key.clone());
                        to_calculate.push(x.clone());
                        to_calculate.push(y.clone());
                    }
                    Point::Value(value) => {
                        cache.insert(key.clone(), value);
                    }
                }
            }
            Some(_) => {}
        }
    }

    *cache.get(&end_point).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pass_line() {
        assert_eq!(
            parse_line("lx -> a"),
            ("a".into(), Operation::Pass("lx".into()))
        );
    }

    #[test]
    fn parses_pass_num_line() {
        assert_eq!(
            parse_line("157 -> a"),
            ("a".into(), Operation::PassNum(157))
        );
    }

    #[test]
    fn parses_not_line() {
        assert_eq!(
            parse_line("NOT ax -> ay"),
            ("ay".into(), Operation::Not("ax".into()))
        );
    }

    #[test]
    fn parses_and_line() {
        assert_eq!(
            parse_line("lr AND lt -> lu"),
            ("lu".into(), Operation::And("lr".into(), "lt".into()))
        );
    }

    #[test]
    fn parses_or_line() {
        assert_eq!(
            parse_line("ci OR ct -> cu"),
            ("cu".into(), Operation::Or("ci".into(), "ct".into()))
        );
    }

    #[test]
    fn parses_rshift_line() {
        assert_eq!(
            parse_line("kk RSHIFT 1 -> ld"),
            ("ld".into(), Operation::Rshift("kk".into(), 1))
        );
    }

    #[test]
    fn parses_lshift_line() {
        assert_eq!(
            parse_line("lc LSHIFT 3 -> lw"),
            ("lw".into(), Operation::Lshift("lc".into(), 3))
        );
    }

    #[test]
    fn parses_and_with_number_line() {
        assert_eq!(
            parse_line("1 AND am -> an"),
            ("an".into(), Operation::AndNum(1, "am".into()))
        );
    }

    #[test]
    fn parses_and_build_circuit() {
        use Operation::*;

        let input: Vec<String> = vec![
            "12 -> d".into(),
            "d OR e -> b".into(),
            "3 -> e".into(),
            "e AND b -> a".into(),
        ];
        let mut expected: Circuit = HashMap::new();
        expected.insert("a".into(), And("e".into(), "b".into()));
        expected.insert("b".into(), Or("d".into(), "e".into()));
        expected.insert("d".into(), PassNum(12));
        expected.insert("e".into(), PassNum(3));
        assert_eq!(expected, parse_and_build(&input));
    }

    #[test]
    fn solves_sinplest_circuit() {
        let input = vec!["12 -> a".into()];
        let solved = solve_circuit(&parse_and_build(&input));
        assert_eq!(12, solved);
    }

    #[test]
    fn solves_3_state_circuit() {
        let input = vec!["e -> a".into(), "NOT d -> e".into(), "5 -> d".into()];
        let solved = solve_circuit(&parse_and_build(&input));
        assert_eq!(-6, solved);
    }

    #[test]
    fn solves_slightly_branched_citcuit() {
        let input: Vec<String> = vec![
            "12 -> d".into(),
            "d OR e -> b".into(),
            "56 -> e".into(),
            "NOT b -> a".into(),
        ];
        let solved = solve_circuit(&parse_and_build(&input));

        assert_eq!(-61, solved);
    }
}
