use std::env;
use std::collections::HashSet;

fn solve(input: &str, len: usize) -> usize {
    let chars: Vec<_> = input.chars().collect();
    let mut result = len;
    for window in chars.windows(len) {
        let set: HashSet<_> = window.iter().cloned().collect();
        if set.len() == len {
            break;
        }
        result += 1;
    }
    result
}

fn part1(input: &str) -> usize {
    solve(input, 4)
}

fn part2(input: &str) -> usize {
    solve(input, 14)
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let input = std::fs::read_to_string(file_path).unwrap();

    println!("Part 1: {}", part1(input.as_str()));
    println!("Part 2: {}", part2(input.as_str()));
}

#[test]
fn example1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part1(input), 7);
    assert_eq!(part2(input), 19);
}

#[test]
fn example2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 23);
}

#[test]
fn example3() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part1(input), 6);
    assert_eq!(part2(input), 23);
}

#[test]
fn example4() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part1(input), 10);
    assert_eq!(part2(input), 29);
}

#[test]
fn example5() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part1(input), 11);
    assert_eq!(part2(input), 26);
}
