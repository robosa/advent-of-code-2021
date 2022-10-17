use std::collections::HashSet;

use super::get_input;

#[derive(Default)]
struct Key {
    content: [String; 10],
}

impl Key {
    fn generate_key(s: &str) -> Key {
        let mut key = Key::default();
        let mut input: Vec<&str> = s.split(' ').collect();
        if input.len() != 10 {
            return key
        }
        input.sort_by_key(|s| s.len());
        key.insert_sorted_key(input[0], 1);
        key.insert_sorted_key(input[1], 7);
        key.insert_sorted_key(input[2], 4);
        key.insert_sorted_key(input[9], 8);
        key.insert_0_6_9(input[6]);
        key.insert_0_6_9(input[7]);
        key.insert_0_6_9(input[8]);
        key.insert_2_3_5(input[3]);
        key.insert_2_3_5(input[4]);
        key.insert_2_3_5(input[5]);
        key
    }
    
    fn insert_sorted_key(&mut self, input: &str, index: usize) {
        let mut chars: Vec<char> = input.chars().collect();
        chars.sort_unstable();
        self.content[index] = chars.iter().collect();
    }

    fn insert_0_6_9(&mut self, input: &str) {
        if str_included(&self.content[4], input) {
            self.insert_sorted_key(input, 9);
        } else if str_included(&self.content[1], input) {
            self.insert_sorted_key(input, 0)
        } else {
            self.insert_sorted_key(input, 6)
        }
    }

    fn insert_2_3_5(&mut self, input: &str) {
        if str_included(input, &self.content[6]) {
            self.insert_sorted_key(input, 5);
        } else if str_included(&self.content[1], input) {
            self.insert_sorted_key(input, 3);
        } else {
            self.insert_sorted_key(input, 2);
        }
    }

    fn decode(&self, s: &str) -> u32 {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let sorted_string: String = chars.iter().collect();
        self.content.iter()
            .position(|k| k == &sorted_string)
            .unwrap_or_default() as u32
    }
}

fn str_included(s1: &str, s2: &str) -> bool {
    match s1.chars().into_iter().find(|c| !s2.contains(*c)) {
        Some(_) => false,
        None => true,
    }
}

fn find_value(s: (&str, &str)) -> u32 {
    let key = Key::generate_key(s.0);
    s.1.split(' ')
        .fold(0, |value, s| value * 10 + key.decode(s))
}

pub fn day_eight(step: u8) -> u32 {
    let input = get_input("input/day8.txt");
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
