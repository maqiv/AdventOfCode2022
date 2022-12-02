use std::fs;

pub fn read_input(file: &str) -> Vec<String> {
    let lines = fs::read_to_string(file).expect("Should have been able to read the file");

    lines
        .split_terminator("\n")
        .map(|s| s.to_string())
        .collect()
}
