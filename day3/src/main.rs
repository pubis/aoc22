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

    let mut part2 = 0u32;
    let mut start: usize = 0;
    loop {
        let group: Vec<_> = input.lines().skip(start).take(3).collect();
        start += 3;

        if group.len() == 0 { break; }

        let first = group[0].as_bytes();
        let mut tmp: Vec<u8> = vec![];

        for b in group[1].as_bytes() {
          if first.contains(b) {
            tmp.push(*b);
          }
        }

        let mut v: Vec<u8> = vec![];
        let last = group[2].as_bytes();
        for b in tmp {
          if last.contains(&b) {
            v.push(b);
          }
        }

        part2 += score(v[0]) as u32;
    }

    println!("Part 2: {}", part2);
}
