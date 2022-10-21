use std::collections::{HashMap, HashSet};

use super::get_input;

struct Caves {
    network: HashMap<String, HashSet<String>>,
}

impl From<String> for Caves {
    fn from(input: String) -> Self {
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        for s in input.lines() {
            if let Some((n, m)) = s.split_once('-') {
                if let Some(dest) = map.get_mut(n) {
                    dest.insert(m.to_string());
                } else {
                    map.insert(n.to_string(), HashSet::from([m.to_string()]));
                }
                if let Some(dest) = map.get_mut(m) {
                    dest.insert(n.to_string());
                } else {
                    map.insert(m.to_string(), HashSet::from([n.to_string()]));
                }
            }
        }
        Self { network: map }
    }
}

impl Caves {
    fn explore(&self, start: &str, excluded: &mut HashSet<String>, twice: bool) -> u32 {
        let mut count = 0;
        if start.to_lowercase() == start {
            excluded.insert(start.to_string());
        }
        if let Some(neighbors) = self.network.get(start) {
            for n in neighbors {
                count += match n.as_str() {
                    "end" => 1,
                    "start" => 0,
                    n if !excluded.contains(n) => self.explore(n, &mut excluded.clone(), twice),
                    n if twice => self.explore(n, &mut excluded.clone(), false),
                    _ => 0,
                }
            }
        }
        count
    }
}

pub fn day_twelve(step: u8) -> u32 {
    let caves = Caves::from(get_input("input/day12.txt"));
    caves.explore("start", &mut HashSet::new(), step == 2)
}
