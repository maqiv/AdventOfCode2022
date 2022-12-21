use regex::Regex;
use shared::read_input;

fn main() {
    quest0();

    quest1();
}

fn quest1() {
    let lines = read_input("./src/input");
    let mut warehouse = build_warehouse(&lines);
    let commands = parse_commands(lines);
    for cmd in commands {
        let pos = warehouse[cmd.from - 1].len() - cmd.steps;
        let mut crt_on_the_move = warehouse[cmd.from - 1].split_off(pos);
        warehouse[cmd.to - 1].append(&mut crt_on_the_move);
    }

    println!("");
    print!("The crates on top of each stack are: ");
    for mut stack in warehouse {
        let crt = stack.pop().unwrap();
        print!("{crt}");
    }
    println!("\n");
}

fn quest0() {
    let lines = read_input("./src/input");
    let mut warehouse = build_warehouse(&lines);
    let commands = parse_commands(lines);
    for cmd in commands {
        for _ in 0..cmd.steps {
            let crt_on_the_move = warehouse[cmd.from - 1].pop().unwrap();
            warehouse[cmd.to - 1].push(crt_on_the_move);
        }
    }
    println!("");
    print!("The crates on top of each stack are: ");
    for mut stack in warehouse {
        let crt = stack.pop().unwrap();
        print!("{crt}");
    }
    println!("\n");
}

fn parse_commands(lines: Vec<String>) -> Vec<MoveCommand> {
    let mut commands: Vec<MoveCommand> = Vec::new();

    for line in lines {
        let (success, parsed_command) = parse_command(line.clone());

        if !success {
            continue;
        }

        commands.push(parsed_command.unwrap());
    }

    commands
}

fn parse_command(line: String) -> (bool, Option<MoveCommand>) {
    let re = Regex::new(r"move\s(?P<steps>\d+)\sfrom\s(?P<from>\d)\sto\s(?P<to>\d)").unwrap();

    let caps = re.captures(&line);
    if caps.is_none() {
        return (false, None);
    }

    let caps_unwrapped = caps.unwrap();
    (
        true,
        Some(MoveCommand {
            steps: parse_named_capture(&caps_unwrapped, "steps"),
            from: parse_named_capture(&caps_unwrapped, "from"),
            to: parse_named_capture(&caps_unwrapped, "to"),
        }),
    )
}

fn parse_named_capture(caps: &regex::Captures, name: &str) -> usize {
    caps.name(name)
        .unwrap()
        .as_str()
        .parse()
        .expect("Not a number")
}

fn build_warehouse(lines: &Vec<String>) -> [Vec<String>; 9] {
    let mut warehouse: [Vec<String>; 9] = Default::default();

    for line in lines {
        if is_warehouse_stack_index_line(line.clone()) || line.trim().is_empty() {
            break;
        }

        let crate_str_chunks = create_crate_chunks(&line);
        dispatch_crates(&mut warehouse, crate_str_chunks);
    }

    // We have to reverse each stack in the warehouse because it
    // is parsed top down onto the stacks in reverse order.
    for (i, _) in warehouse.clone().iter().enumerate() {
        warehouse[i].reverse();
    }

    warehouse
}

fn is_warehouse_stack_index_line(line: String) -> bool {
    let re = Regex::new(r"\s\d\s").unwrap();
    re.is_match(&line)
}

fn create_crate_chunks(line: &String) -> Vec<String> {
    let crate_str_chunks: Vec<String> = line
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect())
        .collect();

    crate_str_chunks
}

fn dispatch_crates(warehouse: &mut [Vec<String>; 9], crate_chunks: Vec<String>) {
    let mut stack_pos = 0;
    let re = Regex::new(r"\[(\w)\]\s?").unwrap();

    for chunk in crate_chunks {
        println!("my chunk {chunk}");
        let caps = re.captures(&chunk);

        if caps.is_some() {
            let ch = caps.unwrap().get(1).unwrap().as_str();
            warehouse[stack_pos].push(String::from(ch));
        }

        stack_pos += 1;
    }
}

struct MoveCommand {
    from: usize,
    to: usize,
    steps: usize,
}
