mod aoc_01;
mod aoc_02;
mod aoc_03;
mod aoc_04;
mod aoc_05;
mod aoc_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod read_input;

fn main() {
    println!("01 / 01: {}", aoc_01::aoc_01_01());
    println!("01 / 02: {:?}", aoc_01::aoc_01_02());
    println!("02 / 01: {}", aoc_02::aoc_02_01());
    println!("02 / 02: {}", aoc_02::aoc_02_02());
    println!("03 / 01: {}", aoc_03::aoc_03_01());
    println!("03 / 02: {}", aoc_03::aoc_03_02());
    println!("04 / 01: {}", aoc_04::aoc_04_01());
    println!("04 / 02: {}", aoc_04::aoc_04_02());
    println!("05 / 01: {}", aoc_05::aoc_05_01());
    println!("05 / 02: {}", aoc_05::aoc_05_02());
    println!("06 / 01: {}", aoc_06::aoc_06_01());
    println!("06 / 02: {}", aoc_06::aoc_06_02());
    println!("07 / 01: {}", day_07::part_1());
    println!("07 / 02: {}", day_07::part_2());
    println!("08 / 01: {}", day_08::part_1());
    println!("08 / 02: {}", day_08::part_2());
    println!("09 / 01: {}", day_09::part_1());
    println!("09 / 02: {}", day_09::part_2());
    println!("10 / 01: {}", day_10::part_1());
    println!("10 / 02: {}", day_10::part_2());
    println!("11 / 01: {}", day_11::part_1());
    println!("11 / 02: {}", day_11::part_2());
}
