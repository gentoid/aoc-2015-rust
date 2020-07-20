use md5;

fn input() -> String {
    "bgvyzdsv".to_string()
}

pub fn aoc_04_01() -> u32 {
    mine(&input(), "00000")
}

pub fn aoc_04_02() -> u32 {
    mine(&input(), "000000")
}

fn mine(input: &str, start: &str) -> u32 {
    let mut number = 0;
    
    loop {
        if starts_with(input, number, start) {
            return number;
        }
        number += 1;
    }
}

fn starts_with(input: &str, number: u32, start: &str) -> bool {
    let digest = md5::compute(format!("{}{}", input, number));
    let stringified = format!("{:x}", digest);
    stringified.starts_with(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mine_coinn_for_abcdef() {
        assert_eq!(609043, mine("abcdef", "00000"));
    }

    #[test]
    fn mine_coinn_for_pqrstuv() {
        assert_eq!(1048970, mine("pqrstuv", "00000"));
    }
}
