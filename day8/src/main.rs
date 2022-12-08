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

fn scenic_score(map: &Vec<Vec<u8>>, row: usize, col: usize, tree: u8) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    let mut left: usize = 0;
    for c in (0..col).rev() {
        left += 1;
        if map[row][c] >= tree {
            break;
        }
    }

    let mut right: usize = 0;
    for c in (col + 1)..cols {
        right += 1;
        if map[row][c] >= tree {
            break;
        }
    }

    let mut up: usize = 0;
    for r in (0..row).rev() {
        up += 1;
        if map[r][col] >= tree {
            break;
        }
    }

    let mut down: usize = 0;
    for r in (row + 1)..rows {
        down += 1;
        if map[r][col] >= tree {
            break;
        }
    }

    left * right * up * down
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line
                .as_bytes()
                .iter()
                .map(|b| b - b'0')
                .collect()
        })
        .collect();

    map
        .iter()
        .enumerate()
        .map(|(i, row)| row
             .iter()
             .enumerate()
             .map(|(j, tree)| scenic_score(&map, i, j, *tree))
             .max()
             .unwrap())
        .max()
        .unwrap()
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let input = std::fs::read_to_string(file_path).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[test]
fn example() {
    let input = "30373
25512
65332
33549
35390";

    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 8);
}
