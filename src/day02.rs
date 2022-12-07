use std::fs;

enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

enum RpsOutcome {
    Loss,
    Draw,
    Win,
}

fn rps_run(move_self: &RpsMove, move_opponent: &RpsMove) -> RpsOutcome {
    match (move_self, move_opponent) {
        (RpsMove::Rock, RpsMove::Rock) => RpsOutcome::Draw,
        (RpsMove::Rock, RpsMove::Paper) => RpsOutcome::Loss,
        (RpsMove::Rock, RpsMove::Scissors) => RpsOutcome::Win,
        (RpsMove::Paper, RpsMove::Rock) => RpsOutcome::Win,
        (RpsMove::Paper, RpsMove::Paper) => RpsOutcome::Draw,
        (RpsMove::Paper, RpsMove::Scissors) => RpsOutcome::Loss,
        (RpsMove::Scissors, RpsMove::Rock) => RpsOutcome::Loss,
        (RpsMove::Scissors, RpsMove::Paper) => RpsOutcome::Win,
        (RpsMove::Scissors, RpsMove::Scissors) => RpsOutcome::Draw,
    }
}

fn rps_infer_move(move_opponent: &RpsMove, outcome: &RpsOutcome) -> RpsMove {
    match (move_opponent, outcome) {
        (RpsMove::Rock, RpsOutcome::Draw) => RpsMove::Rock,
        (RpsMove::Rock, RpsOutcome::Loss) => RpsMove::Scissors,
        (RpsMove::Rock, RpsOutcome::Win) => RpsMove::Paper,
        (RpsMove::Paper, RpsOutcome::Draw) => RpsMove::Paper,
        (RpsMove::Paper, RpsOutcome::Loss) => RpsMove::Rock,
        (RpsMove::Paper, RpsOutcome::Win) => RpsMove::Scissors,
        (RpsMove::Scissors, RpsOutcome::Draw) => RpsMove::Scissors,
        (RpsMove::Scissors, RpsOutcome::Loss) => RpsMove::Paper,
        (RpsMove::Scissors, RpsOutcome::Win) => RpsMove::Rock,
    }
}

pub fn soln01() {
    let input = fs::read_to_string("inputs/day02/input.txt").expect("Unable to read file");
    let mut score = 0;
    for line in input.lines() {
        let (move_opponent, move_self) = line.split_once(' ').expect("Invalid input");
        let move_opponent = match move_opponent {
            "A" => RpsMove::Rock,
            "B" => RpsMove::Paper,
            "C" => RpsMove::Scissors,
            _ => panic!("Invalid input"),
        };
        let move_self = match move_self {
            "X" => RpsMove::Rock,
            "Y" => RpsMove::Paper,
            "Z" => RpsMove::Scissors,
            _ => panic!("Invalid input"),
        };
        score += match move_self {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
        };
        score += match rps_run(&move_self, &move_opponent) {
            RpsOutcome::Loss => 0,
            RpsOutcome::Draw => 3,
            RpsOutcome::Win => 6,
        };
    }
    println!("Answer: {}", score);
}

pub fn soln02() {
    let input = fs::read_to_string("inputs/day02/input.txt").expect("Unable to read file");
    let mut score = 0;
    for line in input.lines() {
        let (move_opponent, outcome) = line.split_once(' ').expect("Invalid input");
        let move_opponent = match move_opponent {
            "A" => RpsMove::Rock,
            "B" => RpsMove::Paper,
            "C" => RpsMove::Scissors,
            _ => panic!("Invalid input"),
        };
        let outcome = match outcome {
            "X" => RpsOutcome::Loss,
            "Y" => RpsOutcome::Draw,
            "Z" => RpsOutcome::Win,
            _ => panic!("Invalid input"),
        };
        let move_self = rps_infer_move(&move_opponent, &outcome);
        score += match move_self {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
        };
        score += match outcome {
            RpsOutcome::Loss => 0,
            RpsOutcome::Draw => 3,
            RpsOutcome::Win => 6,
        };
    }
    println!("Answer: {}", score);
}
