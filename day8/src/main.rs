use std::env;

fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line
                .as_bytes()
                .iter()
                .map(|b| (b - b'0', false))
                .collect()
        })
        .collect();

    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        let mut max = -1i8;
        for col in 0..cols {
            let tree = &mut map[row][col];
            if tree.0 as i8 > max {
                tree.1 = true;
                max = tree.0 as i8;
            }
        }

        max = -1i8;
        for col in (0..cols).rev() {
            let tree = &mut map[row][col];
            if tree.0 as i8 > max {
                tree.1 = true;
                max = tree.0 as i8;
            }
        }
    }

    for col in 0..cols {
        let mut max = -1i8;
        for row in 0..rows {
            let tree = &mut map[row][col];
            if tree.0 as i8 > max {
                tree.1 = true;
                max = tree.0 as i8;
            }
        }

        max = -1i8;
        for row in (0..rows).rev() {
            let tree = &mut map[row][col];
            if tree.0 as i8 > max {
                tree.1 = true;
                max = tree.0 as i8;
            }
        }
    }

    map
        .iter()
        .map(|row| row
             .iter()
             .filter(|tree| tree.1)
             .count())
        .sum()
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
    let input = "30373
25512
65332
33549
35390";

    assert_eq!(part1(input), 21);
}
