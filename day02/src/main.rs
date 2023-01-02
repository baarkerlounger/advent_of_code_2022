use std::env;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("data/input.txt").expect("Valid file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let part: u32 = args[1].parse().unwrap();

        match part {
            1 => println!("Result for part 1 is {}", result(&file_contents, Part::One)),
            2 => println!("Result for part 2 is {}", result(&file_contents, Part::Two)),
            _ => println!("Specify 1 or 2"),
        }
    } else {
        println!("Result for part 1 is {}", result(&file_contents, Part::One));
        println!("Result for part 2 is {}", result(&file_contents, Part::Two));
    }
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

fn result(input: &str, part: Part) -> u32 {
    let rounds = input.trim().split("\n");
    let mut score = 0;

    for round in rounds {
        let moves: Vec<&str> = round.split(" ").collect();
        let opponent_move = decoded_move(moves[0]);
        let your_move = match part {
            Part::One => decoded_move(moves[1]),
            Part::Two => your_move(moves[1], &opponent_move),
        };
        score = score + shape_score(&your_move) + outcome(opponent_move, your_move);
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

fn outcome(opponent_move: Move, your_move: Move) -> u32 {
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
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), 15);
    }

    #[test]
    fn test_strategy_outcome_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), 12);
    }
}
