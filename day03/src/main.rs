use shared::read_input;
use std::slice::Chunks;

fn main() {
    let q0_result = quest0();
    println!("All duplicate items in the rucksacks have an overall sum of {q0_result}");

    let q1_result = quest1();
    println!("All all elve groups share common items with an overall sum of {q1_result}");
}

fn quest0() -> i32 {
    let mut rucksack_comps: Vec<Rucksack> = Vec::new();

    for rucksack in get_rucksacks() {
        let split_result = split_content(rucksack);
        rucksack_comps.push(split_result);
    }

    let mut rucksack_dup_items: Vec<i32> = Vec::new();
    for compartments in rucksack_comps {
        let dup_items_values = analyze_rucksack(compartments);
        rucksack_dup_items.push(dup_items_values)
    }

    rucksack_dup_items.iter().sum()
}

fn quest1() -> i32 {
    let rucksacks = get_rucksacks();
    let rucksack_groups = rucksacks.chunks(3);

    let common_items = extract_common_items(rucksack_groups);

    common_items.iter().map(|&c| evaluate_priority(c)).sum()
}

fn extract_common_items(rucksack_groups: Chunks<String>) -> Vec<char> {
    let mut common_items: Vec<char> = Vec::new();

    for group in rucksack_groups {
        let item = group[0]
            .chars()
            .find(|&s| group[1].contains(s) && group[2].contains(s));

        common_items.push(item.unwrap());
    }

    common_items
}

fn split_content(rucksack: String) -> Rucksack {
    let comp_len = rucksack.len() / 2;
    let (first_comp, second_comp) = rucksack.split_at(comp_len);

    Rucksack {
        first_comp: String::from(first_comp),
        second_comp: String::from(second_comp),
    }
}

fn analyze_rucksack(rucksack: Rucksack) -> i32 {
    println!("{}, {}", rucksack.first_comp, rucksack.second_comp);
    let mut priority: i32 = 0;

    for char in rucksack.first_comp.chars() {
        if rucksack.second_comp.contains(char) {
            priority = evaluate_priority(char);
            break;
        }
    }

    priority
}

fn evaluate_priority(char: char) -> i32 {
    let offset = if char.is_lowercase() { 96 } else { 38 };
    let char_value = char as i32 - offset;
    println!("Duplicate item is {}, {:?}", char, char_value);
    char_value
}

fn get_rucksacks() -> Vec<String> {
    read_input("./src/input")
}

struct Rucksack {
    first_comp: String,
    second_comp: String,
}
