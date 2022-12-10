use std::env;

fn part1(input: &str) -> i32 {
    let mut register: i32 = 1;
    let mut history: Vec<i32> = vec![register];

    for line in input.lines() {
        if line.starts_with("noop") {
            history.push(register);
        } else {
            let v = line[5..].parse::<i32>().unwrap();
            history.push(register);
            register += v;
            history.push(register);
        }
    }

    history[19] * 20
        + history[59] * 60
        + history[99] * 100
        + history[139] * 140
        + history[179] * 180
        + history[219] * 220
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let input = std::fs::read_to_string(file_path).unwrap();

    println!("Part 1: {}", part1(&input));
}

#[test]
fn example() {
    let input = std::fs::read_to_string("day10/example.txt").unwrap();

    assert_eq!(part1(&input), 13140);
}
