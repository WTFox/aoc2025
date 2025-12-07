use anyhow::Result;
use clap::Parser;
use std::fs;
use thiserror::Error;

use crate::days::*;

#[derive(Error, Debug)]
enum ParseError {
    #[error("Day not implemented: {0}")]
    NotImplemented(u8),

    #[error("No input file found: {0}")]
    NoInputFile(u8),
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

fn read_input(day: u8) -> Result<String> {
    match fs::read_to_string(format!("inputs/day{:02}.txt", day)) {
        Ok(val) => Ok(val),
        _ => anyhow::bail!(ParseError::NoInputFile(day)),
    }
}

pub fn run(args: Args) -> Result<()> {
    let day = args.day;
    println!();
    println!("Day {:02}", day);
    println!();
    match day {
        1 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day01::part_one(&input));
            println!("  Part two: {}", day01::part_two(&input));
            Ok(())
        }
        2 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day02::part_one(&input));
            println!("  Part two: {}", day02::part_two(&input));
            Ok(())
        }
        3 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day03::part_one(&input));
            println!("  Part two: {}", day03::part_two(&input));
            Ok(())
        }
        4 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day04::part_one(&input));
            println!("  Part two: {}", day04::part_two(&input));
            Ok(())
        }
        5 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day05::part_one(&input));
            println!("  Part two: {}", day05::part_two(&input));
            Ok(())
        }
        6 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day06::part_one(&input));
            println!("  Part two: {}", day06::part_two(&input));
            Ok(())
        }
        7 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day07::part_one(&input));
            println!("  Part two: {}", day07::part_two(&input));
            Ok(())
        }
        8 => {
            let input = read_input(day)?;
            println!("  Part one: {}", day08::part_one(&input));
            println!("  Part two: {}", day08::part_two(&input));
            Ok(())
        }
        _ => anyhow::bail!(ParseError::NotImplemented(day)),
    }
}
