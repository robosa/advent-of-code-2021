use itertools::Itertools;

use super::get_input;

pub fn day_one(step: u8) -> usize {
    let input = get_input("input/day1.txt");
    let data = input.lines().filter_map(|s| s.parse::<u32>().ok());
    match step {
        1 => data.tuple_windows().filter(|(a, b)| b > a).count(),
        2 => data.tuple_windows().filter(|(a, _, _, b)| b > a).count(),
        _ => 0,
    }
}
