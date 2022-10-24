mod solutions;

use clap::Parser;
use solutions::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    step: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => println!("{}", day_one(args.step)),
        2 => println!("{}", day_two(args.step)),
        3 => println!("{}", day_three(args.step)),
        4 => println!("{}", day_four(args.step)),
        5 => println!("{}", day_five(args.step)),
        6 => println!("{}", day_six(args.step)),
        7 => println!("{}", day_seven(args.step)),
        8 => println!("{}", day_eight(args.step)),
        9 => println!("{}", day_nine(args.step)),
        10 => println!("{}", day_ten(args.step)),
        11 => println!("{}", day_eleven(args.step)),
        12 => println!("{}", day_twelve(args.step)),
        13 => println!("{}", day_thirteen(args.step)),
        14 => println!("{}", day_fourteen(args.step)),
        15 => println!("{}", day_fifteen(args.step)),
        16 => println!("{}", day_sixteen(args.step)),
        _ => (),
    }
}
