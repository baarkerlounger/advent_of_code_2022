use regex::Regex;
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

enum Part {
    One,
    Two,
}

fn result(input: &str, part: Part) -> String {
    let lines = input.split("\n");
    let separator = Regex::new(r"([a-zA-Z\s])").expect("Invalid regex");
    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut line_no = 0;
    let mut result = String::new();

    for line in lines {
        if line.starts_with("m") {
            apply_move(&mut crates, line, &separator, &part)
        } else if line.clone().trim().starts_with("[") {
            transpose_crate_vector(&mut crates, line, &mut line_no)
        }
        line_no = 0;
    }

    for stack in crates {
        result.push(stack[0]);
    }
    result
}

fn transpose_crate_vector(crates: &mut Vec<Vec<char>>, line: &str, line_no: &mut usize) {
    let char_vec: Vec<char> = line.chars().collect();
    for chunk in char_vec.chunks(4) {
        if crates.len() > *line_no {
            crates[*line_no].push(chunk[1]);
        } else {
            if chunk[1].is_whitespace() {
                crates.push(Vec::new());
            } else {
                crates.push(vec![chunk[1]]);
            }
        }
        *line_no = *line_no + 1;
    }
}

fn apply_move(crates: &mut Vec<Vec<char>>, line: &str, separator: &Regex, part: &Part) {
    let instructions: Vec<&str> = separator.split(line).filter(|&v| !v.is_empty()).collect();
    let move_count = instructions[0].parse::<usize>().unwrap();
    let from_crate_stack_idx = instructions[1].parse::<usize>().unwrap() - 1;
    let to_crate_stack_idx = instructions[2].parse::<usize>().unwrap() - 1;
    let mut from_crate_stack = crates[from_crate_stack_idx].to_vec();
    let mut to_crate_stack = crates[to_crate_stack_idx].to_vec();
    from_crate_stack.retain(|&v| !v.is_whitespace());
    let moving_crates = from_crate_stack.drain(0..move_count);
    match part {
        Part::One => {
            for c in moving_crates {
                to_crate_stack.insert(0, c);
            }
        }
        Part::Two => {
            for c in moving_crates.rev() {
                to_crate_stack.insert(0, c);
            }
        }
    }
    crates[from_crate_stack_idx] = from_crate_stack;
    crates[to_crate_stack_idx] = to_crate_stack;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::One), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let file_contents = fs::read_to_string("data/demo_input.txt").expect("valid file");
        assert_eq!(result(&file_contents, Part::Two), "MCD");
    }
}
