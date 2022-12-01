use std::env;
use std::io;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let file_path = env::args().nth(1)
      .expect("No input given");

    let file    = File::open(file_path)?;
    let _reader = BufReader::new(file);

    Ok(())
}
