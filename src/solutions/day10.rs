use super::get_input;

fn check_closer(opener: Option<char>, closer: char) -> bool {
    match (opener, closer) {
        (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => true,
        _ => false,
    }
}

fn get_corrupted_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_incomplete_score(stack: Vec<char>) -> u64 {
    stack.iter().rev().fold(0, |score, c| {
        score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
    })
}

fn parse(input: &str, step: u8) -> Option<u64> {
    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                if !check_closer(stack.pop(), c) {
                    return match step {
                        1 => Some(get_corrupted_score(c)),
                        _ => None,
                    };
                }
            }
        }
    }
    match step {
        2 => Some(get_incomplete_score(stack)),
        _ => None,
    }
}

pub fn day_ten(step: u8) -> u64 {
    let input = get_input("input/day10.txt");
    let mut scores: Vec<_> = input.lines().filter_map(|s| parse(s, step)).collect();
    match step {
        1 => scores.iter().sum(),
        2 => {
            scores.sort_unstable();
            scores[scores.len() / 2]
        }
        _ => 0,
    }
}
