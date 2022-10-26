use itertools::Itertools;

use super::get_input;

fn try_shot(v0: (i32, i32), ((wx0, wx1), (wy0, wy1)): ((i32, i32), (i32, i32))) -> bool {
    let (mut vx, mut vy) = v0;
    let (mut x, mut y) = (0, 0);
    std::iter::repeat_with(|| {
        x += vx;
        y += vy;
        vx = std::cmp::max(0, vx - 1);
        vy -= 1;
        (x, y)
    })
    .take_while(|(x, y)| x <= &wx1 && y >= &wy0)
    .any(|(x, y)| (wx0..=wx1).contains(&x) && (wy0..=wy1).contains(&y))
}

pub fn day_seventeen(step: u8) -> u32 {
    let input = get_input("input/day17.txt");
    let window =
        input
            .split_once("x=")
            .and_then(|(_, s)| s.split_once(", y="))
            .and_then(|(s1, s2)| {
                s1.split_once("..")
                    .and_then(|(sx1, sx2)| sx1.parse::<i32>().ok().zip(sx2.parse::<i32>().ok()))
                    .zip(s2.split_once("..").and_then(|(sy1, sy2)| {
                        sy1.parse::<i32>().ok().zip(sy2.parse::<i32>().ok())
                    }))
            })
            .expect("invalid input");
    let ((_, wx1), (wy0, wy1)) = window;
    if wy0 >= 0 || wy1 >= 1 {
        panic!("target should be below");
    }
    if step == 1 {
        (wy0 * (wy0 + 1) / 2) as u32
    } else {
        (1..=wx1)
            .cartesian_product(wy0..-wy0)
            .filter(|v| try_shot(*v, window))
            .count() as u32
    }
}
