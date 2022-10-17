use std::collections::VecDeque;

use super::get_input;

pub fn day_six(step: u8) -> u64 {
    let input = get_input("input/day6.txt");
    let last_day: u16 = if step == 2 { 256 } else { 80 };
    let mut initial_state = [0; 9];
    input
        .split(",")
        .filter_map(|s| s.parse::<usize>().ok())
        .for_each(|i| initial_state[i] += 1);
    let mut lanternfishes = VecDeque::from(initial_state);
    for _ in 0..last_day {
        let births = lanternfishes.pop_front().unwrap_or_default();
        lanternfishes[6] += births;
        lanternfishes.push_back(births);
    }
    lanternfishes.iter().sum()
}
