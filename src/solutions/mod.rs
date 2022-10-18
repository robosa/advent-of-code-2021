mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

pub use day01::day_one;
pub use day02::day_two;
pub use day03::day_three;
pub use day04::day_four;
pub use day05::day_five;
pub use day06::day_six;
pub use day07::day_seven;
pub use day08::day_eight;
pub use day09::day_nine;

use std::fs::read_to_string;

pub fn get_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap_or_default()
}
