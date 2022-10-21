use std::{cmp::Reverse, collections::HashSet};

use super::get_input;

struct HeightMap {
    content: Vec<Vec<u32>>,
    low_points: Vec<(usize, usize)>,
}

impl From<String> for HeightMap {
    fn from(input: String) -> Self {
        HeightMap {
            content: input
                .lines()
                .filter_map(|s| s.chars().map(|i| i.to_digit(10)).collect())
                .collect(),
            low_points: Vec::new(),
        }
    }
}

impl HeightMap {
    fn get_neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if p.0 >= self.content.len() || p.1 >= self.content[p.0].len() {
            return neighbors;
        }
        if p.0 > 0 && p.1 < self.content[p.0 - 1].len() {
            neighbors.push((p.0 - 1, p.1));
        }
        if p.0 < self.content.len() - 1 && p.1 < self.content[p.0 + 1].len() {
            neighbors.push((p.0 + 1, p.1));
        }
        if p.1 > 0 {
            neighbors.push((p.0, p.1 - 1));
        }
        if p.1 < self.content[p.0].len() {
            neighbors.push((p.0, p.1 + 1));
        }
        neighbors
    }

    fn get_point(&self, p: (usize, usize)) -> u32 {
        if p.0 < self.content.len() && p.1 < self.content[p.0].len() {
            self.content[p.0][p.1]
        } else {
            9
        }
    }

    fn is_low(&self, p: (usize, usize)) -> bool {
        let h = self.get_point(p);
        self.get_neighbors(p)
            .into_iter()
            .find(|n| self.get_point(*n) <= h)
            == None
    }

    fn find_low_points(&mut self) {
        self.low_points = Vec::new();
        for i in 0..self.content.len() {
            for j in 0..self.content[i].len() {
                if self.is_low((i, j)) {
                    self.low_points.push((i, j));
                }
            }
        }
    }

    fn get_basin_size(&self, p: (usize, usize)) -> usize {
        if self.get_point(p) == 9 {
            return 0;
        }
        let mut stack = vec![p];
        let mut visited = HashSet::from([p]);
        while let Some(p) = stack.pop() {
            for n in self.get_neighbors(p) {
                if self.get_point(n) < 9 && visited.insert(n) {
                    stack.push(n);
                }
            }
        }
        visited.len()
    }

    fn step_1(&self) -> u32 {
        self.low_points.iter().map(|p| self.get_point(*p) + 1).sum()
    }

    fn step_2(&self) -> u32 {
        let mut sizes = self
            .low_points
            .iter()
            .map(|p| self.get_basin_size(*p))
            .collect::<Vec<_>>();
        sizes.sort_unstable_by_key(|x| Reverse(*x));
        sizes
            .iter()
            .take(3)
            .fold(1, |product, size| product * *size as u32)
    }
}

pub fn day_nine(step: u8) -> u32 {
    let mut height_map = HeightMap::from(get_input("input/day09.txt"));
    height_map.find_low_points();
    match step {
        1 => height_map.step_1(),
        2 => height_map.step_2(),
        _ => 0,
    }
}
