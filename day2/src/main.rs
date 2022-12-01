use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = env::args().nth(1)
      .expect("No input given");

    println!("Input: {file}");

    let f = File::open(file)?;
    let _reader = BufReader::new(f);

    Ok(())
}
