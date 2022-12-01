use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::vec;

fn main() -> io::Result<()> {
    let file_path = env::args().nth(1)
      .expect("No input given");

    let file   = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut elves = reader.lines()
      .map(|line| line.unwrap())
      .fold(vec![0], |mut acc, line| {
        line.parse::<usize>()
          .map(|v| *acc.last_mut().unwrap() += v)
          .map_err(|_| acc.push(0))
          .ok();
        acc
      });

    elves.sort();

    println!("Part 1: {}", elves.last().unwrap());
    println!("Part 2: {}", elves.iter().rev().take(3).sum::<usize>());

    Ok(())
}
