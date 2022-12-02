use std::env;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let rounds = std::fs::read_to_string(file_path).unwrap();

    let (part1, part2) = rounds
        .lines()
        .map(|round| {
            let moves: Vec<&str> = round.split_ascii_whitespace().collect();
            (moves[0], moves[1])
        })
        .map(|moves| {
            match moves {
              ("A", "X") => (1 + 3, 3 + 0),
              ("A", "Y") => (2 + 6, 1 + 3),
              ("A", "Z") => (3 + 0, 2 + 6),
              ("B", "X") => (1 + 0, 1 + 0),
              ("B", "Y") => (2 + 3, 2 + 3),
              ("B", "Z") => (3 + 6, 3 + 6),
              ("C", "X") => (1 + 6, 2 + 0),
              ("C", "Y") => (2 + 0, 3 + 3),
              ("C", "Z") => (3 + 3, 1 + 6),
              _ => (0, 0),
            }
        })
        .fold((0, 0), |acc, score| (acc.0 + score.0, acc.1 + score.1));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
