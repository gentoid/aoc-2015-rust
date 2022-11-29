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
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod read_input;
mod utils;

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
    println!("12 / 01: {}", day_12::part_1());
    println!("12 / 02: {}", day_12::part_2());
    println!("13 / 01: {}", day_13::part_1());
    println!("13 / 02: {}", day_13::part_2());
    println!("14 / 01: {}", day_14::part_1());
    println!("14 / 02: {}", day_14::part_2());
    println!("15 / 01: {}", day_15::part_1());
    println!("15 / 02: {}", day_15::part_2());
    println!("16 / 01: {}", day_16::part_1());
    println!("16 / 02: {}", day_16::part_2());
    println!("17 / 01: {}", day_17::part_1());
    println!("17 / 02: {}", day_17::part_2());
}
