use super::get_input;

fn step_1(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort_unstable();
    let median = numbers[numbers.len() / 2];
    numbers.iter().map(|n| n.abs_diff(median)).sum()
}

fn step_2(numbers: Vec<u32>) -> u32 {
    let mean = numbers.iter().sum::<u32>() / numbers.len() as u32;
    numbers
        .iter()
        .map(|n| {
            let diff = n.abs_diff(mean);
            diff * (diff + 1) / 2
        })
        .sum()
}

pub fn day_seven(step: u8) -> u32 {
    let input = get_input("input/day07.txt");
    let mut numbers: Vec<u32> = input.split(',').filter_map(|n| n.parse().ok()).collect();
    match step {
        1 => step_1(&mut numbers),
        2 => step_2(numbers),
        _ => 0
    }
}
