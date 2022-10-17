mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub use day1::day_one;
pub use day2::day_two;
pub use day3::day_three;
pub use day4::day_four;
pub use day5::day_five;
pub use day6::day_six;
pub use day7::day_seven;
pub use day8::day_eight;

use std::fs::read_to_string;

pub fn get_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap_or_default()
}
