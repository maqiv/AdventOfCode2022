use shared::read_input;

fn main() {
    let greatest_number = quest0();
    println!("Quest 0: The greatest number is {greatest_number}");

    let top_three = quest1();
    println!("Quest 1: The top three elves carry {top_three} calories");
}

fn quest0() -> i32 {
    let mut greatest = 0;
    let mut walker = 0;

    for line in read_input("./src/input") {
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

    greatest
}

fn quest1() -> i32 {
    let mut walker = 0;
    let mut top_three = [0; 3];

    for line in read_input("./src/input") {
        if !line.trim().is_empty() {
            let number: i32 = line.trim().parse().expect("This should be a number");
            walker += number;
        } else {
            if walker > top_three[0] {
                // Maybe refactor this to a more elegant solution :D
                top_three[2] = top_three[1];
                top_three[1] = top_three[0];
                top_three[0] = walker;
            }
            walker = 0;
        }
    }

    top_three.iter().sum()
}
