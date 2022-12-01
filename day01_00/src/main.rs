use std::fs;

fn main() {
    let mut greatest = 0;
    let mut walker = 0;

    let lines = fs::read_to_string("./src/input").expect("Should have been able to read the file");

    for line in lines.split_terminator("\n") {
        if !line.trim().is_empty() {
            let number: i32 = line.trim().parse().expect("This should be a number");
            walker += number;
        } else {
            if walker > greatest {
                greatest = walker;
            }
            walker = 0;
        }
    }

    println!("{greatest}");
}
