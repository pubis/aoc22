use std::env;

fn part1(input: &str) -> u32 {
    let chars: Vec<_> = input.chars().collect();
    let mut result = 0u32;
    for i in 3..=chars.len() {
        let mut tmp = vec![chars[i - 3], chars[i - 2], chars[i - 1], chars[i]];
        tmp.sort();
        tmp.dedup();
        if tmp.len() == 4 {
            result = i as u32 + 1;
            break;
        }
    }
    result
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let input = std::fs::read_to_string(file_path).unwrap();

    println!("Part 1: {}", part1(input.as_str()));
}

#[test]
fn example1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part1(input), 7);
}

#[test]
fn example2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part1(input), 5);
}

#[test]
fn example3() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part1(input), 6);
}

#[test]
fn example4() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part1(input), 10);
}

#[test]
fn example5() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part1(input), 11);
}
