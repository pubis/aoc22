use std::env;

fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let input = std::fs::read_to_string(file_path).unwrap();

    let assignments = input
        .lines()
        .map(|pair| {
            let seqs: Vec<_> = pair.split(",").collect();
            let seq1: Vec<_> = seqs[0].split("-").map(|v| v.parse::<u32>().unwrap()).collect();
            let seq2: Vec<_> = seqs[1].split("-").map(|v| v.parse::<u32>().unwrap()).collect();
            ((seq1[0], seq1[1]), (seq2[0], seq2[1]))
        });

    let part1 = assignments
        .filter(|(a, b)| {
            contains(*a, *b) || contains(*b, *a)
        })
        .count();

    println!("Part 1: {}", part1);
}
