mod solutions;

use clap::Parser;
use solutions::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
    #[arg(value_parser = clap::value_parser!(u8).range(1..3))]
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
        _ => (),
    }
}
