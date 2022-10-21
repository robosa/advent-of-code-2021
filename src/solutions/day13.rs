use std::{collections::HashSet, iter::FromIterator};

use super::get_input;

struct Paper {
    points: HashSet<(usize, usize)>,
}

impl<'a> FromIterator<&'a str> for Paper {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Paper {
            points: iter
                .into_iter()
                .filter_map(|s| {
                    s.split_once(',')
                        .and_then(|(x, y)| x.parse().ok().zip(y.parse().ok()))
                })
                .collect(),
        }
    }
}

impl Paper {
    fn fold(&mut self, params: (&str, usize)) {
        match params {
            ("fold along x", n) => self.fold_left(n),
            ("fold along y", n) => self.fold_up(n),
            _ => (),
        }
    }

    fn fold_up(&mut self, n: usize) {
        self.points
            .clone()
            .iter()
            .filter(|(_, y)| y > &n)
            .for_each(|(x, y)| {
                self.points.remove(&(*x, *y));
                if 2 * n >= *y {
                    self.points.insert((*x, 2 * n - y));
                };
            })
    }

    fn fold_left(&mut self, n: usize) {
        self.points
            .clone()
            .iter()
            .filter(|(x, _)| x > &n)
            .for_each(|(x, y)| {
                self.points.remove(&(*x, *y));
                if 2 * n >= *x {
                    self.points.insert((2 * n - x, *y));
                }
            })
    }

    fn format(&self, width: usize, height: usize) -> String {
        let mut output = String::new();
        for y in 0..height {
            for x in 0..width {
                if self.points.contains(&(x, y)) {
                    output.push('#')
                } else {
                    output.push(' ')
                }
            }
            output.push('\n')
        }
        output.pop();
        output
    }
}

pub fn day_thirteen(step: u8) -> String {
    let input = get_input("input/day13.txt");
    let mut line_iter = input.lines();
    let mut paper: Paper = line_iter.by_ref().take_while(|s| *s != "").collect();
    let folds = line_iter.filter_map(|s| {
        s.split_once('=')
            .and_then(|(t, s)| Some(t).zip(s.parse().ok()))
    });
    for t in folds {
        paper.fold(t);
        if step == 1 {
            return paper.points.len().to_string();
        }
    }
    paper.format(39, 6)
}
