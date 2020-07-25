use regex::Regex;
use std::collections::HashMap;

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
}
