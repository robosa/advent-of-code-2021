use super::get_input;

struct Octopuses {
    energy: Vec<Vec<Option<u32>>>,
    flashes: u32,
}

impl From<String> for Octopuses {
    fn from(input: String) -> Self {
        Self {
            energy: input
                .lines()
                .map(|s| s.chars().map(|i| i.to_digit(10)).collect())
                .collect(),
            flashes: 0,
        }
    }
}

impl Octopuses {
    fn get_energy(&self, p: (usize, usize)) -> Option<u32> {
        if p.0 < self.energy.len() && p.1 < self.energy[p.0].len() {
            self.energy[p.0][p.1]
        } else {
            None
        }
    }

    fn get_neighbors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if p.0 >= self.energy.len() || p.1 >= self.energy[p.0].len() {
            return neighbors;
        }
        if p.0 > 0 {
            if p.1 > 0 && p.1 - 1 < self.energy[p.0 - 1].len() {
                neighbors.push((p.0 - 1, p.1 - 1));
            }
            if p.1 < self.energy[p.0 - 1].len() {
                neighbors.push((p.0 - 1, p.1));
            }
            if p.1 + 1 < self.energy[p.0 - 1].len() {
                neighbors.push((p.0 - 1, p.1 + 1));
            }
        }
        if p.0 < self.energy.len() - 1 {
            if p.1 > 0 && p.1 - 1 < self.energy[p.0 + 1].len() {
                neighbors.push((p.0 + 1, p.1 - 1));
            }
            if p.1 < self.energy[p.0 + 1].len() {
                neighbors.push((p.0 + 1, p.1));
            }
            if p.1 + 1 < self.energy[p.0 + 1].len() {
                neighbors.push((p.0 + 1, p.1 + 1));
            }
        }
        if p.1 > 0 {
            neighbors.push((p.0, p.1 - 1));
        }
        if p.1 < self.energy[p.0].len() {
            neighbors.push((p.0, p.1 + 1));
        }
        neighbors
    }

    fn increase_energy(&mut self, p: (usize, usize)) {
        if let Some(x) = self.get_energy(p) {
            self.energy[p.0][p.1] = Some(x + 1);
            if x + 1 >= 10 {
                self.energy[p.0][p.1] = None;
                self.flashes += 1;
                self.get_neighbors(p)
                    .iter()
                    .for_each(|p| self.increase_energy(*p))
            }
        }
    }

    fn step(&mut self) -> u32 {
        self.flashes = 0;
        for i in 0..self.energy.len() {
            for j in 0..self.energy[i].len() {
                self.increase_energy((i, j));
            }
        }
        for i in 0..self.energy.len() {
            for j in 0..self.energy[i].len() {
                if self.get_energy((i, j)) == None {
                    self.energy[i][j] = Some(0);
                };
            }
        }
        self.flashes
    }
}

pub fn day_eleven(step: u8) -> u32 {
    let mut octopuses = Octopuses::from(get_input("input/day11.txt"));
    match step {
        1 => (0..100).into_iter().map(|_| octopuses.step()).sum(),
        2 => {
            (1..)
                .map(|i| (i, octopuses.step()))
                .find(|(_, x)| *x == 100)
                .unwrap()
                .0
        }
        _ => 0,
    }
}
