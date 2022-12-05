use std::env;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No input given");

    let _input = std::fs::read_to_string(file_path).unwrap();
}
