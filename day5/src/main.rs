use std::env;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let raw_input = std::fs::read_to_string(file_path).unwrap();

    let input: Vec<_> = raw_input
        .split("\n\n")
        .collect();

    let mut drawing: Vec<_> = input[0]
        .split("\n")
        .collect();

    let procedure = input[1];

    let stacks = drawing
        .pop()
        .unwrap();

    let num_stacks = stacks
        .split_ascii_whitespace()
        .count();

    let mut stacks_part1: Vec<Vec<char>> = vec![vec![]; num_stacks];
    let mut stacks_part2: Vec<Vec<char>> = vec![vec![]; num_stacks];
    while let Some(layer) = drawing.pop() {
        let mut idx   = 0;
        let mut begin = 1;
        while let Some(ch) = layer.get(begin..(begin + 1)) {
            match ch.chars().next().unwrap() {
                ' ' => (),
                c   => {
                    stacks_part1[idx].push(c);
                    stacks_part2[idx].push(c);
                },
            }
            idx   += 1;
            begin += 4;
        }
    }

    for instruction in procedure.lines() {
        let inst: Vec<_> = instruction.split_ascii_whitespace().collect();
        let (num, from, to) = (
            inst[1].parse::<usize>().unwrap(),
            inst[3].parse::<usize>().unwrap() -1,
            inst[5].parse::<usize>().unwrap() -1);

        for _ in 0..num {
            let tmp = stacks_part1[from].pop().unwrap();
            stacks_part1[to].push(tmp);
        }
    }

    let mut part1 = String::new();
    for stack in &stacks_part1 {
        let top = stack.last().unwrap();
        part1.push(*top);
    }
    println!("Part 1: {}", part1);

    for instruction in procedure.lines() {
        let inst: Vec<_> = instruction.split_ascii_whitespace().collect();
        let (num, from, to) = (
            inst[1].parse::<usize>().unwrap(),
            inst[3].parse::<usize>().unwrap() -1,
            inst[5].parse::<usize>().unwrap() -1);

        let mut tmp: Vec<char> = vec![];
        for _ in 0..num {
            tmp.push(stacks_part2[from].pop().unwrap());
        }
        tmp.reverse();
        for ch in tmp {
            stacks_part2[to].push(ch);
        }
    }

    let mut part2 = String::new();
    for stack in &stacks_part2 {
        let top = stack.last().unwrap();
        part2.push(*top);
    }
    println!("Part 2: {}", part2);
}
