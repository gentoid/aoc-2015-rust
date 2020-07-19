use crate::read_input::read_lines;

pub fn aoc_01_01() -> i32 {
    let lines  = read_lines(1);
    calculate_floor(&lines[0])
}

fn calculate_floor(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c== ')' {
            floor -= 1;
        }
    }

    return floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_0_case_1() {
        assert_eq!(0, calculate_floor("(())"));
    }

    #[test]
    fn floor_0_case_2() {
        assert_eq!(0, calculate_floor("()()"));
    }

    #[test]
    fn floor_3_case_1() {
        assert_eq!(3, calculate_floor("((("));
    }

    #[test]
    fn floor_3_case_2() {
        assert_eq!(3, calculate_floor("(()(()("));
    }

    #[test]
    fn floor_3_case_3() {
        assert_eq!(3, calculate_floor("))((((("));
    }

    #[test]
    fn floor_minus_1_case_1() {
        assert_eq!(-1, calculate_floor("())"));
    }

    #[test]
    fn floor_minus_1_case_2() {
        assert_eq!(-1, calculate_floor("))("));
    }

    #[test]
    fn floor_minus_3_case_1() {
        assert_eq!(-3, calculate_floor(")))"));
    }

    #[test]
    fn floor_minus_3_case_2() {
        assert_eq!(-3, calculate_floor(")())())"));
    }
}
