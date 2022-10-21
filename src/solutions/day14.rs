use std::collections::HashMap;

use itertools::Itertools;

use super::get_input;

#[derive(Default)]
struct Polymer {
    rules: HashMap<(char, char), char>,
    pair_count: HashMap<(char, char), u64>,
    elem_count: HashMap<char, u64>,
}

impl From<String> for Polymer {
    fn from(input: String) -> Self {
        let mut polymer = Polymer::default();
        let mut iter = input.lines();
        let template: &str = iter.next().unwrap_or_default();
        iter.next();

        polymer.init_rules(iter);
        polymer.init_pair_count(template);
        polymer.init_elem_count(template);
        polymer
    }
}

impl Polymer {
    fn init_rules<'a, I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        self.rules = iter
            .into_iter()
            .filter_map(|s| {
                s.split_once(" -> ")
                    .and_then(|(s1, s2)| s1.chars().next_tuple().zip(s2.chars().next()))
            })
            .collect();
    }

    fn init_pair_count(&mut self, template: &str) {
        self.pair_count.clear();
        template
            .chars()
            .tuple_windows()
            .for_each(|p| *self.pair_count.entry(p).or_insert(0) += 1);
    }

    fn init_elem_count(&mut self, template: &str) {
        self.elem_count.clear();
        for c in template.chars() {
            *self.elem_count.entry(c).or_insert(0) += 1;
        }
    }

    fn step(&mut self) {
        let mut new_pair_count = HashMap::new();
        for (p, i) in &self.pair_count {
            if let Some(new_elem) = self.rules.get(&p) {
                *new_pair_count.entry((p.0, *new_elem)).or_insert(0) += i;
                *new_pair_count.entry((*new_elem, p.1)).or_insert(0) += i;
                *self.elem_count.entry(*new_elem).or_insert(0) += i;
            }
        }
        self.pair_count = new_pair_count;
    }

    fn result(&self) -> u64 {
        self.elem_count
            .iter()
            .minmax_by_key(|(_, n)| *n)
            .into_option()
            .map(|(a, b)| b.1 - a.1)
            .unwrap_or(0)
    }
}

pub fn day_fourteen(step: u8) -> u64 {
    let mut polymer = Polymer::from(get_input("input/day14.txt"));
    let n = match step {
        1 => 10,
        2 => 40,
        _ => 0,
    };
    for _ in 0..n {
        polymer.step()
    }
    polymer.result()
}
