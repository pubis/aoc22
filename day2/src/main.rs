use std::env;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let rounds = std::fs::read_to_string(file_path).unwrap();

    let mut score = 0u64;
    for round in rounds.lines() {
        let moves: Vec<&str> = round.split(' ').collect();
        let opponent = moves[0];
        let me       = moves[1];

        match opponent {
            "A" => {
                match me {
                    "X" => score += 1 + 3,
                    "Y" => score += 2 + 6,
                    "Z" => score += 3 + 0,
                    _   => panic!("Unknown move"),
                }
            },
            "B" => {
                match me {
                    "X" => score += 1 + 0,
                    "Y" => score += 2 + 3,
                    "Z" => score += 3 + 6,
                    _   => panic!("Unknown move"),
                }
            },
            "C" => {
                match me {
                    "X" => score += 1 + 6,
                    "Y" => score += 2 + 0,
                    "Z" => score += 3 + 3,
                    _   => panic!("Unknown move"),
                }
            },
            _ => panic!("Unknown opponent move"),
        }
    }

    println!("Part 1: {}", score);
}
