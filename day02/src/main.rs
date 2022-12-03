use shared::read_input;
use std::str::FromStr;

// ToDo: Refactor this code to have function pointers instead of
// `get_rounds`/`get_round_predictions` respectively `evaluate_round`
// and `evaluate_round_predictions`.

fn main() {
    let q0_result = quest0();
    println!("My total score according to my strategy guide would be {q0_result}");
    
    let q1_result = quest1();
    println!("My total score with the second value being the expected result would be {q1_result}");
}

fn quest0() -> i32 {
    let mut evaluated_rounds: Vec<i32> = Vec::new();

    for round in get_rounds() {
        let eval_result = evaluate_round(round);
        evaluated_rounds.push(eval_result);
    }

    evaluated_rounds.iter().sum()
}

fn quest1() -> i32 {
    let mut evaluated_rounds: Vec<i32> = Vec::new();

    for round_prediction in get_round_predictions() {
        let eval_result = evaluate_round_predictions(round_prediction);
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

fn get_round_predictions() -> Vec<RoundPrediction> {
    let mut rounds: Vec<RoundPrediction> = Vec::new();

    for line in read_input("./src/input") {
        let round = parse_line_prediction(line.trim());
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

fn evaluate_round_predictions(round_prediction: RoundPrediction) -> i32 {
    let my_result = match round_prediction.exp_result {
        RoundResult::Loose => loose_against(&round_prediction.opponent),
        RoundResult::Draw => draw_against(&round_prediction.opponent),
        RoundResult::Win => win_against(&round_prediction.opponent),
    };

    shape_value(my_result) + round_value(round_prediction.exp_result)
}

fn loose_against(opponent: &Shape) -> Shape {
    match opponent {
        Shape::Rock => Shape::Scissor,
        Shape::Paper => Shape::Rock,
        Shape::Scissor => Shape::Paper,
    }
}

fn draw_against(opponent: &Shape) -> Shape {
    *opponent
}

fn win_against(opponent: &Shape) -> Shape {
    match opponent {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissor,
        Shape::Scissor => Shape::Rock,
    }
}

fn parse_line(line: &str) -> Round {
    let split_line: Vec<&str> = line.split(' ').collect();

    let first = Shape::from_str(split_line[0]);
    let second = Shape::from_str(split_line[1]);

    Round {
        opponent: first.unwrap(),
        me: second.unwrap(),
    }
}

fn parse_line_prediction(line: &str) -> RoundPrediction {
    let split_line: Vec<&str> = line.split(' ').collect();

    let first = Shape::from_str(split_line[0]);
    let second = RoundResult::from_str(split_line[1]);

    RoundPrediction {
        opponent: first.unwrap(),
        exp_result: second.unwrap(),
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

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<RoundResult, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
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

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

struct Round {
    opponent: Shape,
    me: Shape,
}

struct RoundPrediction {
    opponent: Shape,
    exp_result: RoundResult,
}

#[derive(PartialEq)]
enum RoundResult {
    Win,
    Loose,
    Draw,
}
