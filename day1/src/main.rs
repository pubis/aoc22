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

    let mut v: Vec<i64> = vec![];
    let mut acc = 0i64;
    for line in reader.lines() {
      let l = line?;
      if l.is_empty() {
        v.push(acc);
        acc = 0;
      } else {
        let v = i64::from_str_radix(&l, 10).unwrap();
        acc += v;
      }
    }

    v.sort();
    v.reverse();

    println!("most: {}", v[0]);

    Ok(())
}
