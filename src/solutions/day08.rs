use std::{collections::HashSet, iter::FromIterator};

use super::get_input;

#[derive(Default)]
struct Key {
    content: [HashSet<char>; 10]
}

impl Key {
    fn generate_key(s: &str) -> Key {
        let mut key = Key::default();
        let mut input: Vec<&str> = s.split(' ').collect();
        if input.len() != 10 {
            return key;
        }
        input.sort_by_key(|s| s.len());
        key.content[1] = HashSet::from_iter(input[0].chars());
        key.content[7] = HashSet::from_iter(input[1].chars());
        key.content[4] = HashSet::from_iter(input[2].chars());
        key.content[8] = HashSet::from_iter(input[9].chars());
        key.insert_0_6_9(input[6]);
        key.insert_0_6_9(input[7]);
        key.insert_0_6_9(input[8]);
        key.insert_2_3_5(input[3]);
        key.insert_2_3_5(input[4]);
        key.insert_2_3_5(input[5]);
        key
    }

    fn insert_0_6_9(&mut self, input: &str) {
        let new_symbol = HashSet::from_iter(input.chars());
        if new_symbol.is_superset(&self.content[4]) {
            self.content[9] = new_symbol;
        } else if new_symbol.is_superset(&self.content[1]) {
            self.content[0] = new_symbol;
        } else {
            self.content[6] = new_symbol;
        }
    }

    fn insert_2_3_5(&mut self, input: &str) {
        let new_symbol = HashSet::from_iter(input.chars());
        if new_symbol.is_subset(&self.content[6]) {
            self.content[5] = new_symbol;
        } else if new_symbol.is_superset(&self.content[1]) {
            self.content[3] = new_symbol;
        } else {
            self.content[2] = new_symbol;
        }
    }

    fn decode(&self, s: &str) -> u32 {
        let symbol = HashSet::from_iter(s.chars());
        self.content
            .iter()
            .position(|k| k == &symbol)
            .unwrap_or_default() as u32
    }
}

fn find_value(s: (&str, &str)) -> u32 {
    let key = Key::generate_key(s.0);
    s.1.split(' ')
        .fold(0, |value, s| value * 10 + key.decode(s))
}

pub fn day_eight(step: u8) -> u32 {
    let input = get_input("input/day08.txt");
    let len_step1 = HashSet::from([2, 3, 4, 7]);
    let data = input.lines().filter_map(|s| s.split_once(" | "));
    match step {
        1 => data
            .flat_map(|(_, s)| s.split(' '))
            .filter(|s| len_step1.contains(&s.len()))
            .count() as u32,
        2 => data.map(find_value).sum(),
        _ => 0,
    }
}
