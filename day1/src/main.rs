use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::vec;

fn main() -> io::Result<()> {
    let file = env::args().nth(1)
      .expect("No input given");

    println!("Input: {file}");

    let f = File::open(file)?;
    let reader = BufReader::new(f);
    let lines = reader.lines()
      .map(|line| line.unwrap())
      .map(|line| line.parse::<i64>());

    let mut v: Vec<i64> = vec![];
    let mut acc = 0i64;
    for line in lines {
      match line {
        Ok(val) => acc += val,
        Err(_)  => {
          v.push(acc);
          acc = 0;
        },
      }
    }

    v.sort();
    v.reverse();

    println!("most: {}", v[0]);

    let top3 = v[0] + v[1] + v[2];
    println!("top3: {}", top3);

    Ok(())
}
