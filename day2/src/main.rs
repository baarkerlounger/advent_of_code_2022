use std::fs;

fn main() {
    let filename = "data/input.txt";
    println!(
        "Total score, part 1 is {}",
        strategy_outcome(filename, Part::One)
    );
    println!(
        "Total score, part 2 is {}",
        strategy_outcome(filename, Part::Two)
    );
}

#[derive(Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Part {
    One,
    Two,
}

fn strategy_outcome(filename: &str, part: Part) -> u32 {
    let file_contents =
        fs::read_to_string(filename).expect("Should have been able to read the file");
    let rounds = file_contents.trim().split("\n");
    let mut score = 0;

    for round in rounds {
        let moves: Vec<&str> = round.split(" ").collect();
        let opponent_move = decoded_move(moves[0]);
        let your_move = match part {
            Part::One => decoded_move(moves[1]),
            Part::Two => your_move(moves[1], &opponent_move),
        };
        score = score + shape_score(&your_move) + result(opponent_move, your_move);
    }
    score
}

fn decoded_move(encoded_move: &str) -> Move {
    match encoded_move {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn shape_score(shape: &Move) -> u32 {
    match shape {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn losing_move(opponent_move: &Move) -> Move {
    match opponent_move {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn winning_move(opponent_move: &Move) -> Move {
    match opponent_move {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn tying_move(opponent_move: &Move) -> Move {
    opponent_move.clone()
}

fn result(opponent_move: Move, your_move: Move) -> u32 {
    match your_move {
        your_move if your_move == winning_move(&opponent_move) => 6,
        your_move if your_move == losing_move(&opponent_move) => 0,
        _ => 3,
    }
}

fn your_move(result: &str, opponent_move: &Move) -> Move {
    match result {
        "Y" => tying_move(opponent_move),
        "X" => losing_move(opponent_move),
        "Z" => winning_move(opponent_move),
        _ => panic!("Invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_outcome_part_1() {
        let result = strategy_outcome("data/demo_input.txt", Part::One);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_strategy_outcome_part_2() {
        let result = strategy_outcome("data/demo_input.txt", Part::Two);
        assert_eq!(result, 12);
    }
}
