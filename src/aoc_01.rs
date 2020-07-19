use crate::read_input::read_lines;

pub fn aoc_01_01() -> i32 {
    let lines = read_lines(1);
    calculate_floor(&lines[0])
}

pub fn aoc_01_02() -> Option<u32> {
    let lines = read_lines(1);
    first_time_enter_basement(&lines[0])
}

fn calculate_floor(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        floor += calculate_next_floor(&c);
    }

    floor
}

fn first_time_enter_basement(input: &str) -> Option<u32> {
    let mut floor = 0;
    for (index, c) in input.char_indices() {
        floor += calculate_next_floor(&c);
        if floor == -1 {
            return Some((index + 1) as _)
        }
    }

    None
}

fn calculate_next_floor(c: &char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0
    }
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

    #[test]
    fn enter_the_basement_case_1() {
        assert_eq!(Some(1), first_time_enter_basement(")"));
    }

    #[test]
    fn enter_the_basement_case_2() {
        assert_eq!(Some(5), first_time_enter_basement("()())"));
    }
}
