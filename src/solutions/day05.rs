use super::get_input;

struct Grid {
    content: Vec<Vec<u32>>,
    overlap_count: usize,
}

impl Grid {
    fn create() -> Self {
        Grid {
            content: vec![vec![0; 1000]; 1000],
            overlap_count: 0,
        }
    }

    fn add_line(&mut self, p1: (usize, usize), p2: (usize, usize), diagonals: bool) {
        match (p1, p2) {
            ((a, b), (c, d)) if (a == c) && (b < d) => self.add_horizontal(a, b, d),
            ((a, b), (c, d)) if (a == c) && (d < b) => self.add_horizontal(a, d, b),
            ((a, b), (c, d)) if (a < c) && (b == d) => self.add_vertical(b, a, c),
            ((a, b), (c, d)) if (c < a) && (b == d) => self.add_vertical(b, c, a),
            ((a, b), (c, d)) if diagonals && (a < c) && (b < d) => self.add_diagonal_down(a, c, b),
            ((a, b), (c, d)) if diagonals && (a < c) && (d < b) => self.add_diagonal_up(a, c, b),
            ((a, b), (c, d)) if diagonals && (c < a) && (b < d) => self.add_diagonal_up(c, a, d),
            ((a, b), (c, d)) if diagonals && (c < a) && (d < b) => self.add_diagonal_down(c, a, d),
            _ => (),
        }
    }

    fn add_horizontal(&mut self, i: usize, j0: usize, j1: usize) {
        for j in j0..=j1 {
            self.content[i][j] += 1;
            if self.content[i][j] == 2 {
                self.overlap_count += 1;
            }
        }
    }

    fn add_vertical(&mut self, j: usize, i0: usize, i1: usize) {
        for i in i0..=i1 {
            self.content[i][j] += 1;
            if self.content[i][j] == 2 {
                self.overlap_count += 1;
            }
        }
    }

    fn add_diagonal_down(&mut self, i0: usize, i1: usize, j0: usize) {
        let mut j = j0;
        for i in i0..=i1 {
            self.content[i][j] += 1;
            if self.content[i][j] == 2 {
                self.overlap_count += 1;
            }
            j += 1;
        }
    }

    fn add_diagonal_up(&mut self, i0: usize, i1: usize, j0: usize) {
        let mut j = j0;
        for i in i0..=i1 {
            self.content[i][j] += 1;
            if self.content[i][j] == 2 {
                self.overlap_count += 1;
            }
            j -= 1;
        }
    }
}

fn parse_point(str: &str) -> Option<(usize, usize)> {
    if let Some((s1, s2)) = str.split_once(',') {
        if let (Some(a), Some(b)) = (s1.parse().ok(), s2.parse().ok()) {
            return Some((a, b));
        }
    }
    None
}

fn parse_line(line: &str) -> Option<((usize, usize), (usize, usize))> {
    if let Some((s1, s2)) = line.split_once(" -> ") {
        if let (Some(p1), Some(p2)) = (parse_point(s1), parse_point(s2)) {
            return Some((p1, p2));
        }
    }
    None
}

pub fn day_five(step: u8) -> usize {
    let input = get_input("input/day05.txt");
    let mut grid = Grid::create();
    let diagonals = step == 2;
    input
        .lines()
        .filter_map(parse_line)
        .for_each(|(p1, p2)| grid.add_line(p1, p2, diagonals));
    grid.overlap_count
}
