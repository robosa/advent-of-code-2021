use std::convert::TryInto;

use super::get_input;

fn str_to_vec(string: &str) -> Option<Vec<u32>> {
    match string.len() {
        12 => string.chars().map(|i| i.to_digit(2)).collect(),
        _ => None,
    }
}

fn count_nth_bit<'a>(data: impl Iterator<Item = &'a Vec<u32>>, i: usize) -> (u32, u32) {
    data.fold((0, 0), |(nb0, nb1), v| {
        if v[i] == 0 {
            (nb0 + 1, nb1)
        } else {
            (nb0, nb1 + 1)
        }
    })
}

fn most_common(nb0: u32, nb1: u32) -> u32 {
    match (nb0, nb1) {
        (0, _) => 1,
        (_, 0) => 0,
        (a, b) if a > b => 0,
        _ => 1,
    }
}

fn least_common(nb0: u32, nb1: u32) -> u32 {
    match (nb0, nb1) {
        (0, _) => 1,
        (_, 0) => 0,
        (a, b) if a > b => 1,
        _ => 0,
    }
}

fn get_mask(data: &Vec<Vec<u32>>, selector: fn(u32, u32) -> u32) -> Vec<bool> {
    let nb_lines = data.len();
    let mut mask = vec![true; nb_lines];
    for i in 0..12 {
        let (nb0, nb1) = count_nth_bit(
            mask
                .iter()
                .zip(data.iter())
                .filter(|(b, _)| **b)
                .map(|(_, v)| v),
            i,
        );
        let selected = selector(nb0, nb1);
        for j in 0..nb_lines {
            mask[j] = mask[j] && data[j][i] == selected;
        }
        if nb0 + nb1 == 1 {
            break;
        }
    }
    mask
}

fn step1(data: Vec<Vec<u32>>) -> u32 {
    let half_nb_of_lines = data.len() / 2;
    let (gamma, epsilon) = data
        .iter()
        .fold(vec![0; 12], |mut v, s| {
            s.iter().enumerate().for_each(|(i, b)| v[i] += b);
            v
        })
        .iter()
        .fold((0, 0), |(gamma, epsilon), i| {
            if *i > half_nb_of_lines.try_into().unwrap_or(0) {
                (gamma * 2 + 1, epsilon * 2)
            } else {
                (gamma * 2, epsilon * 2 + 1)
            }
        });
    gamma * epsilon
}

fn step2(data: Vec<Vec<u32>>) -> u32 {
    let oxygen_index = get_mask(&data, most_common)
        .iter()
        .position(|&x| x)
        .unwrap_or(0);
    let co2_index = get_mask(&data, least_common)
        .iter()
        .position(|&x| x)
        .unwrap_or(0);
    let oxygen = data[oxygen_index].iter().fold(0, |v, b| v * 2 + b);
    let co2 = data[co2_index].iter().fold(0, |v, b| v * 2 + b);
    oxygen * co2
}

pub fn day_three(step: u8) -> u32 {
    let input = get_input("input/day03.txt");
    let data: Vec<Vec<u32>> = input.lines().filter_map(str_to_vec).collect();
    match step {
        1 => step1(data),
        2 => step2(data),
        _ => 0,
    }
}
