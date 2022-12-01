use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file_path = env::args().nth(1)
      .expect("No input given");

    let mut file   = File::open(file_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let mut elves = buffer
      .split("\n\n")
      .map(|elf| elf
           .lines()
           .map(|line| line.parse::<usize>().unwrap())
           .sum())
      .collect::<Vec<_>>();

    elves.sort();

    println!("Part 1: {}", elves.last().unwrap());
    println!("Part 2: {}", elves.iter().rev().take(3).sum::<usize>());

    Ok(())
}
