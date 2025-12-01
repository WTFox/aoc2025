use anyhow::Result;
use clap::Parser;
use std::fs;
use thiserror::Error;

use crate::days::*;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Day not implemented: {0}")]
    NotImplemented(u8),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

pub fn parse_args() -> Result<Args> {
    Ok(Args::try_parse()?)
}

fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn run(args: Args) -> Result<()> {
    let day = args.day;
    match day {
        1 => {
            let input = read_input(day);
            println!("Part one: {}", day01::part_one(&input));
            println!("Part two: {}", day01::part_two(&input));
            Ok(())
        }
        _ => anyhow::bail!(ParseError::NotImplemented(day)),
    }
}
