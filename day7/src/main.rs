use std::env;
use std::collections::HashMap;
use std::path::PathBuf;

enum Entry {
    Directory(PathBuf),
    File(usize),
}

type Tree = HashMap<PathBuf, Vec<Entry>>;

const SIZE_LIMIT: usize = 100_000;

fn calculate_size(tree: &Tree, path: &PathBuf) -> usize {
    tree[path]
        .iter()
        .map(|e| match e {
            Entry::Directory(path) => calculate_size(tree, path),
            Entry::File(size) => *size,
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let mut tree: Tree = HashMap::new();
    let mut curr = PathBuf::new();

    for line in input.lines() {
        let split: Vec<_> = line.split_ascii_whitespace().collect();

        match split[0] {
            "$" => match split[1] {
                "cd" => match split[2] {
                    ".." => {
                        curr.pop();
                    },
                    _ => {
                        curr.push(split[2]);
                        tree.entry(curr.clone()).or_default();
                    },
                },
                "ls" => (),
                _ => unreachable!(),
            },
            "dir" => {
                let dir = curr.join(split[1]);
                tree.entry(dir.clone()).or_default();
                tree
                    .get_mut(&curr)
                    .unwrap()
                    .push(Entry::Directory(dir));
            },
            _ => {
                let size = split[0].parse().unwrap();
                tree
                    .get_mut(&curr)
                    .unwrap()
                    .push(Entry::File(size));
            }
        }
    }

    tree
        .iter()
        .map(|e| calculate_size(&tree, e.0))
        .filter(|size| *size <= SIZE_LIMIT)
        .sum()
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let input = std::fs::read_to_string(file_path).unwrap();

    println!("Part 1: {}", part1(input.as_str()));
}

#[test]
fn example() {
    let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    assert_eq!(part1(input), 95437);
}
