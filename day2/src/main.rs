use std::env;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let rounds = std::fs::read_to_string(file_path).unwrap();

    let mut part1 = 0u64;
    let mut part2 = 0u64;
    for round in rounds.lines() {
        let moves: Vec<&str> = round.split(' ').collect();
        let opponent = moves[0];
        let me       = moves[1];

        match opponent {
            "A" => {
                match me {
                    "X" => { part1 += 1 + 3; part2 += 3 + 0; },
                    "Y" => { part1 += 2 + 6; part2 += 1 + 3; },
                    "Z" => { part1 += 3 + 0; part2 += 2 + 6; },
                    _   => panic!("Unknown move"),
                }
            },
            "B" => {
                match me {
                    "X" => { part1 += 1 + 0; part2 += 1 + 0; },
                    "Y" => { part1 += 2 + 3; part2 += 2 + 3; },
                    "Z" => { part1 += 3 + 6; part2 += 3 + 6; },
                    _   => panic!("Unknown move"),
                }
            },
            "C" => {
                match me {
                    "X" => { part1 += 1 + 6; part2 += 2 + 0; },
                    "Y" => { part1 += 2 + 0; part2 += 3 + 3; },
                    "Z" => { part1 += 3 + 3; part2 += 1 + 6; },
                    _   => panic!("Unknown move"),
                }
            },
            _ => panic!("Unknown opponent move"),
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
