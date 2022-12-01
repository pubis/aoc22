use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("day1/example1.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
