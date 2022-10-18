use super::get_input;

fn clean_line(line: &str) -> Option<(&str, u32)> {
    if let Some((s, i)) = line.split_once(' ') {
        if let Some(i) = i.parse::<u32>().ok() {
            return Some((s, i));
        }
    }
    None
}

fn update_position_1(pos: (u32, u32, u32), mov: (&str, u32)) -> (u32, u32, u32) {
    let (height, depth, _) = pos;
    let (dir, steps) = mov;
    match dir {
        "forward" => (height + steps, depth, 0),
        "down" => (height, depth + steps, 0),
        "up" => (height, depth - steps, 0),
        _ => (height, depth, 0),
    }
}

fn update_position_2(pos: (u32, u32, u32), mov: (&str, u32)) -> (u32, u32, u32) {
    let (height, depth, aim) = pos;
    let (dir, steps) = mov;
    match dir {
        "forward" => (height + steps, depth + aim * steps, aim),
        "down" => (height, depth, aim + steps),
        "up" => (height, depth, aim - steps),
        _ => (height, depth, aim),
    }
}

pub fn day_two(step: u8) -> u32 {
    let input = get_input("input/day02.txt");
    let data = input.lines().filter_map(clean_line);
    let (height, depth, _) = match step {
        1 => data.fold((0, 0, 0), update_position_1),
        2 => data.fold((0, 0, 0), update_position_2),
        _ => (0, 0, 0),
    };
    height * depth
}
