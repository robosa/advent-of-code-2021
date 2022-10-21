use std::iter::FromIterator;

use itertools::Itertools;

use super::get_input;

struct Grid {
    rows: [u8; 5],
    columns: [u8; 5],
    content: Vec<u8>,
}

impl<'a> FromIterator<&'a str> for Grid {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Grid {
            rows: [0; 5],
            columns: [0; 5],
            content: iter
                .into_iter()
                .flat_map(|row| row.split(' ').filter_map(|value| value.parse().ok()))
                .take(25)
                .collect(),
        }
    }
}

impl Grid {
    fn check_number(&mut self, number: u8) -> bool {
        if let Some(p) = self.content.iter().position(|x| *x == number) {
            let (i, j) = (p / 5, p % 5);
            self.content[p] = 0;
            self.rows[i] += 1;
            self.columns[j] += 1;
            self.rows[i] == 5 || self.columns[j] == 5
        } else {
            false
        }
    }

    fn sum(&self) -> u32 {
        self.content.iter().map(|x| *x as u32).sum()
    }

    fn play(&mut self, numbers: &Vec<u8>) -> Option<(usize, u32)> {
        if let Some((turns, last_value)) = numbers
            .iter()
            .enumerate()
            .find(|(_, n)| self.check_number(**n))
        {
            Some((turns, self.sum() * *last_value as u32))
        } else {
            None
        }
    }
}

fn get_score(scores: Vec<(usize, u32)>, turns_compare: fn(&usize, &usize) -> bool) -> u32 {
    scores
        .into_iter()
        .reduce(|(turns, score), (new_turns, new_score)| {
            if turns_compare(&new_turns, &turns) {
                (new_turns, new_score)
            } else {
                (turns, score)
            }
        })
        .unwrap_or_default()
        .1
}

pub fn day_four(step: u8) -> u32 {
    let input = get_input("input/day04.txt");
    let mut line_iter = input.lines();

    let numbers = line_iter
        .next()
        .unwrap_or_default()
        .split(',')
        .filter_map(|i| i.parse().ok())
        .collect();

    let scores = line_iter
        .chunks(6)
        .into_iter()
        .filter_map(|chunk| chunk.skip(1).collect::<Grid>().play(&numbers))
        .collect();

    match step {
        1 => get_score(scores, usize::lt),
        2 => get_score(scores, usize::gt),
        _ => 0,
    }
}
