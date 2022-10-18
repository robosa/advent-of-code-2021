use std::{collections::HashSet, cmp::Reverse};

use super::get_input;

#[derive(Default)]
struct HeightMap {
    content: Vec<Vec<u32>>,
    low_points: Vec<(usize, usize)>,
}

impl HeightMap {
    fn generate(input: &str) -> HeightMap {
        HeightMap {
            content: input
                .lines()
                .filter_map(|s| s.chars().map(|i| i.to_digit(10)).collect())
                .collect(),
            low_points: Vec::new(),
        }
    }

    fn get_point(&self, p: (usize, usize)) -> u32 {
        if p.0 < self.content.len() && p.1 < self.content[p.0].len() {
            self.content[p.0][p.1]
        } else {
            9
        }
    }

    fn check_up(&self, p: (usize, usize)) -> u32 {
        if p.0 == 0 || self.content[p.0 - 1].len() <= p.1 {
            9
        } else {
            self.content[p.0 - 1][p.1]
        }
    }

    fn check_down(&self, p: (usize, usize)) -> u32 {
        if p.0 == self.content.len() - 1 || self.content[p.0 + 1].len() <= p.1 {
            9
        } else {
            self.content[p.0 + 1][p.1]
        }
    }

    fn check_left(&self, p: (usize, usize)) -> u32 {
        if p.1 == 0 {
            9
        } else {
            self.content[p.0][p.1 - 1]
        }
    }

    fn check_right(&self, p: (usize, usize)) -> u32 {
        if p.1 == self.content[p.0].len() - 1 {
            9
        } else {
            self.content[p.0][p.1 + 1]
        }
    }

    fn is_low(&self, p: (usize, usize)) -> bool {
        let h = self.get_point(p);
        h < self.check_up(p)
            && h < self.check_down(p)
            && h < self.check_left(p)
            && h < self.check_right(p)
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
            return 0
        }
        let mut stack = vec![p];
        let mut visited = HashSet::from([p]);
        while let Some(p) = stack.pop() {
            if self.check_up(p) < 9 && visited.insert((p.0 - 1, p.1)) {
                stack.push((p.0 - 1, p.1))
            };
            if self.check_down(p) < 9 && visited.insert((p.0 + 1, p.1)) {
                stack.push((p.0 + 1, p.1))
            };
            if self.check_left(p) < 9 && visited.insert((p.0, p.1 - 1)) {
                stack.push((p.0, p.1 - 1))
            };
            if self.check_right(p) < 9 && visited.insert((p.0, p.1 + 1)) {
                stack.push((p.0, p.1 + 1))
            };
        }
        visited.len()
    }

    fn step_1(&self) -> u32 {
        self.low_points.iter().map(|p| self.get_point(*p) + 1).sum()
    }

    fn step_2(&self) -> u32 {
        let mut sizes: Vec<usize> = self
            .low_points
            .iter()
            .map(|p| self.get_basin_size(*p))
            .collect();
        sizes.sort_unstable_by_key(|x| Reverse(*x));
        sizes
            .iter()
            .take(3)
            .fold(1, |product, size| product * *size as u32)
    }
}

pub fn day_nine(step: u8) -> u32 {
    let input = get_input("input/day09.txt");
    let mut height_map = HeightMap::generate(&input);
    height_map.find_low_points();
    match step {
        1 => height_map.step_1(),
        2 => height_map.step_2(),
        _ => 0,
    }
}
