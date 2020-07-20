use md5;

fn input() -> String {
    "bgvyzdsv".to_string()
}

pub fn aoc_04_01() -> u32 {
    mine(&input())
}

fn mine(input: &str) -> u32 {
    let mut number = 0;
    
    loop {
        if starts_with_5_zeros(input, number) {
            return number;
        }
        number += 1;
    }
}

fn starts_with_5_zeros(input: &str, number: u32) -> bool {
    let digest = md5::compute(format!("{}{}", input, number));
    let stringified = format!("{:x}", digest);
    stringified.starts_with("00000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mine_coinn_for_abcdef() {
        assert_eq!(609043, mine("abcdef"));
    }

    #[test]
    fn mine_coinn_for_pqrstuv() {
        assert_eq!(1048970, mine("pqrstuv"));
    }
}
