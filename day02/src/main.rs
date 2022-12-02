use shared::read_input;
use std::str::FromStr;

fn main() {
    let q0_result = quest0();
    println!("My total score according to my strategy guide would be {q0_result}");
}

fn quest0() -> i32 {
    let mut evaluated_rounds: Vec<i32> = Vec::new();

    for round in get_rounds() {
        let eval_result = evaluate_round(round);
        evaluated_rounds.push(eval_result);
    }

    evaluated_rounds.iter().sum()
}

fn get_rounds() -> Vec<Round> {
    let mut rounds: Vec<Round> = Vec::new();

    for line in read_input("./src/input") {
        let round = parse_line(line.trim());
        rounds.push(round);
    }

    rounds
}

fn evaluate_round(round: Round) -> i32 {
    let my_result = match round.me {
        Shape::Rock => rock_vs(&round.opponent),
        Shape::Paper => paper_vs(&round.opponent),
        Shape::Scissor => scissor_vs(&round.opponent),
    };

    shape_value(round.me) + round_value(my_result)
}

fn parse_line(line: &str) -> Round {
    let split_line: Vec<&str> = line.split(' ').collect();

    let op = Shape::from_str(split_line[0]);
    let mine = Shape::from_str(split_line[1]);

    Round {
        opponent: op.unwrap(),
        me: mine.unwrap(),
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Shape, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissor),
            _ => Err(()),
        }
    }
}

fn shape_value(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

fn round_value(round_result: RoundResult) -> i32 {
    match round_result {
        RoundResult::Loose => 0,
        RoundResult::Draw => 3,
        RoundResult::Win => 6,
    }
}

fn rock_vs(shape: &Shape) -> RoundResult {
    match shape {
        Shape::Rock => RoundResult::Draw,
        Shape::Paper => RoundResult::Loose,
        Shape::Scissor => RoundResult::Win,
    }
}

fn paper_vs(shape: &Shape) -> RoundResult {
    match shape {
        Shape::Rock => RoundResult::Win,
        Shape::Paper => RoundResult::Draw,
        Shape::Scissor => RoundResult::Loose,
    }
}

fn scissor_vs(shape: &Shape) -> RoundResult {
    match shape {
        Shape::Rock => RoundResult::Loose,
        Shape::Paper => RoundResult::Win,
        Shape::Scissor => RoundResult::Draw,
    }
}

#[derive(Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

struct Round {
    opponent: Shape,
    me: Shape,
}

#[derive(PartialEq)]
enum RoundResult {
    Win,
    Loose,
    Draw,
}
