use regex::Regex;
use std::collections::HashMap;

fn parse_line(line: &String) -> InstructionLine {
    let mut instruction = Instruction::Toggle;
    if line.starts_with("turn off") {
        instruction = Instruction::TurnOff;
    } else if line.starts_with("turn on") {
        instruction = Instruction::TurnOn;
    }

    let digits =
        Regex::new(r"([[:digit:]]+),([[:digit:]]+) through ([[:digit:]]+),([[:digit:]]+)$")
            .unwrap();

    let captures = digits.captures(line).unwrap();
    println!("{:#?}", captures);

    InstructionLine {
        instruction,
        from: (
            captures[1].parse::<u32>().unwrap(),
            captures[2].parse::<u32>().unwrap(),
        ),
        to: (
            captures[3].parse::<u32>().unwrap(),
            captures[4].parse::<u32>().unwrap(),
        ),
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(PartialEq, Debug)]
struct InstructionLine {
    instruction: Instruction,
    from: (u32, u32),
    to: (u32, u32),
}

struct LightsField {
    matrix: HashMap<(u32, u32), bool>,
}

impl LightsField {
    fn new() -> Self {
        LightsField {
            matrix: HashMap::new(),
        }
    }

    fn turn_on(&mut self, from: (u32, u32), to: (u32, u32)) {
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                let light = self.matrix.entry((x, y)).or_insert(true);
                *light = true;
            }
        }
    }

    fn turn_off(&mut self, from: (u32, u32), to: (u32, u32)) {
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                let light = self.matrix.entry((x, y)).or_insert(false);
                *light = false;
            }
        }
    }

    fn toggle(&mut self, from: (u32, u32), to: (u32, u32)) {
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                let light = self.matrix.entry((x, y)).or_insert(false);
                *light = !*light;
            }
        }
    }

    fn count(&self) -> usize {
        self.matrix
            .iter()
            .filter(|(_, &value)| value)
            .collect::<Vec<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_lights_after_init() {
        assert_eq!(LightsField::new().count(), 0)
    }

    #[test]
    fn turns_on_small_square() {
        let mut lights = LightsField::new();
        lights.turn_on((1, 1), (4, 4));
        assert_eq!(lights.count(), 16);
    }

    #[test]
    fn parses_turn_on_line() {
        let line = "turn on 123,65 through 43,92".to_string();
        assert_eq!(
            parse_line(&line),
            InstructionLine {
                instruction: Instruction::TurnOn,
                from: (123, 65),
                to: (43, 92),
            }
        );
    }

    #[test]
    fn parses_turn_off_line() {
        let line = "turn off 23,675 through 56,962".to_string();
        assert_eq!(
            parse_line(&line),
            InstructionLine {
                instruction: Instruction::TurnOff,
                from: (23, 675),
                to: (56, 962),
            }
        );
    }

    #[test]
    fn parses_toggle_line() {
        let line = "toggle 37,63 through 678,45".to_string();
        assert_eq!(
            parse_line(&line),
            InstructionLine {
                instruction: Instruction::Toggle,
                from: (37,63),
                to: (678,45),
            }
        );
    }
}
