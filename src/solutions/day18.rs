use std::{fmt::Display, ops::Add};

use itertools::Itertools;

use super::get_input;

struct Number {
    left: Box<Element>,
    right: Box<Element>,
}

enum Element {
    Val(u32),
    Num(Number),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(x) => write!(f, "{}", x),
            Self::Num(number) => write!(f, "{}", number),
        }
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        let half = value / 2;
        Self {
            left: Box::new(Element::Val(half)),
            right: Box::new(Element::Val(value - half)),
        }
    }
}

impl From<String> for Number {
    fn from(str: String) -> Self {
        let mut iter = str.chars();
        if iter.next() != Some('[') {
            panic!("invalid string")
        }
        Self::parse(&mut iter)
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self {
        let mut result = Self {
            left: Box::new(Element::Num(self)),
            right: Box::new(Element::Num(rhs)),
        };
        result.reduce();
        result
    }
}

impl Number {
    fn parse(iter: &mut impl Iterator<Item = char>) -> Self {
        let left = Box::new(Element::parse(iter));
        if iter.next() != Some(',') {
            panic!("invalid string");
        }
        let right = Box::new(Element::parse(iter));
        if iter.next() != Some(']') {
            panic!("invalid string");
        }
        Self { left, right }
    }

    fn explode(&mut self, depth: u32) -> (Option<u32>, Option<u32>) {
        let (left_out, right_in) = self.left.explode(depth + 1);
        if let Some(value) = right_in {
            self.right.add_left(value);
        }
        let (left_in, right_out) = self.right.explode(depth + 1);
        if let Some(value) = left_in {
            self.left.add_right(value);
        }
        (left_out, right_out)
    }

    fn split(&mut self) -> bool {
        self.left.split() || self.right.split()
    }

    fn reduce(&mut self) {
        let mut modified = true;
        while modified {
            self.explode(0);
            modified = self.split();
        }
    }

    fn magnitude(&self) -> u32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

impl Element {
    fn get_value(&self) -> Option<u32> {
        match self {
            Self::Val(i) => Some(*i),
            _ => None,
        }
    }

    fn parse(iter: &mut impl Iterator<Item = char>) -> Self {
        match iter.next() {
            Some('[') => Self::Num(Number::parse(iter)),
            Some(char) => Self::Val(char.to_digit(10).expect("invalid string")),
            _ => panic!("invalid string"),
        }
    }

    fn explode(&mut self, depth: u32) -> (Option<u32>, Option<u32>) {
        match self {
            Self::Val(_) => (None, None),
            Self::Num(number) if depth < 4 => number.explode(depth),
            Self::Num(number) => {
                let (left, right) = (number.left.get_value(), number.right.get_value());
                *self = Self::Val(0);
                (left, right)
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Val(x) if *x > 9 => {
                *self = Self::Num(Number::from(*x));
                true
            }
            Self::Val(_) => false,
            Self::Num(number) => number.split(),
        }
    }

    fn add_left(&mut self, value: u32) {
        match self {
            Self::Val(x) => *x += value,
            Self::Num(number) => number.left.add_left(value),
        }
    }

    fn add_right(&mut self, value: u32) {
        match self {
            Self::Val(x) => *x += value,
            Self::Num(number) => number.right.add_right(value),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Val(x) => *x,
            Self::Num(number) => number.magnitude(),
        }
    }
}

pub fn day_eighteen(step: u8) -> u32 {
    let input = get_input("input/day18.txt");

    match step {
        1 => input
            .lines()
            .map(|s| Number::from(s.to_string()))
            .reduce(|a, b| a + b)
            .expect("at least one number should be in input")
            .magnitude(),
        2 => input
            .lines()
            .enumerate()
            .cartesian_product(input.lines().enumerate())
            .filter(|((i, _), (j, _))| i != j)
            .map(|((_, a), (_, b))| {
                (Number::from(a.to_string()) + Number::from(b.to_string())).magnitude()
            })
            .max()
            .expect("at least two numbers should be in input"),
        _ => 0,
    }
}
