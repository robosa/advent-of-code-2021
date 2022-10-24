use super::get_input;

fn compute(operands: Vec<u64>, op: u8) -> u64 {
    match op {
        0 => operands.iter().sum(),
        1 => operands.iter().product(),
        2 => *operands.iter().min().expect("no min"),
        3 => *operands.iter().max().expect("no max"),
        5 | 6 | 7 if operands.len() != 2 => panic!("incorrect number of operands for comparison"),
        5 => {
            if operands[0] > operands[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if operands[0] < operands[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if operands[0] == operands[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("should not reach"),
    }
}

fn parse_n_bits(iter: impl Iterator<Item = char>, n: usize) -> u16 {
    u16::from_str_radix(&iter.take(n).collect::<String>(), 2).expect("parsing failed")
}

fn parse_literal(iter: &mut impl Iterator<Item = char>) -> u64 {
    let mut value = 0;
    while let Some('1') = iter.next() {
        value = value * 16 + parse_n_bits(iter.by_ref(), 4) as u64;
    }
    value * 16 + parse_n_bits(iter, 4) as u64
}

fn parse_operator(iter: &mut impl Iterator<Item = char>, op: u8) -> (u16, u64) {
    let mut version_sum = 0;
    let mut operands = Vec::new();
    if iter.next().expect("incomplete operator") == '0' {
        let len = parse_n_bits(iter.by_ref(), 15);
        let mut peekable = iter
            .take(len as usize)
            .collect::<Vec<_>>()
            .into_iter()
            .peekable();
        while peekable.peek().is_some() {
            let (v, r) = parse_packet(&mut peekable);
            version_sum += v;
            operands.push(r);
        }
    } else {
        for _ in 0..parse_n_bits(iter.by_ref(), 11) {
            let (v, r) = parse_packet(iter);
            version_sum += v;
            operands.push(r);
        }
    }
    (version_sum, compute(operands, op))
}

fn parse_packet(iter: &mut impl Iterator<Item = char>) -> (u16, u64) {
    let version_sum = parse_n_bits(iter.by_ref(), 3);
    let op = parse_n_bits(iter.by_ref(), 3) as u8;
    if op == 4 {
        (version_sum, parse_literal(iter))
    } else {
        let (v, result) = parse_operator(iter, op);
        (version_sum + v, result)
    }
}

pub fn day_sixteen(_step: u8) -> u64 {
    let binary_input: String = hex::decode(get_input("input/day16.txt"))
        .expect("invalid input")
        .iter()
        .map(|x| format!("{:08b}", x))
        .collect();
    let mut iter = binary_input.chars();
    let (v, r) = parse_packet(&mut iter);
    match _step {
        1 => v as u64,
        2 => r,
        _ => 0,
    }
}
