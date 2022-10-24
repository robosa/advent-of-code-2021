use std::collections::{BinaryHeap, HashMap, HashSet};

use super::get_input;

#[derive(Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost)
            .cmp(&self.cost)
            .then_with(|| self.pos.0.cmp(&other.pos.0))
            .then_with(|| self.pos.1.cmp(&other.pos.1))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    height: usize,
    width: usize,
    weights: Vec<Vec<usize>>,
    repeats: usize,
    repeated_height: usize,
    repeated_width: usize,
}

impl From<String> for Grid {
    fn from(input: String) -> Self {
        let weights: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map(|x| x as usize))
                    .collect()
            })
            .collect::<Option<Vec<Vec<usize>>>>()
            .unwrap_or_default();
        let height = weights.len();
        let width = if height > 0 { weights[0].len() } else { 0 };
        if weights.iter().all(|line| line.len() == width) {
            Self {
                height,
                width,
                weights,
                repeats: 1,
                repeated_height: height,
                repeated_width: width,
            }
        } else {
            Default::default()
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            height: 1,
            width: 1,
            weights: vec![vec![0]],
            repeats: 1,
            repeated_height: 1,
            repeated_width: 1,
        }
    }
}

impl Grid {
    fn update_repeats(&mut self, repeats: usize) {
        self.repeats = repeats;
        self.repeated_height = self.height * repeats;
        self.repeated_width = self.width * repeats;
    }

    fn weight(&self, p: (usize, usize)) -> usize {
        let (qx, rx) = (p.0 / self.height, p.0 % self.height);
        let (qy, ry) = (p.1 / self.width, p.1 % self.width);
        (self.weights[rx][ry] + qx + qy - 1) % 9 + 1
    }

    fn neighbors(&self, p: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let mut neighbors = Vec::new();
        if p.0 > 0 {
            neighbors.push((p.0 - 1, p.1))
        }
        if p.1 > 0 {
            neighbors.push((p.0, p.1 - 1))
        }
        if p.0 + 1 < self.repeated_height {
            neighbors.push((p.0 + 1, p.1))
        }
        if p.1 + 1 < self.repeated_width {
            neighbors.push((p.0, p.1 + 1))
        }
        neighbors.into_iter()
    }

    fn distance_to_end(&self, p: (usize, usize)) -> usize {
        p.0.abs_diff(self.repeated_height - 1) + p.1.abs_diff(self.repeated_width - 1)
    }

    fn astar(&self) -> Option<usize> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        let end = (self.repeated_height - 1, self.repeated_width - 1);

        costs.insert((0, 0), 0);
        queue.push(Node {
            pos: (0, 0),
            cost: self.distance_to_end((0, 0)),
        });

        while let Some(Node { pos, cost }) = queue.pop() {
            if pos == end {
                println!("{}", visited.len());
                return Some(cost);
            }
            if !visited.insert(pos) {
                continue;
            }
            for neighbor in self.neighbors(pos).filter(|p| !visited.contains(p)) {
                let new_cost = *costs.get(&pos).expect("Current node should have a cost")
                    + self.weight(neighbor);
                if new_cost < *costs.get(&neighbor).unwrap_or(&usize::MAX) {
                    costs.insert(neighbor, new_cost);
                    queue.push(Node {
                        pos: neighbor,
                        cost: new_cost + self.distance_to_end(neighbor),
                    })
                }
            }
        }
        None
    }
}

pub fn day_fifteen(step: u8) -> usize {
    let mut grid = Grid::from(get_input("input/day15.txt"));
    if step == 2 {
        grid.update_repeats(5);
    }
    grid.astar().unwrap_or_default()
}
