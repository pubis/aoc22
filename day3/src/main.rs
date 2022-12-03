use std::env;

fn score(ch: u8) -> u8 {
  if ch >= 97 {
    ch - b'a' + 1
  } else {
    ch - b'A' + 27
  }
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let input = std::fs::read_to_string(file_path).unwrap();
    let rucksacks = input.split_ascii_whitespace();

    let mut part1 = 0u32;
    for rucksack in rucksacks {
      let (first, last) = rucksack.split_at(rucksack.len() / 2);
      let bytes = last.as_bytes();

      for b in first.as_bytes() {
        if bytes.contains(b) {
          part1 += score(*b) as u32;
          break;
        }
      }
    }

    println!("Part 1: {}", part1);
}
